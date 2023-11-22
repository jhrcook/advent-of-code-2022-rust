use std::collections::HashMap;

use crate::data::load_raw;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("Failed parsing integer.")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("String parsing error: {}", .0)]
    StringParsingError(String),
    #[error("Unrecognized math operator: {}", .0)]
    UnrecognizedMathOperator(String),
    #[error("Division operator is not supported because rounding is unspecified.")]
    UnclearHowToRoundDivision,
    #[error("No monkey with ID {}.", .0)]
    NoMonkeyWithId(usize),
}

#[derive(Debug, Clone, Copy)]
enum OperationVar {
    Constant(isize),
    Old,
}

impl OperationVar {
    fn identify_var(x: &str) -> Result<Self, PuzzleError> {
        if x == "old" {
            return Ok(OperationVar::Old);
        }
        let parsed_val = x.parse::<isize>()?;
        Ok(OperationVar::Constant(parsed_val))
    }
}

#[derive(Debug, Clone, Copy)]
enum MathOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl MathOperator {
    fn identify_op(op_str: &str) -> Result<Self, PuzzleError> {
        match op_str.trim() {
            "+" => Ok(MathOperator::Add),
            "-" => Ok(MathOperator::Subtract),
            "*" => Ok(MathOperator::Multiply),
            "/" => Ok(MathOperator::Divide),
            _ => Err(PuzzleError::UnrecognizedMathOperator(op_str.to_string())),
        }
    }

    fn do_math(&self, x: &isize, y: &isize) -> Result<isize, PuzzleError> {
        match self {
            MathOperator::Add => Ok(x + y),
            MathOperator::Subtract => Ok(x - y),
            MathOperator::Multiply => Ok(x * y),
            MathOperator::Divide => Err(PuzzleError::UnclearHowToRoundDivision),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MonkeyOperation {
    x: OperationVar,
    y: OperationVar,
    op: MathOperator,
}

impl MonkeyOperation {
    fn from_str(input: &str) -> Result<Self, PuzzleError> {
        let split_input = input.trim().split(' ').collect::<Vec<_>>();
        let x = OperationVar::identify_var(split_input[0])?;
        let y = OperationVar::identify_var(split_input[2])?;
        let op = MathOperator::identify_op(split_input[1])?;
        Ok(MonkeyOperation { x, y, op })
    }

    fn perform(&self, old_val: &isize) -> Result<isize, PuzzleError> {
        let x_val: isize = match self.x {
            OperationVar::Constant(a) => a,
            OperationVar::Old => *old_val,
        };
        let y_val: isize = match self.y {
            OperationVar::Constant(a) => a,
            OperationVar::Old => *old_val,
        };
        self.op.do_math(&x_val, &y_val)
    }
}

#[derive(Debug, Clone, Copy)]
struct MonkeyDecision {
    receiver_monkey_id: usize,
    item_value: isize,
}

impl MonkeyDecision {
    fn new(receiver_monkey_id: usize, item_value: isize) -> Self {
        MonkeyDecision {
            receiver_monkey_id,
            item_value,
        }
    }
}

impl std::fmt::Display for MonkeyDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "item {} to monkey {}",
            self.item_value, self.receiver_monkey_id
        )
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    id: usize,
    items: Vec<isize>,
    _operation_str: String,
    operation: MonkeyOperation,
    test_division_value: isize,
    true_monkey: usize,
    false_monkey: usize,
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Monkey {}  -  items: {}",
            self.id,
            self.items
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Monkey {
    fn inspect_items_1(&mut self) -> Result<Vec<MonkeyDecision>, PuzzleError> {
        log::debug!(
            "Monkey {} is inspecting {} items.",
            self.id,
            self.items.len()
        );
        let mut results = Vec::new();
        for item in self.items.iter() {
            log::debug!("Starting worry level: {}.", item);
            let post_inspection_val = self.operation.perform(item)?;
            log::debug!("Post-inspection worry level: {}", post_inspection_val);
            let worry_reduced_val = (post_inspection_val as f32 / 3.0).floor() as isize;
            log::debug!("Reduced worry level: {}", worry_reduced_val);
            let receiving_monkey = match worry_reduced_val % self.test_division_value {
                0 => {
                    log::debug!("Test result TRUE  ->  monkey {}", self.true_monkey);
                    self.true_monkey
                }
                _ => {
                    log::debug!("Test result FALSE  ->  monkey {}", self.false_monkey);
                    self.false_monkey
                }
            };
            results.push(MonkeyDecision::new(receiving_monkey, worry_reduced_val));
        }
        self.items = Vec::new();
        log::debug!("Final results for monkey:\n{:?}", results);
        Ok(results)
    }

    fn inspect_items_2(&mut self) -> Result<Vec<MonkeyDecision>, PuzzleError> {
        log::debug!(
            "Monkey {} is inspecting {} items.",
            self.id,
            self.items.len()
        );
        let mut results = Vec::new();
        for item in self.items.iter() {
            log::debug!("Starting worry level: {}.", item);
            let post_inspection_val = self.operation.perform(item)?;
            log::debug!("Post-inspection worry level: {}", post_inspection_val);
            let receiving_monkey = match post_inspection_val % self.test_division_value {
                0 => {
                    log::debug!("Test result TRUE  ->  monkey {}", self.true_monkey);
                    self.true_monkey
                }
                _ => {
                    log::debug!("Test result FALSE  ->  monkey {}", self.false_monkey);
                    self.false_monkey
                }
            };
            results.push(MonkeyDecision::new(receiving_monkey, post_inspection_val));
        }
        self.items = Vec::new();
        log::debug!("Final results for monkey:\n{:?}", results);
        Ok(results)
    }
}

#[derive(Debug, Clone)]
pub struct Monkeys {
    order: Vec<usize>,
    monkeys: HashMap<usize, Monkey>,
}

impl std::fmt::Display for Monkeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, monkey) in self.monkeys.iter() {
            write!(f, "{k}: {}", monkey)?;
        }
        write!(f, "")
    }
}

