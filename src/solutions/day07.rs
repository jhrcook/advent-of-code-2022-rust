use crate::data::load_raw;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    // #[error("Unexpected command.")]
    // UnexpectedCommand(String),
    // #[error("Adding edge from non-existent node.")]
    // AddingLinkFromNonexistentNode(String, String),
    // #[error("Logic error trying to add a child without a current parent node.")]
    // AddingChildToNoneNode,
    #[error("Node does not have a parent.")]
    NoParentNode(String),
    #[error("Cannot parse file size.")]
    ParsingFileSize(String),
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
    fn new(root: &Node) -> Self {
        let mut graph = Graph {
            node_list: HashSet::new(),
            map: HashMap::new(),
            parents: HashMap::new(),
        };
        graph.node_list.insert(root.clone());
        graph.map.insert(root.clone(), HashSet::new());
        graph
    }
}

impl Graph {
    fn contains_node(&self, node: Node) -> bool {
        self.node_list.contains(&node)
    }

    fn add_node(&mut self, node: &Node) {
        self.node_list.insert(node.clone());
    }

    fn add_edge(&mut self, from: &Node, to: &Node) -> Result<(), PuzzleError> {
        // Add link: from -> to.
        if !self.map.contains_key(from) {
            self.map.insert(from.clone(), HashSet::new());
        }
        self.map.get_mut(from).unwrap().insert(to.clone());
        // Add parent: to -> from.
        self.parents.insert(to.clone(), from.clone());
        // Add new node to list.
        self.node_list.insert(to.clone());
        Ok(())
    }

    fn get_parent(&self, of: &Node) -> Result<Node, PuzzleError> {
        Ok(self
            .parents
            .get(of)
            .ok_or(PuzzleError::NoParentNode(of.name.clone()))
            .unwrap()
            .clone())
    }

    fn sum_subtree_size(&self, node: &Node) -> usize {
        let subtree_size: usize = match self.map.get(node) {
            Some(children) => children.iter().map(|n| self.sum_subtree_size(n)).sum(),
            None => 0,
        };
        node.size + subtree_size
    }
}

fn build_filesystem_tree(input_data: &str) -> Result<Graph, PuzzleError> {
    let root_node = Node {
        name: "/".to_string(),
        size: 0,
    };
    let mut graph = Graph::new(&root_node);
    let mut cwd: Node = root_node.clone();
    for line in input_data.lines().map(|x| x.trim()) {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("$ cd") {
            let node_name = line.split(' ').collect::<Vec<_>>()[2];
            if node_name == ".." {
                cwd = graph.get_parent(&cwd).unwrap();
            } else {
                cwd = Node {
                    name: node_name.to_string(),
                    size: 0,
                };
            }
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("dir") {
            let dir_name = line.split(' ').collect::<Vec<_>>()[1];
            let dir_node = Node {
                name: dir_name.to_string(),
                size: 0,
            };
            graph.add_edge(&cwd, &dir_node).unwrap();
        } else {
            let split_line = line.split(' ').collect::<Vec<_>>();
            let file_size: usize = match split_line[0].parse() {
                Ok(x) => Ok(x),
                Err(_) => Err(PuzzleError::ParsingFileSize(line.to_string())),
            }?;
            let file_name = line.split(' ').collect::<Vec<_>>()[1];
            let file_node = Node {
                name: file_name.to_string(),
                size: file_size,
            };
            graph.add_edge(&cwd, &file_node).unwrap();
        }
    }
    Ok(graph)
}

pub fn puzzle_1(input_data: &str) -> Result<usize, PuzzleError> {
    let filesystem = build_filesystem_tree(input_data)?;
    let mut counter = 0;
    for node in filesystem.node_list.iter().filter(|n| n.size == 0) {
        let du = filesystem.sum_subtree_size(node);
        if du <= 100000 {
            counter += du;
        }
    }
    println!("{:?}", filesystem);
    Ok(counter)
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
    assert_eq!(answer_1, Ok(1334506));

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
        assert_eq!(puzzle_1(EXAMPLE_1), Ok(95437));
    }

    #[test]
    fn puzzle_2_examples() {
        // assert_eq!(puzzle_2(EXAMPLE_1), Ok(19));
    }
}
