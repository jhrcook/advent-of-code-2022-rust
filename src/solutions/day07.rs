use crate::data::load_raw;
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("Unexpected command.")]
    UnexpectedCommand(String),
    #[error("Adding edge from non-existent node.")]
    AddingLinkFromNonexistentNode(String, String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
struct Graph {
    node_list: HashSet<Node>,
    map: HashMap<Node, HashSet<Node>>,
    parents: HashMap<Node, Node>,
}

impl Graph {
    fn new(root: &str) -> Self {
        let mut graph = Graph {
            node_list: HashSet::new(),
            map: HashMap::new(),
            parents: HashMap::new(),
        };
        let root_node = Node {
            name: root.to_string(),
            size: 0,
        };
        graph.node_list.insert(root_node.clone());
        graph.map.insert(root_node, HashSet::new());
        graph
    }
}

impl Graph {
    fn contains_node(self, node: Node) -> bool {
        self.node_list.contains(&node)
    }

    fn add_edge(mut self, from: Node, to: Node) -> Result<(), PuzzleError> {
        // Add link: from -> to.
        self.map
            .get_mut(&from)
            .ok_or(PuzzleError::AddingLinkFromNonexistentNode(
                from.name.clone(),
                to.name.clone(),
            ))?
            .insert(to.clone());
        // Add parent: to -> from.
        self.parents.insert(to.clone(), from);
        // Add new node to list.
        self.node_list.insert(to);
        Ok(())
    }
}

fn build_filesystem_tree(input_data: &str) -> Result<Graph, PuzzleError> {
    let mut graph = Graph::new("/");
    for line in input_data.lines().map(|x| x.trim()) {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("$ cd") {
            println!("cd command");
        } else if line.starts_with("$ ls") {
            println!("ls command");
        } else if line.starts_with("dir") {
            println!("sub-dir");
        } else if line.chars().next().unwrap().is_numeric() {
            println!("file");
        } else {
            return Err(PuzzleError::UnexpectedCommand(line.to_string()));
        }
    }
    Ok(graph)
}

pub fn puzzle_1(input_data: &str) -> Result<usize, PuzzleError> {
    let filesystem = build_filesystem_tree(input_data.trim());
    Ok(0)
}

pub fn main(data_dir: &str) {
    println!("Day 7: No Space Left On Device");
    let data = load_raw(data_dir, 7, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match &answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("Error on Puzzle 1: {}", e),
    }
    assert_eq!(answer_1, Ok(1210));

    // Puzzle 2.
    // let answer_2 = puzzle_2(&data);
    // match &answer_2 {
    //     Ok(x) => println!(" Puzzle 2: {}", x),
    //     Err(e) => panic!("Error on Puzzle 2: {}", e),
    // }
    // assert_eq!(answer_2, Ok(3476));
}

#[cfg(test)]
mod tests {
    use crate::solutions::day07::puzzle_1;

    const EXAMPLE_1: &str = "
    $ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k";

    #[test]
    fn puzzle_1_examples() {
        assert_eq!(puzzle_1(EXAMPLE_1), Ok(7));
    }

    #[test]
    fn puzzle_2_examples() {
        // assert_eq!(puzzle_2(EXAMPLE_1), Ok(19));
    }
}