impl Monkeys {
    fn new() -> Self {
        Monkeys {
            order: Vec::new(),
            monkeys: HashMap::new(),
        }
    }

    fn new_monkey(&mut self, monkey: &Monkey) {
        self.order.push(monkey.id);
        self.monkeys.insert(monkey.id, monkey.clone());
    }

    fn disperse_results(
        &mut self,
        inspection_descisions: &[MonkeyDecision],
    ) -> Result<(), PuzzleError> {
        for decision in inspection_descisions.iter() {
            self.monkeys
                .get_mut(&decision.receiver_monkey_id)
                .ok_or(PuzzleError::NoMonkeyWithId(decision.receiver_monkey_id))?
                .items
                .push(decision.item_value);
        }
        Ok(())
    }

    fn monkey_divisor(&self) -> isize {
        self.monkeys
            .values()
            .map(|m| m.test_division_value)
            .product()
    }

    fn reduce_all_monkey_values(&mut self) {
        log::info!("Reducing monkey values.");
        let div = self.monkey_divisor();
        for monkey in self.monkeys.values_mut() {
            monkey.items = monkey.items.iter().map(|x| *x % div).collect();
        }
    }

    fn perform_round(
        &mut self,
        item_counter: &mut HashMap<usize, usize>,
        div_by_3: bool,
    ) -> Result<(), PuzzleError> {
        for monkey_id in self.order.clone().iter() {
            let monkey = self
                .monkeys
                .get_mut(monkey_id)
                .ok_or(PuzzleError::NoMonkeyWithId(*monkey_id))?;
            *item_counter.entry(*monkey_id).or_insert(0) += monkey.items.len();
            let decision_results = match div_by_3 {
                true => monkey.inspect_items_1(),
                false => monkey.inspect_items_2(),
            }?;
            self.disperse_results(&decision_results)?;
        }
        if !div_by_3 {
            self.reduce_all_monkey_values()
        }
        Ok(())
    }
}

