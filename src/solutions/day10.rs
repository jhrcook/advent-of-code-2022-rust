use crate::data::load_raw;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PuzzleError {
    #[error("Iterating over line returned `None`.")]
    EmptyInputDataLine,
    #[error("Unexpected operation in input line: {}", .0)]
    UnexpectedOperation(String),
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Noop,
    Addx(isize),
}

#[derive(Debug, Clone, Copy)]
pub struct Cpu {
    x: isize,
    cycles_complete: isize,
    total_signal_strength: isize,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            x: 1,
            // Specifically, the number of cycles *completed*. Therefore, a value of 1
            // indicates that 1 cycle has been completed and it is currently in cycle 2.
            cycles_complete: 0,
            total_signal_strength: 0,
        }
    }

    /// The current execution cycle number.
    fn current_cycle(&self) -> isize {
        self.cycles_complete + 1
    }

    fn update_signal_strength(&mut self) {
        match self.current_cycle() {
            20 | 60 | 100 | 140 | 180 | 220 => {
                self.total_signal_strength += (self.current_cycle()) * self.x
            }
            _ => (),
        }
    }

    fn compute_cycle(&mut self, add_x: isize) {
        self.update_signal_strength();
        self.cycles_complete += 1;
        self.x += add_x;
    }

    fn perform(&mut self, op: &Operation, crt: Option<&mut Crt>) {
        match op {
            Operation::Noop => {
                if let Some(_crt) = crt {
                    _crt.update(self);
                }
                self.compute_cycle(0)
            }
            Operation::Addx(x) => {
                let _cpu_capture = *self;
                self.compute_cycle(0);
                if let Some(_crt) = crt {
                    _crt.update(&_cpu_capture);
                    _crt.update(self);
                }
                self.compute_cycle(*x);
            }
        };
    }
}

#[derive(Debug, Clone)]
struct Crt {
    dims: (usize, usize), // width x height
    pixels: Vec<Vec<char>>,
}

impl Crt {
    fn new() -> Self {
        let dims = (40, 6);
        let mut pixels = Vec::new();
        for _ in 0..dims.1 {
            pixels.push((0..dims.0).map(|_| '.').collect());
        }
        Crt { dims, pixels }
    }

    fn _print(&self) {
        let vbreak = (0..self.dims.0).map(|_| "-").collect::<Vec<_>>().join("");
        println!("{}", vbreak);
        for row in self.pixels.iter() {
            for c in row {
                print!("{}", c);
            }
            println!()
        }
        println!("{}", vbreak);
    }

    fn display_as_string(&self) -> String {
        self.pixels
            .iter()
            .map(|r| {
                r.clone()
                    .iter()
                    .map(|x| String::from(*x))
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn update(&mut self, cpu: &Cpu) {
        let pixel_col = cpu.cycles_complete % self.dims.0 as isize;
        if pixel_col < -1 {
            return;
        }
        let pixel_row = (cpu.cycles_complete as f32 / self.dims.0 as f32).floor() as isize;
        for sprite in (cpu.x - 1)..=(cpu.x + 1) {
            if sprite == pixel_col {
                let mut row = self.pixels[pixel_row as usize].clone();
                row[pixel_col as usize] = '#';
                self.pixels[pixel_row as usize] = row;
                return;
            }
        }
    }
}

pub fn parse_input(input_data: &str) -> Result<Vec<Operation>, PuzzleError> {
    let mut operations = Vec::new();
    for line in input_data.trim().lines() {
        let pieces: Vec<&str> = line.trim().split(' ').collect();
        match pieces.first() {
            Some(&"noop") => operations.push(Operation::Noop),
            Some(&"addx") => operations.push(Operation::Addx(
                pieces[1].to_string().parse::<isize>().unwrap(),
            )),
            Some(_) => return Err(PuzzleError::UnexpectedOperation(line.to_string())),
            None => return Err(PuzzleError::EmptyInputDataLine),
        };
    }
    Ok(operations)
}

pub fn puzzle_1(input_data: &str) -> Result<isize, PuzzleError> {
    let operations = parse_input(input_data)?;
    let mut cpu = Cpu::new();
    for op in operations {
        cpu.perform(&op, None);
    }
    Ok(cpu.total_signal_strength)
}

pub fn puzzle_2(input_data: &str) -> Result<String, PuzzleError> {
    let operations = parse_input(input_data)?;
    let mut cpu = Cpu::new();
    let mut crt = Crt::new();
    for op in operations {
        cpu.perform(&op, Some(&mut crt));
    }
    Ok(crt.display_as_string())
}

pub fn main(data_dir: &str) {
    println!("Day 10: Cathode-Ray Tube");
    let data = load_raw(data_dir, 10, None);

    // Puzzle 1.
    let answer_1 = puzzle_1(&data);
    match &answer_1 {
        Ok(x) => println!(" Puzzle 1: {}", x),
        Err(e) => panic!("Error on Puzzle 1: {}", e),
    }
    assert_eq!(answer_1, Ok(15220));

    // Puzzle 2.
    let answer_2 = puzzle_2(&data);
    match &answer_2 {
        Ok(x) => println!(" Puzzle 2: \n{}", x),
        Err(e) => panic!("Error on Puzzle 2: {}", e),
    }
    assert_eq!(
        answer_2,
        Ok("###..####.####.####.#..#.###..####..##..
#..#.#.......#.#....#.#..#..#.#....#..#.
#..#.###....#..###..##...###..###..#..#.
###..#.....#...#....#.#..#..#.#....####.
#.#..#....#....#....#.#..#..#.#....#..#.
#..#.#....####.####.#..#.###..#....#..#."
            .to_string())
    );
}

#[cfg(test)]
mod tests {
    use crate::data::load_raw;
    use crate::solutions::day10::{parse_input, puzzle_1, puzzle_2, Cpu};

    const EXAMPLE_1: &str = "
    noop
    addx 3
    addx -5
    ";

    #[test]
    fn puzzle_1_example_1() {
        // Test parsing of input operations.
        let example_operations = parse_input(EXAMPLE_1);
        assert!(example_operations.is_ok());
        assert_eq!(example_operations.unwrap().len(), 3);

        // Test performing operations.
        let example_operations = parse_input(EXAMPLE_1).unwrap();
        let mut cpu = Cpu::new();
        for op in example_operations.iter() {
            cpu.perform(op, None);
        }
        assert_eq!(cpu.x, -1);
        assert_eq!(cpu.cycles_complete, 5);
        assert_eq!(cpu.total_signal_strength, 0);

        assert_eq!(puzzle_1(EXAMPLE_1), Ok(0));
    }

    #[test]
    fn puzzle_1_example_2() {
        let data = load_raw("puzzle-input", 10, Some("_ex1"));
        assert_eq!(puzzle_1(&data), Ok(13140));
    }

    #[test]
    fn puzzle_2_examples() {
        let data = load_raw("puzzle-input", 10, Some("_ex1"));
        let res = puzzle_2(&data);
        assert_eq!(
            res,
            Ok("##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .to_string())
        )
    }
}
