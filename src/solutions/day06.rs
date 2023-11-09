use crate::data::load_raw;
use std::collections::HashSet;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("no window with all unique characters found")]
    NoUniqueWindowFound,
}

fn find_unique_window(data_stream: &str, window_size: usize) -> Result<usize, PuzzleError> {
    for (i, window) in data_stream
        .trim()
        .chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
    {
        if window.iter().collect::<HashSet<_>>().len() == window_size {
            return Ok(i + window_size);
        }
    }
    Err(PuzzleError::NoUniqueWindowFound)
}

pub fn puzzle_1(input_data: &str) -> Result<usize, PuzzleError> {
    find_unique_window(input_data, 4)
}

pub fn puzzle_2(input_data: &str) -> Result<usize, PuzzleError> {
    find_unique_window(input_data, 14)
}

pub fn main(data_dir: &str) {
    println!("Day 6: Tuning Trouble");
    let data = load_raw(data_dir, 6, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match &answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("Error on Puzzle 1: {}", e),
    }
    assert_eq!(answer_1, Ok(1210));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match &answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("Error on Puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(3476));
}

#[cfg(test)]
mod tests {
    use crate::solutions::day06::{puzzle_1, puzzle_2};

    const EXAMPLE_1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const EXAMPLE_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const EXAMPLE_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const EXAMPLE_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const EXAMPLE_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn puzzle_1_examples() {
        assert_eq!(puzzle_1(EXAMPLE_1), Ok(7));
        assert_eq!(puzzle_1(EXAMPLE_2), Ok(5));
        assert_eq!(puzzle_1(EXAMPLE_3), Ok(6));
        assert_eq!(puzzle_1(EXAMPLE_4), Ok(10));
        assert_eq!(puzzle_1(EXAMPLE_5), Ok(11));
    }

    #[test]
    fn puzzle_2_examples() {
        assert_eq!(puzzle_2(EXAMPLE_1), Ok(19));
        assert_eq!(puzzle_2(EXAMPLE_2), Ok(23));
        assert_eq!(puzzle_2(EXAMPLE_3), Ok(23));
        assert_eq!(puzzle_2(EXAMPLE_4), Ok(29));
        assert_eq!(puzzle_2(EXAMPLE_5), Ok(26));
    }
}