fn extract_and_parse_last_word<T: std::str::FromStr>(s: &str) -> Result<T, PuzzleError> {
    s.split(' ')
        .last()
        .ok_or(PuzzleError::StringParsingError(s.to_string()))?
        .parse::<T>()
        .or(Err(PuzzleError::StringParsingError(s.to_string())))
}

pub fn parse_input(input_data: &str) -> Result<Monkeys, PuzzleError> {
    let mut monkeys = Monkeys::new();
    for lines in input_data
        .trim()
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .windows(6)
        .step_by(6)
    {
        // Extract ID.
        let id: usize = extract_and_parse_last_word(&lines[0].replace(':', ""))?;

        // Extract starting items.
        let items_string = lines[1].replace("Starting items: ", "");
        let items = items_string
            .split(", ")
            .map(|x| x.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?;
        // Extract operation.
        let operation_str = lines[2].replace("Operation: new = ", "").to_string();
        let operation = MonkeyOperation::from_str(&operation_str)?;
        // Extract division test.
        let test_division_value: isize = extract_and_parse_last_word(lines[3])?;
        // Extract true result.
        let true_monkey: usize = extract_and_parse_last_word(lines[4])?;
        // Extract false result.
        let false_monkey: usize = extract_and_parse_last_word(lines[5])?;

        monkeys.new_monkey(&Monkey {
            id,
            items,
            _operation_str: operation_str,
            operation,
            test_division_value,
            true_monkey,
            false_monkey,
        });
    }
    Ok(monkeys)
}

pub fn puzzle_1(input_data: &str) -> Result<usize, PuzzleError> {
    let mut monkeys = parse_input(input_data)?;
    let mut item_counter = HashMap::new();
    for i in 0..20 {
        log::info!("Round {}", i);
        monkeys.perform_round(&mut item_counter, true)?;
    }
    let mut item_counts = item_counter.values().collect::<Vec<_>>();
    item_counts.sort();
    item_counts.reverse();
    Ok(item_counts[0] * item_counts[1])
}

pub fn puzzle_2(input_data: &str, n_rounds: usize) -> Result<usize, PuzzleError> {
    let mut monkeys = parse_input(input_data)?;
    let mut item_counter = HashMap::new();
    for i in 0..n_rounds {
        log::info!("Round {}", i);
        monkeys.perform_round(&mut item_counter, false)?;
    }
    let mut item_counts = item_counter.values().collect::<Vec<_>>();
    item_counts.sort();
    item_counts.reverse();
    Ok(item_counts[0] * item_counts[1])
}

pub fn main(data_dir: &str) {
    println!("Day 11: Monkey in the Middle");
    let data = load_raw(data_dir, 11, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match &answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("Error on Puzzle 1: {}", e),
    }
    assert_eq!(answer_1, Ok(113232));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data, 10000);
    match &answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("Error on Puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(29703395016));
}

#[cfg(test)]
mod tests {
    use crate::solutions::day11::{puzzle_1, puzzle_2};

    const EXAMPLE_1: &str = "
    Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3

    Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0

    Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3

    Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1
    ";

    #[test]
    fn puzzle_1_example() {
        let res = puzzle_1(EXAMPLE_1);
        assert_eq!(res, Ok(10605));
    }

    #[test]
    fn puzzle_2_example_n1() {
        let res: Result<usize, crate::solutions::day11::PuzzleError> = puzzle_2(EXAMPLE_1, 1);
        assert_eq!(res, Ok(24));
    }

    #[test]
    fn puzzle_2_example_n20() {
        let res: Result<usize, crate::solutions::day11::PuzzleError> = puzzle_2(EXAMPLE_1, 20);
        assert_eq!(res, Ok(10197));
    }

    #[test]
    fn puzzle_2_example_n10000() {
        let res = puzzle_2(EXAMPLE_1, 10000);
        assert_eq!(res, Ok(2713310158));
    }
}
