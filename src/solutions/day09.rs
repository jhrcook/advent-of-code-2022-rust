use crate::data::load_raw;
use std::cmp::max;
use std::{collections::HashSet, fmt};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Knot {
    x: isize,
    y: isize,
}

impl fmt::Display for Knot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Knot {
    fn step(&self, direction: &Direction) -> Knot {
        match direction {
            Direction::Up(_) => Knot {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down(_) => Knot {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left(_) => Knot {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right(_) => Knot {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    fn euclidean_distance(&self, to: &Knot) -> f32 {
        let dx = (self.x - to.x).pow(2);
        let dy = (self.y - to.y).pow(2);
        f32::sqrt((dx + dy) as f32)
    }

    fn chebyshev_distance(&self, to: &Knot) -> usize {
        max(self.x.abs_diff(to.x), self.y.abs_diff(to.y))
    }

    fn move_towards(&self, lead_knot: &Knot) -> Knot {
        if self.chebyshev_distance(lead_knot) <= 1 {
            return *self;
        }
        let mut closest_new_knot = *self;
        let mut closest_knot_dist = 10.0;
        for dx in -1..=1 {
            for dy in -1..=1 {
                let new_knot = Knot {
                    x: self.x + dx,
                    y: self.y + dy,
                };
                let dist = new_knot.euclidean_distance(lead_knot);
                if dist < closest_knot_dist {
                    closest_new_knot = new_knot;
                    closest_knot_dist = dist;
                }
            }
        }
        closest_new_knot
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

    fn step(&mut self, direction: &Direction) -> Result<(), PuzzleError> {
        // New knot locations.
        let mut new_knots = Vec::new();

        // Move head knot based on the direction.
        let new_head = self
            .knots
            .first()
            .ok_or(PuzzleError::NoKnots)?
            .step(direction);
        new_knots.push(new_head);

        // Update all other knots.
        for (i, k) in self.knots.iter().skip(1).enumerate() {
            let lead_k = new_knots[i]; // Because of skip, the index points to prior knot.
            let new_k = k.move_towards(&lead_k);
            new_knots.push(new_k);
        }

        self.knots = new_knots;
        Ok(())
    }

    fn perform_motion(&mut self, direction: &Direction) -> Result<HashSet<Knot>, PuzzleError> {
        let mut tail_locations = HashSet::new();
        for _ in 0..direction.value() {
            self.step(direction)?;
            tail_locations.insert(*self.knots.last().ok_or(PuzzleError::NoKnots)?);
        }
        Ok(tail_locations)
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
    let mut rope = Rope::new(2);

    let mut tail_locations = HashSet::new();
    for direction in directions.iter() {
        tail_locations.extend(rope.perform_motion(direction)?);
    }

    Ok(tail_locations.len())
}

pub fn puzzle_2(input_data: &str) -> Result<usize, PuzzleError> {
    let directions = parse_directions(input_data)?;
    let mut rope = Rope::new(10);

    let mut tail_locations = HashSet::new();
    for direction in directions.iter() {
        tail_locations.extend(rope.perform_motion(direction)?);
    }

    Ok(tail_locations.len())
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
    assert_eq!(answer_1, Ok(6332));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match &answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("Error on Puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(2511));
}

#[cfg(test)]
mod tests {
    use crate::solutions::day09::{puzzle_1, puzzle_2};

    const EXAMPLE_1: &str = "
    R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2";

    const EXAMPLE_2: &str = "
    R 5
    U 8
    L 8
    D 3
    R 17
    D 10
    L 25
    U 20";

    #[test]
    fn puzzle_1_examples() {
        assert_eq!(puzzle_1(EXAMPLE_1), Ok(13));
    }

    #[test]
    fn puzzle_2_examples() {
        assert_eq!(puzzle_2(EXAMPLE_1), Ok(1));
        assert_eq!(puzzle_2(EXAMPLE_2), Ok(36));
    }
}
