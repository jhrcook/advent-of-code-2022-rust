use std::ops::RangeInclusive;

use crate::data::load;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("could not split pair data")]
    SplittingPair(String),
    #[error("could not split elf range")]
    SplittingElfRange,
}

#[derive(Debug, Clone, Copy)]
struct ElfRange {
    from: u32,
    to: u32,
}

impl ElfRange {
    fn from_str(pair_str: &str) -> Self {
        let mut split_data = pair_str.split('-').collect::<Vec<&str>>();
        assert_eq!(split_data.len(), 2);
        ElfRange {
            to: split_data
                .pop()
                .ok_or(PuzzleError::SplittingElfRange)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
            from: split_data
                .pop()
                .ok_or(PuzzleError::SplittingElfRange)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        }
    }

    fn to_range(self) -> RangeInclusive<u32> {
        RangeInclusive::new(self.from, self.to)
    }

    fn contains(self, elf: &ElfRange) -> bool {
        let r = self.to_range();
        (elf.from..=elf.to).map(|x| r.contains(&x)).all(|x| x)
    }

    fn overlaps(self, elf: &ElfRange) -> bool {
        let r = self.to_range();
        (elf.from..=elf.to).map(|x| r.contains(&x)).any(|x| x)
    }
}

fn parse_data(line: &str) -> Result<(ElfRange, ElfRange), PuzzleError> {
    let mut pairs: Vec<&str> = line.split(',').collect();
    match pairs.len() {
        2 => (),
        _ => return Err(PuzzleError::SplittingPair(line.to_string())),
    };
    let elf2 = ElfRange::from_str(
        pairs
            .pop()
            .ok_or(PuzzleError::SplittingPair(line.to_string()))?,
    );
    let elf1 = ElfRange::from_str(
        pairs
            .pop()
            .ok_or(PuzzleError::SplittingPair(line.to_string()))?,
    );
    Ok((elf1, elf2))
}

pub fn puzzle_1(input_data: &str) -> Result<u32, PuzzleError> {
    let mut count = 0;
    for line in input_data
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
    {
        let (elf1, elf2) = parse_data(line)?;
        if elf1.contains(&elf2) | elf2.contains(&elf1) {
            count += 1;
        }
    }
    Ok(count)
}

pub fn puzzle_2(input_data: &str) -> Result<u32, PuzzleError> {
    let mut count = 0;
    for line in input_data
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
    {
        let (elf1, elf2) = parse_data(line)?;
        if elf1.overlaps(&elf2) {
            count += 1;
        }
    }
    Ok(count)
}

pub fn main(data_dir: &str) {
    println!("Day 4: Camp Cleanup");
    let data = load(data_dir, 4, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("Error on Puzzle 1: {}", e),
    }
    assert_eq!(answer_1, Ok(507));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("Error on Puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(897))
}

#[cfg(test)]
mod tests {
    use crate::solutions::day04::{puzzle_1, puzzle_2};

    const EXAMPLE_1: &str = "
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    ";

    #[test]
    fn example_1_puzzle_1() {
        assert_eq!(puzzle_1(EXAMPLE_1), Ok(2))
    }
    #[test]
    fn example_1_puzzle_2() {
        assert_eq!(puzzle_2(EXAMPLE_1), Ok(4))
    }
}
