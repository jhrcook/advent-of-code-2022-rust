use crate::data::load_raw;
use petgraph::algo::k_shortest_path;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;
use std::hash::Hash;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("Failed parsing integer.")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Unkown height: {}", .0)]
    UnknownHeight(String),
    #[error("No start coordinate.")]
    NoStartCoord,
    #[error("No end coordinate.")]
    NoEndCoord,
    #[error("No paths found.")]
    NoPathsFound,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    fn add_value(&mut self, row: usize, col: usize, value: Height) {
        log::debug!("Adding value: [{},{}] -> {}", row, col, value);
        self.heights.insert(Coord { r: row, c: col }, value);
    }
}

struct HeightTranslator {
    score_map: HashMap<char, usize>,
}

impl HeightTranslator {
    fn new() -> Self {
        let mut score_map = HashMap::new();
        for (i, c) in ('a'..='z').enumerate() {
            score_map.insert(c, i);
        }
        Self { score_map }
    }

    fn translate(&self, input: &char) -> Result<Height, PuzzleError> {
        match input {
            'S' => Ok(Height::Start(
                self.score_map
                    .get(&'a')
                    .cloned()
                    .ok_or(PuzzleError::UnknownHeight(input.to_string()))?,
            )),
            'E' => Ok(Height::End(
                self.score_map
                    .get(&'z')
                    .cloned()
                    .ok_or(PuzzleError::UnknownHeight(input.to_string()))?,
            )),
            x => {
                let val = self
                    .score_map
                    .get(x)
                    .cloned()
                    .ok_or(PuzzleError::UnknownHeight(input.to_string()))?;
                Ok(Height::H(val))
            }
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Position {
    height: Height,
    coord: Coord,
}

struct Nodes {
    nodes: HashMap<Position, NodeIndex>,
}

impl Nodes {
    fn new() -> Self {
        Nodes {
            nodes: HashMap::new(),
        }
    }

    fn get(&mut self, pos: &Position, tree: &mut DiGraph<Position, &str>) -> NodeIndex {
        match self.nodes.get(pos) {
            Some(i) => *i,
            None => {
                let i = tree.add_node(pos.clone());
                self.nodes.insert(pos.clone(), i);
                i
            }
        }
    }
}

#[derive(Debug, Clone)]
struct HeightTree<'a> {
    tree: DiGraph<Position, &'a str>,
    start: NodeIndex,
    end: NodeIndex,
}

impl<'a> HeightTree<'a> {
    fn from_height_map(height_map: &HeightMap) -> Result<Self, PuzzleError> {
        let mut tree = DiGraph::new();
        let mut start: Option<NodeIndex> = Option::None;
        let mut end: Option<NodeIndex> = Option::None;
        let mut nodes = Nodes::new();

        for (coord, height) in height_map.heights.iter() {
            let p = Position {
                height: *height,
                coord: *coord,
            };
            let node_idx = nodes.get(&p, &mut tree);

            // Record Start and End nodes.
            match height {
                Height::Start(_) => start = Some(node_idx),
                Height::End(_) => {
                    end = Some(node_idx);
                    continue;
                }
                _ => (),
            }

            for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                let new_r = coord.r as isize + dir.0;
                let new_c = coord.c as isize + dir.1;
                if (new_r < 0) | (new_c < 0) {
                    continue;
                }
                let neighbor_coord = Coord {
                    r: new_r as usize,
                    c: new_c as usize,
                };
                if let Some(neighbor_height) = height_map.heights.get(&neighbor_coord) {
                    if neighbor_height.get_height() <= (height.get_height() + 1) {
                        let neighbor_pos = Position {
                            height: *neighbor_height,
                            coord: neighbor_coord,
                        };
                        let neighbor_idx = nodes.get(&neighbor_pos, &mut tree);
                        tree.add_edge(node_idx, neighbor_idx, "");
                    }
                }
            }
        }
        Ok(HeightTree {
            tree,
            start: start.ok_or(PuzzleError::NoStartCoord)?,
            end: end.ok_or(PuzzleError::NoEndCoord)?,
        })
    }

    fn shortest_distance(&self) -> Result<usize, PuzzleError> {
        let search_results = k_shortest_path(&self.tree, self.start, Some(self.end), 1, |_| 1);
        match search_results.get(&self.end) {
            Some(dist) => Ok(*dist),
            None => Err(PuzzleError::NoPathsFound),
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

pub fn puzzle_1(input_data: &str) -> Result<usize, PuzzleError> {
    let height_tree = HeightTree::from_height_map(&parse_input(input_data)?)?;
    height_tree.shortest_distance()
}

pub fn puzzle_2(input_data: &str) -> Result<usize, PuzzleError> {
    let mut height_tree = HeightTree::from_height_map(&parse_input(input_data)?)?;
    height_tree.tree.reverse(); // Reverse and go from E to all nodes with height "a".
    height_tree
        .tree
        .node_indices()
        .filter(|i| height_tree.tree[*i].height.get_height() == 0)
        .filter_map(|i| {
            k_shortest_path(&height_tree.tree, height_tree.end, Some(i), 1, |_| 1)
                .get(&i)
                .cloned()
        })
        .min()
        .ok_or(PuzzleError::NoPathsFound)
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
    let answer_2 = puzzle_2(&data);
    match &answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("Error on Puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(446));
}

#[cfg(test)]
mod tests {
    use crate::solutions::day12::{puzzle_1, puzzle_2};

    const EXAMPLE_1: &str = "
    Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi
    ";

    #[test]
    fn puzzle_1_example_1() {
        env_logger::init();
        let res = puzzle_1(EXAMPLE_1);
        assert_eq!(res, Ok(31));
    }

    #[test]
    fn puzzle_2_example_1() {
        env_logger::init();
        let res = puzzle_2(EXAMPLE_1);
        assert_eq!(res, Ok(29));
    }
}
