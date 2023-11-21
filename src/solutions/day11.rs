use crate::data::load_raw;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("Failed parsing integer.")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("String parsing error: {}", .0)]
    StringParsingError(String),
}

#[derive(Debug, Clone)]
pub struct Monkey {
    id: usize,
    items: Vec<isize>,
    operation_str: String,
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

#[derive(Debug, Clone)]
pub struct Monkeys {
    monkeys: Vec<Monkey>,
}

impl std::fmt::Display for Monkeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for monkey in self.monkeys.iter() {
            write!(f, "{}", monkey)?;
        }
        write!(f, "")
    }
}

impl Monkeys {
    fn new() -> Self {
        Monkeys {
            monkeys: Vec::new(),
        }
    }

    fn new_monkey(&mut self, monkey: &Monkey) {
        self.monkeys.push(monkey.clone());
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
        // Extract division test.

        let test_division_value: isize = extract_and_parse_last_word(lines[3])?;
        // Extract true result.

        let true_monkey: usize = extract_and_parse_last_word(lines[4])?;
        // Extract false result.
        let false_monkey: usize = extract_and_parse_last_word(lines[5])?;

        monkeys.new_monkey(&Monkey {
            id,
            items,
            operation_str,
            test_division_value,
            true_monkey,
            false_monkey,
        });
    }

    log::debug!("{}", monkeys);

    Ok(Monkeys {
        monkeys: Vec::new(),
    })
}

pub fn puzzle_1(input_data: &str) -> Result<isize, PuzzleError> {
    Ok(-1)
}

pub fn main(data_dir: &str) {
    println!("Day 11: Monkey in the Middle");
    let data = load_raw(data_dir, 10, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match &answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("Error on Puzzle 1: {}", e),
    }
    assert_eq!(answer_1, Ok(15220));

    // Puzzle 2.
    let answer_1 = puzzle_1(&data);
    match &answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("Error on Puzzle 1: {}", e),
    }
    assert_eq!(answer_1, Ok(15220));
}

#[cfg(test)]
mod tests {
    use crate::solutions::day11::{parse_input, puzzle_1};

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
    fn puzzle_1_example_1() {
        env_logger::init();
        let monkeys = parse_input(EXAMPLE_1).unwrap();
        assert!(false);
        // assert_eq!(puzzle_1(EXAMPLE_1), Ok(10605));
    }
}
