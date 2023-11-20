use crate::data::load_raw;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("Iterating over line returned `None`.")]
    EmptyInputDataLine,
    #[error("Unexpected operation in input line: {}", .0)]
    UnexpectedOperation(String),
}

pub enum Operation {
    Noop,
    Addx(isize),
}

pub struct CathodeRayTube {
    x: isize,
    cycles_complete: isize,
    total_signal_strength: isize,
}

impl CathodeRayTube {
    fn new() -> Self {
        CathodeRayTube {
            x: 1,
            // Specifically, the number of cycles *completed*. Therefore, a value of 1
            // indicates that 1 cycle has been completed and it is currently in cycle 2.
            cycles_complete: 0,
            total_signal_strength: 0,
        }
    }

    /// The current execution cycle number.
    fn current_cycle(&self) -> isize {
        self.cycles_complete + 1
    }

    fn update_signal_strength(&mut self) {
        match self.current_cycle() {
            20 | 60 | 100 | 140 | 180 | 220 => {
                self.total_signal_strength += (self.current_cycle()) * self.x
            }
            _ => (),
        }
    }

    fn compute_cycle(&mut self, add_x: isize) {
        self.update_signal_strength();
        self.cycles_complete += 1;
        self.x += add_x;
    }

    fn perform(&mut self, op: &Operation) {
        match op {
            Operation::Noop => self.compute_cycle(0),
            Operation::Addx(x) => {
                self.compute_cycle(0);
                self.compute_cycle(*x);
            }
        };
    }
}

pub fn parse_input(input_data: &str) -> Result<Vec<Operation>, PuzzleError> {
    let mut operations = Vec::new();
    for line in input_data.trim().lines() {
        let pieces: Vec<&str> = line.trim().split(' ').collect();
        match pieces.first() {
            Some(&"noop") => operations.push(Operation::Noop),
            Some(&"addx") => operations.push(Operation::Addx(
                pieces[1].to_string().parse::<isize>().unwrap(),
            )),
            Some(_) => return Err(PuzzleError::UnexpectedOperation(line.to_string())),
            None => return Err(PuzzleError::EmptyInputDataLine),
        };
    }
    Ok(operations)
}

pub fn puzzle_1(input_data: &str) -> Result<isize, PuzzleError> {
    let operations = parse_input(input_data)?;
    let mut ray_tube = CathodeRayTube::new();
    for op in operations {
        ray_tube.perform(&op);
    }
    Ok(ray_tube.total_signal_strength)
}

pub fn main(data_dir: &str) {
    println!("Day 10: Cathode-Ray Tube");
    let data = load_raw(data_dir, 10, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match &answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("Error on Puzzle 1: {}", e),
    }
    assert_eq!(answer_1, Ok(15220));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match &answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("Error on Puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(2511));
}

#[cfg(test)]
mod tests {
    use crate::data::load_raw;
    use crate::solutions::day10::{parse_input, puzzle_1, CathodeRayTube};

    const EXAMPLE_1: &str = "
    noop
    addx 3
    addx -5
    ";

    #[test]
    fn puzzle_1_example_1() {
        // Test parsing of input operations.
        let example_operations = parse_input(EXAMPLE_1);
        assert!(example_operations.is_ok());
        assert_eq!(example_operations.unwrap().len(), 3);

        // Test performing operations.
        let example_operations = parse_input(EXAMPLE_1).unwrap();
        let mut tube = CathodeRayTube::new();
        for op in example_operations.iter() {
            tube.perform(op);
        }
        assert_eq!(tube.x, -1);
        assert_eq!(tube.cycles_complete, 5);
        assert_eq!(tube.total_signal_strength, 0);

        assert_eq!(puzzle_1(EXAMPLE_1), Ok(0));
    }

    #[test]
    fn puzzle_1_example_2() {
        let data = load_raw("puzzle-input", 10, Some("_ex1"));
        assert_eq!(puzzle_1(&data), Ok(13140));
    }

    // #[test]
    // fn puzzle_2_examples() {
    //     assert_eq!(puzzle_2(EXAMPLE_1), Ok(1));
    // }
}
