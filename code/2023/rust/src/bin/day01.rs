use anyhow::Result;

fn main() -> Result<()> {
    let part = std::env::args().nth(1).unwrap_or("part1".to_string());
    match part.as_str() {
        "part1" => part1(),
        "part2" => part2(),
        _ => panic!("invalid part"),
    }
}

pub fn part1() -> Result<()> {
    let input = std::fs::read_to_string("data/01.txt")?;

    let values: Vec<_> = input
        .lines()
        .into_iter()
        .map(|line| {
            let digits: Vec<_> = line
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .collect();

    println!("Sum: {}", values.iter().sum::<u32>());

    Ok(())
}

fn part2() -> Result<()> {
    let input = std::fs::read_to_string("data/01.txt")?;
    let values: Vec<_> = input
        .lines()
        .into_iter()
        .map(|line| {
            let digits = real_digits(line);
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .collect();

    println!("Sum: {}", values.iter().sum::<u32>());
    Ok(())
}

pub fn real_digits(line: &str) -> Vec<u32> {
    let mut digits = Vec::new();

    let spelled = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let numbers = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];

    for (idx, s) in spelled.iter().enumerate() {
        for (pos, _) in line.match_indices(s) {
            digits.push(((idx + 1) as u32, pos));
        }
    }

    for (idx, n) in numbers.iter().enumerate() {
        for (pos, _) in line.match_indices(n) {
            digits.push(((idx + 1) as u32, pos));
        }
    }

    // Sort by the position.
    digits.sort_by(|a, b| a.1.cmp(&b.1));

    // Return the first item in the tuple.
    digits.into_iter().map(|(d, _)| d).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_digits() {
        assert_eq!(real_digits("oneoneone"), vec![1, 1, 1]);
        assert_eq!(real_digits("two1nine1"), vec![2, 1, 9, 1]);
        assert_eq!(real_digits("eightwothree"), vec![8, 2, 3]);
        assert_eq!(real_digits("abcone2threexyz"), vec![1, 2, 3]);
        assert_eq!(real_digits("xtwone3four"), vec![2, 1, 3, 4]);
        assert_eq!(real_digits("4nineeightseven2"), vec![4, 9, 8, 7, 2]);
        assert_eq!(real_digits("zoneight234"), vec![1, 8, 2, 3, 4]);
        assert_eq!(real_digits("7pqrstsixteen"), vec![7, 6]);
        assert_eq!(real_digits("fourthree73fiveseven"), vec![4, 3, 7, 3, 5, 7]);
        assert_eq!(
            real_digits("fourthree73fiveseven7"),
            vec![4, 3, 7, 3, 5, 7, 7]
        );
        assert_eq!(real_digits("9five9six8threet"), vec![9, 5, 9, 6, 8, 3]);
    }
}
