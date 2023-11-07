use crate::data::load_raw;
use textwrap::dedent;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("could not move crate from top of stack")]
    FailedTakeFromStack,
}

#[derive(Debug, Clone, Copy)]
struct CraneOp {
    n: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone)]
struct Supplies {
    stacks: Vec<Vec<char>>,
}

impl Supplies {
    fn new() -> Self {
        Supplies { stacks: vec![] }
    }

    // fn push(&mut self, stack: usize, crate_id: char) {
    //     match self.stacks.get_mut(stack) {
    //         Some(stack) => stack.push(crate_id),
    //         None => self.stacks.push(vec![crate_id]),
    //     };
    // }

    fn insert_at_bottom(&mut self, stack: usize, crate_id: char) {
        match self.stacks.get_mut(stack) {
            Some(stack) => stack.insert(0, crate_id),
            None => self.stacks.push(vec![crate_id]),
        };
    }

    fn drop_empty_crates(&mut self) {
        let _ = self
            .stacks
            .iter_mut()
            .map(|s| s.retain(|c| c != &' '))
            .collect::<Vec<_>>();
    }

    fn perform_9000(&mut self, crane_op: &CraneOp) -> Result<(), PuzzleError> {
        for _ in 0..crane_op.n {
            let c = self.stacks[crane_op.from - 1]
                .pop()
                .ok_or(PuzzleError::FailedTakeFromStack)?;
            self.stacks[crane_op.to - 1].push(c);
        }
        Ok(())
    }

    fn perform_9001(&mut self, crane_op: &CraneOp) -> Result<(), PuzzleError> {
        let mut temp_stack = vec![];
        for _ in 0..crane_op.n {
            let c = self.stacks[crane_op.from - 1]
                .pop()
                .ok_or(PuzzleError::FailedTakeFromStack)?;
            temp_stack.push(c);
        }
        let _ = temp_stack
            .iter()
            .rev()
            .map(|c| self.stacks[crane_op.to - 1].push(*c))
            .collect::<Vec<_>>();
        Ok(())
    }

    fn top_of_stacks(self) -> Result<String, PuzzleError> {
        let res = self
            .stacks
            .iter()
            .map(|s| s.last().ok_or(PuzzleError::FailedTakeFromStack))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .cloned()
            .collect::<String>();
        Ok(res)
    }
}

fn parse_input(data: &str) -> (Supplies, Vec<CraneOp>) {
    let data = dedent(data);
    let mut final_line = 0;

    // Parsing stacks.
    let mut supplies = Supplies::new();
    for (i, line) in data.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        if line.trim().starts_with('1') {
            final_line = i;
            break;
        }
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            supplies = supplies.clone();
            supplies.insert_at_bottom(i, c);
        }
    }
    supplies.drop_empty_crates();

    // Parsing crane instructions.
    let mut crane_operations: Vec<CraneOp> = vec![];
    for line in data.lines().skip(final_line + 1) {
        if line.trim().is_empty() {
            continue;
        }
        let split_insts: Vec<_> = line.trim().splitn(6, ' ').collect();
        let crane_op = CraneOp {
            n: split_insts[1].parse().unwrap(),
            from: split_insts[3].parse().unwrap(),
            to: split_insts[5].parse().unwrap(),
        };
        crane_operations.push(crane_op)
    }

    (supplies, crane_operations)
}

pub fn puzzle_1(input_data: &str) -> Result<String, PuzzleError> {
    let (mut supplies, crane_ops) = parse_input(input_data);
    for crane_op in crane_ops {
        let _ = supplies.perform_9000(&crane_op);
    }
    supplies.top_of_stacks()
}

pub fn puzzle_2(input_data: &str) -> Result<String, PuzzleError> {
    let (mut supplies, crane_ops) = parse_input(input_data);
    for crane_op in crane_ops {
        let _ = supplies.perform_9001(&crane_op);
    }
    supplies.top_of_stacks()
}

pub fn main(data_dir: &str) {
    println!("Day 5: Supply Stacks");
    let data = load_raw(data_dir, 5, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match &answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("Error on Puzzle 1: {}", e),
    }
    assert_eq!(answer_1, Ok("RFFFWBPNS".to_string()));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match &answer_2 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("Error on Puzzle 1: {}", e),
    }
    assert_eq!(answer_2, Ok("CQQBBJFCS".to_string()));
}

#[cfg(test)]
mod tests {
    use crate::solutions::day05::{puzzle_1, puzzle_2};

    const EXAMPLE_1: &str = "
        [D]
    [N] [C]
    [Z] [M] [P]
     1   2   3

    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2
    ";

    #[test]
    fn example_1_puzzle_1() {
        assert_eq!(puzzle_1(EXAMPLE_1), Ok("CMZ".to_string()))
    }

    #[test]
    fn example_1_puzzle_2() {
        assert_eq!(puzzle_2(EXAMPLE_1), Ok("MCD".to_string()))
    }
}
