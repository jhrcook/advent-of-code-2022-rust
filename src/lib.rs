mod data;
mod solutions;

pub fn run_all(data_dir: &str) {
    solutions::day01::main(data_dir);
    solutions::day02::main(data_dir);
    solutions::day03::main(data_dir);
    solutions::day04::main(data_dir);
    solutions::day05::main(data_dir);
    solutions::day06::main(data_dir);
    solutions::day07::main(data_dir);
    solutions::day08::main(data_dir);
    solutions::day09::main(data_dir);
    solutions::day10::main(data_dir);
}

pub fn run_day(data_dir: &str, day: &usize) {
    match day {
        1 => solutions::day01::main(data_dir),
        2 => solutions::day02::main(data_dir),
        3 => solutions::day03::main(data_dir),
        4 => solutions::day04::main(data_dir),
        5 => solutions::day05::main(data_dir),
        6 => solutions::day06::main(data_dir),
        7 => solutions::day07::main(data_dir),
        8 => solutions::day08::main(data_dir),
        9 => solutions::day09::main(data_dir),
        10 => solutions::day10::main(data_dir),
        _ => panic!("Puzzle for day {} not completed yet.", day),
    }
}
