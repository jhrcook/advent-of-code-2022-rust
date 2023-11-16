use crate::data::load_raw;
use log;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("Could not parse the following direction: {}.", .0)]
    UnknownDirection(String),
    #[error("Failed parsing line: {}.", .0)]
    FailedParsing(String),
    #[error("Failed parsing integer: {}.", .0)]
    ParseIntError(std::num::ParseIntError),
    #[error("Performing action on rope with no knots.")]
    NoKnots,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = match self {
            Direction::Up(x) => write!(f, "U {}", x),
            Direction::Down(x) => write!(f, "D {}", x),
            Direction::Left(x) => write!(f, "L {}", x),
            Direction::Right(x) => write!(f, "R {}", x),
        };
        write!(f, "")
    }
}

impl Direction {
    fn value(self) -> usize {
        match self {
            Direction::Up(n) => n,
            Direction::Down(n) => n,
            Direction::Left(n) => n,
            Direction::Right(n) => n,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Knot {
    x: usize,
    y: usize,
}

impl fmt::Display for Knot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Knot {
    fn step(&mut self, direction: Direction) {
        (self.x, self.y) = match direction {
            Direction::Up(_) => (self.x, self.y + 1),
            Direction::Down(_) => (self.x, self.y - 1),
            Direction::Left(_) => (self.x - 1, self.y),
            Direction::Right(_) => (self.x + 1, self.y),
        }
    }
}

#[derive(Debug, Clone)]
struct Rope {
    knots: Vec<Knot>,
}

impl fmt::Display for Rope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.knots.is_empty() {
            return write!(f, "No knots in rope.");
        }
        for (i, k) in self.knots.clone().iter().enumerate() {
            let _ = write!(f, "{}", k);
            if i < (self.knots.len() - 1) {
                let _ = write!(f, " -> ");
            }
        }
        write!(f, "")
    }
}

impl Rope {
    fn new(n_knots: usize) -> Self {
        let mut knots = Vec::new();
        for _ in 0..n_knots {
            knots.push(Knot { x: 0, y: 0 });
        }
        Rope { knots }
    }

    fn step(&self, direction: Direction) -> Result<(), PuzzleError> {
        // TODO: Still working on the logic here.
        // May be best to create a new vector of knots during the process.
        // Should make ownership management easier.
        // Will also make it easier to know starting and finishing positions.

        // Move head knot.
        self.knots
            .first()
            .ok_or(PuzzleError::NoKnots)?
            .clone()
            .step(direction);
        // Update all other knots.
        Ok(())
    }

    fn perform_motion(&mut self, direction: Direction) -> Result<(), PuzzleError> {
        for _ in 0..direction.value() {
            self.step(direction)?;
        }
        Ok(())
    }
}

fn parse_directions(input_data: &str) -> Result<Vec<Direction>, PuzzleError> {
    let mut directions = Vec::new();
    for line in input_data.trim().lines().map(|x| x.trim()) {
        let steps = match line.split(' ').collect::<Vec<_>>()[1]
            .trim()
            .to_string()
            .parse::<usize>()
        {
            Ok(x) => Ok(x),
            Err(e) => Err(PuzzleError::ParseIntError(e)),
        }?;
        let dir = match line.chars().next() {
            Some('U') => Ok(Direction::Up(steps)),
            Some('D') => Ok(Direction::Down(steps)),
            Some('L') => Ok(Direction::Left(steps)),
            Some('R') => Ok(Direction::Right(steps)),
            Some(x) => Err(PuzzleError::UnknownDirection(x.to_string())),
            None => Err(PuzzleError::FailedParsing(line.to_string())),
        }?;
        directions.push(dir)
    }
    Ok(directions)
}

pub fn puzzle_1(input_data: &str) -> Result<usize, PuzzleError> {
    let directions = parse_directions(input_data)?;
    log::debug!("Directions:");
    for (i, d) in directions.iter().enumerate() {
        log::debug!("  {})  {}", i, d);
    }
    let mut rope = Rope::new(2);
    log::debug!("Rope: {}", rope);
    Ok(0)
}

pub fn main(data_dir: &str) {
    println!("Day 9: Rope Bridge");
    let data = load_raw(data_dir, 9, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match &answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("Error on Puzzle 1: {}", e),
    }
    assert_eq!(answer_1, Ok(1801));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match &answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("Error on Puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(209880));
}

#[cfg(test)]
mod tests {
    use crate::solutions::day09::puzzle_1;

    const EXAMPLE_1: &str = "
    R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2";

    #[test]
    fn puzzle_1_examples() {
        env_logger::init();
        assert_eq!(puzzle_1(EXAMPLE_1), Ok(21));
    }

    // #[test]
    // fn puzzle_2_examples() {
    //     env_logger::init();
    //     assert_eq!(puzzle_2(EXAMPLE_1), Ok(8));
    // }
}
