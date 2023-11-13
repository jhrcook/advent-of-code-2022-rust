use crate::data::load_raw;
use std::collections::{HashMap, HashSet};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("Could not parse the following to int: {}.", .0)]
    InputValueParsingError(String),
    #[error("Cannot perform computation on empty grid.")]
    EmptyGrid,
    #[error("Position not in grid: {}", .0)]
    UnknownPosition(Position),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    row: usize,
    col: usize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Debug, Clone)]
struct Grid {
    array: HashMap<Position, usize>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (pos, size) in self.array.iter() {
            let _ = writeln!(f, "{} -> {}", pos, size);
        }
        write!(f, "")
    }
}

impl Grid {
    fn new() -> Self {
        Grid {
            array: HashMap::new(),
        }
    }

    // Add a value to the grid.
    fn add_value(&mut self, pos: &Position, val: &usize) {
        self.array.insert(*pos, *val);
    }

    fn get_value(&self, pos: &Position) -> Result<usize, PuzzleError> {
        match self.array.get(pos) {
            Some(x) => Ok(*x),
            None => Err(PuzzleError::UnknownPosition(*pos)),
        }
    }

    // Retrieve the width of the grid.
    fn width(&self) -> Result<usize, PuzzleError> {
        match self.array.keys().map(|p| p.col).max() {
            Some(x) => Ok(x + 1),
            None => Err(PuzzleError::EmptyGrid),
        }
    }

    // Retrieve the height of the grid.
    fn height(&self) -> Result<usize, PuzzleError> {
        match self.array.keys().map(|p| p.row).max() {
            Some(x) => Ok(x + 1),
            None => Err(PuzzleError::EmptyGrid),
        }
    }

    fn is_hidden(&self, p: &Position) -> Result<bool, PuzzleError> {
        let height = self.get_value(p)?;
        if (p.row == 0)
            | (p.row == (self.height()? - 1))
            | (p.col == 0)
            | (p.col == (self.width()? - 1))
        {
            log::debug!("Pos. {} is VISIBLE.", p);
            return Ok(false);
        }

        let left = (0..p.col)
            .map(|c| self.get_value(&Position { row: p.row, col: c }))
            .any(|h| match h {
                Ok(x) => x >= height,
                Err(e) => panic!("{}", e),
            });
        if !left {
            log::debug!("Pos. {} is VISIBLE.", p);
            return Ok(false);
        }

        let right = ((p.col + 1)..self.width()?)
            .map(|c| self.get_value(&Position { row: p.row, col: c }))
            .any(|h| match h {
                Ok(x) => x >= height,
                Err(e) => panic!("{}", e),
            });
        if !right {
            log::debug!("Pos. {} is VISIBLE.", p);
            return Ok(false);
        }

        let up = (0..p.row)
            .map(|r| self.get_value(&Position { row: r, col: p.col }))
            .any(|h| match h {
                Ok(x) => x >= height,
                Err(e) => panic!("{}", e),
            });
        if !up {
            log::debug!("Pos. {} is VISIBLE.", p);
            return Ok(false);
        }

        let down = ((p.row + 1)..self.height()?)
            .map(|r| self.get_value(&Position { row: r, col: p.col }))
            .any(|h| match h {
                Ok(x) => x >= height,
                Err(e) => panic!("{}", e),
            });
        if !down {
            log::debug!("Pos. {} is VISIBLE.", p);
            return Ok(false);
        }

        log::debug!("Pos. {} is HIDDEN.", p);
        Ok(true)
    }

    fn scenic_score(&self, p: &Position) -> Result<usize, PuzzleError> {
        let pos_height = self.get_value(p)?;

        let mut left = 0;
        for c in (0..p.col).rev() {
            let h = self.get_value(&Position { row: p.row, col: c })?;
            if pos_height > h {
                left += 1;
            } else if pos_height <= h {
                left += 1;
                break;
            }
        }
        if left == 0 {
            return Ok(0);
        }

        let mut right = 0;
        for c in (p.col + 1)..self.width()? {
            let h = self.get_value(&Position { row: p.row, col: c })?;
            if pos_height > h {
                right += 1;
            } else if pos_height <= h {
                right += 1;
                break;
            }
        }
        if right == 0 {
            return Ok(0);
        }

        let mut up = 0;
        for r in (0..p.row).rev() {
            let h = self.get_value(&Position { row: r, col: p.col })?;
            if pos_height > h {
                up += 1;
            } else if pos_height <= h {
                up += 1;
                break;
            }
        }
        if up == 0 {
            return Ok(0);
        }

        let mut down = 0;
        for r in (p.row + 1)..self.height()? {
            let h = self.get_value(&Position { row: r, col: p.col })?;
            if pos_height > h {
                down += 1;
            } else if pos_height <= h {
                down += 1;
                break;
            }
        }
        if down == 0 {
            return Ok(0);
        }

        Ok(left * right * up * down)
    }
}

fn create_forest_grid(input_data: &str) -> Result<Grid, PuzzleError> {
    let mut forest_grid = Grid::new();
    for (row, line) in input_data.trim().lines().enumerate() {
        for (col, height) in line.trim().chars().enumerate() {
            let height = match String::from(height).parse::<usize>() {
                Ok(x) => Ok(x),
                Err(_) => Err(PuzzleError::InputValueParsingError(String::from(height))),
            }?;
            forest_grid.add_value(&Position { row, col }, &height);
        }
    }
    Ok(forest_grid)
}

pub fn puzzle_1(input_data: &str) -> Result<usize, PuzzleError> {
    let forest_grid = create_forest_grid(input_data)?;
    let num_hidden: usize = forest_grid
        .array
        .keys()
        .map(|p| forest_grid.clone().is_hidden(p))
        .collect::<Result<Vec<bool>, PuzzleError>>()?
        .iter()
        .filter(|hidden| !*hidden)
        .collect::<Vec<_>>()
        .len();
    Ok(num_hidden)
}
pub fn puzzle_2(input_data: &str) -> Result<usize, PuzzleError> {
    let forest_grid = create_forest_grid(input_data)?;
    let scenic_scores = forest_grid
        .array
        .keys()
        .map(|p| forest_grid.clone().scenic_score(p))
        .collect::<Result<HashSet<usize>, PuzzleError>>()?;
    if let Some(highest_score) = scenic_scores.iter().max() {
        Ok(*highest_score)
    } else {
        Err(PuzzleError::EmptyGrid)
    }
}

pub fn main(data_dir: &str) {
    println!("Day 8: Treetop Tree House");
    let data = load_raw(data_dir, 8, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match &answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("Error on Puzzle 1: {}", e),
    }
    assert_eq!(answer_1, Ok(1801));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match &answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("Error on Puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(209880));
}

#[cfg(test)]
mod tests {
    use crate::solutions::day08::{puzzle_1, puzzle_2};

    const EXAMPLE_1: &str = "
    30373
    25512
    65332
    33549
    35390
    ";

    #[test]
    fn puzzle_1_examples() {
        env_logger::init();
        assert_eq!(puzzle_1(EXAMPLE_1), Ok(21));
    }

    #[test]
    fn puzzle_2_examples() {
        env_logger::init();
        assert_eq!(puzzle_2(EXAMPLE_1), Ok(8));
    }
}
