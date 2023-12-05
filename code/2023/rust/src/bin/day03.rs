use anyhow::Result;

fn main() -> Result<()> {
    let part = std::env::args().nth(1).unwrap_or("part1".to_string());
    match part.as_str() {
        "part1" => part1(),
        "part2" => part2(),
        _ => panic!("invalid part"),
    }
}

#[derive(Debug)]
pub struct Schematic {
    pub raw: Vec<Vec<char>>,
}

impl Schematic {
    pub fn from_input(input: &str) -> Self {
        let mut raw = Vec::new();

        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            raw.push(row);
        }

        Self { raw }
    }

    pub fn sum(&self) -> u64 {
        let mut sum = 0;

        for row in 0..self.raw.len() {
            for col in 0..self.raw[row].len() {
                if let Some(number) = self.raw[row][col].to_digit(10) {
                    if col != 0 && self.raw[row][col - 1].is_ascii_digit() {
                        continue;
                    }

                    let mut value: u64 = number as u64;
                    let mut is_valid = self.is_valid_part(row as i32, col as i32);

                    let mut next_digit = col + 1.clamp(0, self.raw[row].len());

                    while let Some(new_unit) = self.raw[row][next_digit].to_digit(10) {
                        value *= 10;
                        value += new_unit as u64;
                        if self.is_valid_part(row as i32, next_digit as i32) {
                            is_valid = true
                        }

                        next_digit += 1;
                        if next_digit == self.raw[row].len() {
                            break;
                        }
                    }

                    if is_valid {
                        sum += value;
                    }
                }
            }
        }
        sum
    }

    pub fn gears(&self) -> u32 {
        let mut res = 0;

        for row in 0..self.raw.len() {
            for col in 0..self.raw[row].len() {
                if '*' == self.raw[row][col] {
                    res += self.compute(row, col);
                }
            }
        }

        res
    }

    fn compute(&self, row: usize, col: usize) -> u32 {
        let start_r = if row == 0 { 0 } else { row - 1 };
        let end_r = if row == self.raw.len() {
            self.raw.len()
        } else {
            row + 1
        };

        let start_c = if col == 0 { 0 } else { col - 1 };
        let end_c = if col == self.raw[row].len() {
            self.raw[row].len()
        } else {
            col + 1
        };

        let mut numbers: Vec<u32> = Vec::new();
        for n_row in start_r..end_r + 1 {
            for n_col in start_c..end_c + 1 {
                if 0 != self.get_number(n_row, n_col) {
                    if n_col == start_c || 0 == self.get_number(n_row, n_col - 1) {
                        numbers.push(self.get_number(n_row, n_col));
                    }
                }
            }
        }
        let mut mult = 0;
        if numbers.len() == 2 {
            mult = 1;
            for n in numbers {
                mult *= n;
            }
        }
        return mult;
    }

    fn get_number(&self, row: usize, col: usize) -> u32 {
        if !self.raw[row][col].is_ascii_digit() {
            return 0;
        }

        let mut index = col;
        let mut value = 0;
        loop {
            if self.raw[row][index].is_ascii_digit() {
                if index == 0 {
                    break;
                }
                index -= 1;
            } else {
                index += 1;
                break;
            }
        }

        loop {
            if let Some(unit) = self.raw[row][index].to_digit(10) {
                value *= 10;
                value += unit;
                if index == self.raw[row].len() {
                    break;
                }
                index += 1;
                if index == self.raw[row].len() {
                    break;
                }
            } else {
                break;
            }
        }
        return value;
    }

    fn is_valid_part(&self, row: i32, col: i32) -> bool {
        for r in -1..2 {
            for c in -1..2 {
                let ro: usize = (row + r).clamp(0, self.raw.len() as i32 - 1) as usize;
                let co = (col + c).clamp(0, self.raw[ro].len() as i32 - 1);

                if self.raw[ro][co as usize].to_digit(10).is_none()
                    && self.raw[ro][co as usize] != '.'
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn print(&self) {
        for row in &self.raw {
            for c in row {
                print!("{}", c);
            }
            println!();
        }
        println!();
    }
}

pub fn part1() -> Result<()> {
    let input = std::fs::read_to_string("data/03.txt")?;
    let s = Schematic::from_input(&input);

    println!("Sum: {}", s.sum());

    Ok(())
}

pub fn part2() -> Result<()> {
    let input = std::fs::read_to_string("data/03.txt")?;
    let s = Schematic::from_input(&input);

    println!("Gear Ratios: {:?}", s.gears());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        let s = Schematic::from_input(input);
        assert_eq!(s.sum(), 4361);
    }
}
