use std::string::ParseError;

use crate::data::load;

fn parse_puzzle_input(input_data: &str) -> Result<Vec<u32>, ParseError> {
    let mut elves: Vec<u32> = vec![];
    let mut new_elf: u32 = 0;
    for item in input_data.lines() {
        if item.trim().is_empty() {
            elves.push(new_elf);
            new_elf = 0;
        } else {
            let item_value: u32 = item.trim().parse().unwrap();
            new_elf += item_value
        }
    }
    if new_elf > 0 {
        elves.push(new_elf);
    }
    Ok(elves)
}

pub fn puzzle_1(input_data: &str) -> Result<u32, ParseError> {
    Ok(*parse_puzzle_input(input_data)
        .unwrap()
        .iter()
        .max()
        .unwrap())
}

pub fn puzzle_2(input_data: &str) -> Result<u32, ParseError> {
    let mut elf_cals = parse_puzzle_input(input_data).unwrap();
    elf_cals.sort_by(|a, b| b.cmp(a));
    elf_cals.truncate(3);
    Ok(elf_cals.iter().sum())
}

pub fn main(data_dir: &str) {
    println!("Day 1");
    let data = load(data_dir, 1, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        _ => panic!("No solution to puzzle 1."),
    }
    assert_eq!(answer_1, Ok(68787));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        _ => panic!("No solution to puzzle 2."),
    }
    assert_eq!(answer_2, Ok(198041))
}

#[cfg(test)]
mod tests {
    use crate::solutions::day01::{puzzle_1, puzzle_2};

    const EXAMPLE_INPUT: &str = "
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000";

    #[test]
    fn example_1() {
        assert_eq!(puzzle_1(self::EXAMPLE_INPUT), Ok(24_000));
    }

    #[test]
    fn example_2() {
        assert_eq!(puzzle_2(self::EXAMPLE_INPUT), Ok(45000));
    }
}
