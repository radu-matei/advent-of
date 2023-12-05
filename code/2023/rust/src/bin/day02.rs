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
pub struct Turn {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Turn {
    pub fn from(input: Vec<(u32, &str)>) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for (count, color) in input {
            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => panic!("invalid color"),
            }
        }

        Self { red, green, blue }
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub turns: Vec<Turn>,
}

impl Game {
    pub fn from_line(line: &str) -> Self {
        let parts: Vec<_> = line.split(": ").collect();
        let id: u32 = parts[0].split("Game ").nth(1).unwrap().parse().unwrap();

        let turns: Vec<_> = parts[1]
            .split("; ")
            .map(|turn| {
                let turn = turn
                    .split(", ")
                    .into_iter()
                    .map(|color| {
                        let parts: Vec<_> = color.split(" ").collect();
                        let count: u32 = parts[0].parse().unwrap();
                        let color = parts[1];
                        (count, color)
                    })
                    .collect::<Vec<_>>();

                Turn::from(turn)
            })
            .collect();

        Self { id, turns }
    }

    pub fn impossible(&self, possible: &Turn) -> bool {
        self.turns.iter().any(|turn| {
            turn.red > possible.red || turn.green > possible.green || turn.blue > possible.blue
        })
    }

    pub fn power(&self) -> u32 {
        let red = self.turns.iter().map(|turn| turn.red).max().unwrap();
        let green = self.turns.iter().map(|turn| turn.green).max().unwrap();
        let blue = self.turns.iter().map(|turn| turn.blue).max().unwrap();

        red * green * blue
    }
}

pub fn part1() -> Result<()> {
    let input = std::fs::read_to_string("data/02.txt")?;

    let possible = Turn {
        red: 12,
        green: 13,
        blue: 14,
    };

    let res: u32 = input
        .lines()
        .into_iter()
        .map(|line| Game::from_line(&line))
        .filter(|game| !game.impossible(&possible))
        .map(|game| game.id)
        .sum();

    println!("Result: {}", res);

    Ok(())
}

pub fn part2() -> Result<()> {
    let input = std::fs::read_to_string("data/02.txt")?;
    let res: u32 = input
        .lines()
        .into_iter()
        .map(|line| Game::from_line(&line))
        .map(|game| game.power())
        .sum();

    println!("Result: {}", res);

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_game_parse() -> Result<()> {
        let input = "Game 1: 7 green, 14 red, 5 blue; 8 red, 4 green; 6 green, 18 red, 9 blue";

        let g1 = Game::from_line(input);

        assert_eq!(g1.id, 1);
        assert_eq!(g1.turns.len(), 3);

        assert_eq!(g1.turns[0].red, 14);
        assert_eq!(g1.turns[0].green, 7);
        assert_eq!(g1.turns[0].blue, 5);

        assert_eq!(g1.turns[1].red, 8);
        assert_eq!(g1.turns[1].green, 4);
        assert_eq!(g1.turns[1].blue, 0);

        assert_eq!(g1.turns[2].red, 18);
        assert_eq!(g1.turns[2].green, 6);
        assert_eq!(g1.turns[2].blue, 9);

        Ok(())
    }

    #[test]
    fn test_game_is_possible() -> Result<()> {
        let input =
            "Game 65: 13 red, 2 green, 3 blue; 1 red, 2 blue, 1 green; 1 blue; 2 green, 1 red";

        let g1 = Game::from_line(input);

        let possible = Turn {
            red: 12,
            green: 13,
            blue: 14,
        };

        assert!(g1.impossible(&possible));

        Ok(())
    }

    #[test]
    fn test_game_power() -> Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let g1 = Game::from_line(input);

        assert_eq!(g1.power(), 48);

        Ok(())
    }
}
