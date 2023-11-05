use crate::data::load;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Day3Error {
    #[error("no shared itmes")]
    NoSharedItem,
}

struct AlphaScore {
    score: HashMap<char, u32>,
}

impl AlphaScore {
    fn new() -> Self {
        let alphabet = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
        let mut score: HashMap<char, u32> = HashMap::new();
        for (i, a) in alphabet.chars().enumerate() {
            score.insert(a, 1 + i as u32);
        }
        AlphaScore { score }
    }
}

#[derive(Debug)]
struct RuckSack {
    c1: Vec<char>,
    c2: Vec<char>,
}

impl RuckSack {
    fn from_str(contents: &str) -> Self {
        let chars: Vec<char> = contents.chars().collect();
        let n_chars = chars.len();
        RuckSack {
            c1: chars[0..(n_chars / 2)].to_vec(),
            c2: chars[(n_chars / 2)..n_chars].to_vec(),
        }
    }
}

impl RuckSack {
    fn first_shared_item(self) -> Result<char, Day3Error> {
        let h1: HashSet<char> = HashSet::from_iter(self.c1);
        let h2: HashSet<char> = HashSet::from_iter(self.c2);
        match h1.intersection(&h2).next() {
            Some(a) => Ok(*a),
            None => Err(Day3Error::NoSharedItem),
        }
    }
}

pub fn puzzle_1(input_data: &str) -> Result<u32, Day3Error> {
    let score_map = AlphaScore::new();
    let mut tally = 0;
    for line in input_data.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let rucksack = RuckSack::from_str(line.trim());
        tally += score_map.score[&rucksack.first_shared_item().unwrap()];
    }
    Ok(tally)
}

pub fn puzzle_2(input_data: &str) -> Result<u32, Day3Error> {
    let mut tally = 0;
    let scorer = AlphaScore::new();
    let iter: Vec<&str> = input_data
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect();
    for group in iter.windows(3).step_by(3) {
        let mut counter = HashMap::new();
        for line in group {
            let set: HashSet<char> = line.chars().collect();
            for c in set {
                counter.entry(c).and_modify(|e| *e += 1).or_insert(1);
            }
        }
        for (c, n) in counter.iter() {
            if n == &3 {
                tally += scorer.score[c];
            }
        }
    }
    Ok(tally)
}

pub fn main(data_dir: &str) {
    println!("Day 3: Rucksack Reorganization");
    let data = load(data_dir, 3, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("Error on Puzzle 1: {}", e),
    }
    assert_eq!(answer_1, Ok(7446));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("Error on Puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(2646))
}

#[cfg(test)]
mod tests {
    use crate::solutions::day03::{puzzle_1, puzzle_2};

    const EXAMPLE_1: &str = "
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw
    ";

    #[test]
    fn example_1_puzzle_1() {
        assert_eq!(puzzle_1(EXAMPLE_1), Ok(157))
    }
    #[test]
    fn example_1_puzzle_2() {
        assert_eq!(puzzle_2(EXAMPLE_1), Ok(70))
    }
}
