use crate::data::load_raw;
use std::collections::{HashMap, HashSet};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("No parent.")]
    NoParentNode(String),
    #[error("No child.")]
    NoChildNode(String),
    #[error("Failed to parse file size.")]
    ParsingFileSize(String),
    #[error("No minimum size that meets constrains.")]
    NoMinimumValue,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Node {
    uuid: Uuid,
    name: String,
    size: usize,
}

impl Node {
    fn new(name: &str, size: usize) -> Self {
        Node {
            uuid: Uuid::new_v4(),
            name: name.to_string(),
            size,
        }
    }
}

struct Tree {
    root: Node,
    nodes: HashMap<Uuid, Node>,
    edges: HashMap<Node, HashSet<Node>>,
    parents: HashMap<Node, Node>,
}

impl Tree {
    fn new() -> Self {
        let root = Node::new("/", 0);
        let mut nodes = HashMap::new();
        nodes.insert(root.uuid, root.clone());
        let mut edges = HashMap::new();
        edges.insert(root.clone(), HashSet::new());

        Tree {
            root,
            nodes,
            edges,
            parents: HashMap::new(),
        }
    }

    fn get_parent(&self, node: &Node) -> Result<Node, PuzzleError> {
        match self.parents.get(node) {
            Some(n) => Ok(n.clone()),
            None => Err(PuzzleError::NoParentNode(node.name.clone())),
        }
    }

    fn get_child(&self, node: &Node, child_name: &str) -> Result<Node, PuzzleError> {
        for child_node in self
            .edges
            .get(node)
            .ok_or(PuzzleError::NoChildNode(node.name.clone()))?
        {
            if child_node.name == child_name {
                return Ok(child_node.clone());
            }
        }
        Err(PuzzleError::NoChildNode(node.name.clone()))
    }

    fn add_child(&mut self, parent: &Node, name: &str, size: usize) -> Result<(), PuzzleError> {
        // If node with name already in children set, return that node.
        let children_nodes = self
            .edges
            .get(parent)
            .ok_or(PuzzleError::NoParentNode(parent.name.clone()))?;
        for child_node in children_nodes.iter() {
            if child_node.name == name {
                return Ok(());
            }
        }

        // Make new node and add to `nodes``, `edges`, and `parents` collections.
        let new_node = Node::new(name, size);
        self.nodes.insert(new_node.uuid, new_node.clone());
        self.edges
            .get_mut(parent)
            .ok_or(PuzzleError::NoParentNode(parent.name.clone()))?
            .insert(new_node.clone());
        self.edges.insert(new_node.clone(), HashSet::new());
        self.parents.insert(new_node, parent.clone());
        Ok(())
    }
}

impl Tree {
    fn directory_nodes(&self) -> HashSet<Node> {
        self.nodes
            .values()
            .filter(|n| n.size == 0)
            .cloned()
            .collect::<HashSet<_>>()
    }
    fn calculate_size(&self, node: &Node, node_sizes: &mut HashMap<Node, usize>) -> usize {
        if let Some(s) = node_sizes.get(node) {
            return *s;
        };
        let mut size = node.size;
        size += match self.edges.get(node) {
            Some(children) => children
                .iter()
                .map(|n| self.calculate_size(n, node_sizes))
                .sum(),
            None => 0,
        };
        node_sizes.insert(node.clone(), size);
        size
    }

    fn calculate_sizes(&self) -> HashMap<Node, usize> {
        let mut sizes = HashMap::new();
        let _ = self
            .nodes
            .values()
            // .iter()
            .map(|x| self.calculate_size(x, &mut sizes))
            .collect::<Vec<usize>>();
        sizes
    }
}

fn build_filesystem_tree(input_data: &str) -> Result<Tree, PuzzleError> {
    let mut fs: Tree = Tree::new();
    let mut cwd = fs.root.clone();
    for line in input_data.trim().lines().skip(1).map(|x| x.trim()) {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("$ cd") {
            let node_name = line.split(' ').collect::<Vec<_>>()[2];
            if node_name == ".." {
                cwd = fs.get_parent(&cwd)?;
            } else {
                cwd = fs.get_child(&cwd, node_name)?;
            }
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("dir") {
            let dir_name = line.split(' ').collect::<Vec<_>>()[1];
            fs.add_child(&cwd, dir_name, 0)?;
        } else {
            // Is a file.
            let split_line = line.split(' ').collect::<Vec<_>>();
            let file_size: usize = match split_line[0].parse() {
                Ok(x) => Ok(x),
                Err(_) => Err(PuzzleError::ParsingFileSize(line.to_string())),
            }?;
            let file_name = line.split(' ').collect::<Vec<_>>()[1];
            fs.add_child(&cwd, file_name, file_size)?;
        }
    }
    Ok(fs)
}

pub fn puzzle_1(input_data: &str) -> Result<usize, PuzzleError> {
    let fs = build_filesystem_tree(input_data)?;
    let dir_nodes = fs.directory_nodes();
    let size = fs
        .calculate_sizes()
        .iter()
        .filter(|(n, s)| dir_nodes.contains(n) & (s <= &&100000))
        .map(|(_, s)| s)
        .sum();
    Ok(size)
}

pub fn puzzle_2(input_data: &str) -> Result<usize, PuzzleError> {
    let fs = build_filesystem_tree(input_data)?;
    let dir_nodes = fs.directory_nodes();
    let sizes = fs.calculate_sizes();

    let device_size = 70000000;
    let space_required = 30000000;
    let space_used = sizes.get(&fs.root).unwrap();
    let min_deletion_size = space_required - (device_size - space_used);

    let deletion_size = sizes
        .iter()
        .filter(|(n, s)| dir_nodes.contains(n) & (s >= &&min_deletion_size))
        .map(|(_, s)| s)
        .min()
        .ok_or(PuzzleError::NoMinimumValue)
        .unwrap();
    Ok(*deletion_size)
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
    let answer_2 = puzzle_2(&data);
    match &answer_2 {
        Ok(x) => println!(" Puzzle 2: {}", x),
        Err(e) => panic!("Error on Puzzle 2: {}", e),
    }
    assert_eq!(answer_2, Ok(7421137));
}

#[cfg(test)]
mod tests {
    use crate::solutions::day07::{puzzle_1, puzzle_2};

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
        assert_eq!(puzzle_2(EXAMPLE_1), Ok(24933642));
    }
}
