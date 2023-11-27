use std::collections::{HashMap, HashSet};

use crate::data::load_raw;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("Failed parsing integer.")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Unkown height: {}", .0)]
    UnknownHeight(String),
    #[error("No start coordinate.")]
    NoStartCoord,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    r: usize,
    c: usize,
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.r, self.c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Height {
    Start(usize),
    End(usize),
    H(usize),
}

impl Height {
    fn get_height(&self) -> usize {
        match self {
            Self::Start(x) | Self::End(x) | Self::H(x) => *x,
        }
    }
}

impl std::fmt::Display for Height {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Height::Start(x) => write!(f, "Start({})", x),
            Height::End(x) => write!(f, "End({})", x),
            Height::H(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug, Clone)]
struct HeightMap {
    heights: HashMap<Coord, Height>,
}

impl HeightMap {
    fn new() -> Self {
        HeightMap {
            heights: HashMap::new(),
        }
    }

    fn start_coord(&self) -> Result<Coord, PuzzleError> {
        for (coord, value) in self.heights.iter() {
            if let Height::Start(_) = value {
                return Ok(*coord);
            }
        }
        Err(PuzzleError::NoStartCoord)
    }

    fn add_value(&mut self, row: usize, col: usize, value: Height) {
        log::debug!("Adding value: [{},{}] -> {}", row, col, value);
        self.heights.insert(Coord { r: row, c: col }, value);
    }
}

struct HeightTranslator {
    score_map: HashMap<char, Height>,
}

impl HeightTranslator {
    fn new() -> Self {
        let mut score_map = HashMap::new();
        for (i, c) in ('a'..='z').enumerate() {
            score_map.insert(c, Height::H(i));
        }
        Self { score_map }
    }

    fn translate(&self, input: &char) -> Result<Height, PuzzleError> {
        match input {
            'S' => Ok(Height::Start(0)),
            'E' => Ok(Height::End(25)),
            x => self
                .score_map
                .get(x)
                .cloned()
                .ok_or(PuzzleError::UnknownHeight(input.to_string())),
        }
    }
}

fn parse_input(input_data: &str) -> Result<HeightMap, PuzzleError> {
    let mut heightmap = HeightMap::new();
    let height_translator = HeightTranslator::new();
    for (row, line) in input_data.trim().lines().map(|a| a.trim()).enumerate() {
        for (col, c) in line.chars().enumerate() {
            heightmap.add_value(row, col, height_translator.translate(&c)?)
        }
    }
    Ok(heightmap)
}

#[derive(Debug, Clone)]
struct HeightTree {
    tree: HashMap<Coord, HashSet<Coord>>,
}

impl HeightTree {
    fn new() -> Self {
        HeightTree {
            tree: HashMap::new(),
        }
    }
    fn from_height_map(height_map: &HeightMap) -> Self {
        let mut tree = HeightTree::new();
        for (current_coord, current_height) in height_map.heights.iter() {
            log::debug!(
                "Adding children for coord {} with value {}",
                current_coord,
                current_height
            );

            // Skip if End node.
            if let Height::End(_) = current_height {
                log::debug!("Skipping End node.");
                continue;
            }

            // Compare current coord's height against neighbors' heights.
            let mut neighbors = HashSet::new();
            for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                let new_r = current_coord.r as isize + dir.0;
                let new_c = current_coord.c as isize + dir.1;
                if (new_r < 0) | (new_c < 0) {
                    continue;
                }
                let neighbor_coord = Coord {
                    r: new_r as usize,
                    c: new_c as usize,
                };
                match height_map.heights.get(&neighbor_coord) {
                    Some(Height::End(_)) | None => (),
                    Some(Height::Start(x)) | Some(Height::H(x)) => {
                        if x <= &(current_height.get_height() + 1) {
                            neighbors.insert(neighbor_coord);
                        }
                    }
                }
            }
            log::debug!("  children: {:?}", neighbors);
            tree.tree.insert(*current_coord, neighbors);
        }
        tree
    }
}

impl std::fmt::Display for HeightTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "----- HEIGHT TREE -----")?;
        for (node, children) in self.tree.iter() {
            let children_str: String = children
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(f, "{}: {}", node, children_str)?
        }
        write!(f, "--------------------")
    }
}

pub fn puzzle_1(input_data: &str) -> Result<usize, PuzzleError> {
    log::info!("Building Height Map..");
    let height_map = parse_input(input_data)?;
    log::info!("Building Height Tree.");
    let height_tree = HeightTree::from_height_map(&height_map);
    log::info!("Finished building Tree.");
    log::debug!("{}", height_tree);
    Ok(0)
}

pub fn main(data_dir: &str) {
    println!("Day 12: Hill Climbing Algorithm");
    let data = load_raw(data_dir, 12, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match &answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("Error on Puzzle 1: {}", e),
    }
    assert_eq!(answer_1, Ok(447));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data, 10000);
    // match &answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("Error on Puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(29703395016));
}

#[cfg(test)]
mod tests {
    use crate::solutions::day12::puzzle_1;

    const EXAMPLE_1: &str = "
    Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi
    ";

    #[test]
    fn puzzle_1_example() {
        env_logger::init();
        let res = puzzle_1(EXAMPLE_1);
        assert_eq!(res, Ok(31));
    }
}
