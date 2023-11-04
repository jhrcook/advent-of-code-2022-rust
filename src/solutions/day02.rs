use crate::data::load;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Day2Error {
    #[error("input data parsing")]
    Parsing(String),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum GameResult {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

struct GameRound {
    opponent: Shape,
    you: Shape,
}

impl GameRound {
    fn result(self) -> GameResult {
        match (self.you as i8 - self.opponent as i8) % 3i8 {
            0 => GameResult::Draw,
            1 | -2 => GameResult::Win,
            2 | -1 => GameResult::Lose,
            x => panic!(
                "Unexpected game calculation result: {:?} {:?} = {}",
                self.you, self.opponent, x
            ),
        }
    }
}

// fn parse_puzzle_input(input_data: &str) -> () {}

pub fn puzzle_1(input_data: &str) -> Result<u32, Day2Error> {
    let mut tally: u32 = 0;
    for line in input_data.trim().lines() {
        let plays: Vec<&str> = line.trim().split(' ').collect();
        assert_eq!(plays.len(), 2, "plays: {:?}", plays);

        let opponent = match plays[0] {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            v => Err(Day2Error::Parsing(v.to_string())),
        }?;
        let you = match plays[1] {
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            v => Err(Day2Error::Parsing(v.to_string())),
        }?;
        let game = GameRound { opponent, you };
        tally += game.you as u32 + game.result() as u32;
    }
    Ok(tally)
}

pub fn puzzle_2(input_data: &str) -> Result<u32, Day2Error> {
    let mut tally: u32 = 0;
    for line in input_data.trim().lines() {
        let plays: Vec<&str> = line.trim().split(' ').collect();
        assert_eq!(plays.len(), 2, "plays: {:?}", plays);

        let opponent = match plays[0] {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            v => Err(Day2Error::Parsing(v.to_string())),
        }?;
        let result = match plays[1] {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            v => Err(Day2Error::Parsing(v.to_string())),
        }?;
        let you: u32 = match result {
            GameResult::Lose => {
                if opponent == Shape::Rock {
                    3
                } else {
                    opponent as u32 - 1
                }
            }
            GameResult::Draw => opponent as u32,
            GameResult::Win => {
                if opponent == Shape::Scissors {
                    1
                } else {
                    opponent as u32 + 1
                }
            }
        };
        tally += you + result as u32;
    }
    Ok(tally)
}

pub fn main(data_dir: &str) {
    println!("Day 2: Rock Paper Scissors");
    let data = load(data_dir, 2, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("Error on Puzzle 1: {}", e),
    }
    assert_eq!(answer_1, Ok(11873));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("Error on Puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(12014))
}

#[cfg(test)]
mod tests {
    use crate::solutions::day02::{puzzle_1, puzzle_2};

    const EXAMPLE_1: &str = "
    A Y
    B X
    C Z
    ";

    #[test]
    fn example_1_puzzle_1() {
        assert_eq!(puzzle_1(EXAMPLE_1), Ok(15))
    }
    #[test]
    fn example_1_puzzle_2() {
        assert_eq!(puzzle_2(EXAMPLE_1), Ok(12))
    }
}
