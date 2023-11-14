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
}
