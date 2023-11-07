use advent_of_code_2022_rust::run_all;
use clap::Parser;
use std::time::Instant;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Data directory.
    #[arg(short, long, default_value_t = String::from("puzzle-input"))]
    data_dir: String,
}

fn main() {
    println!("Running all solutions.");
    let args = Args::parse();
    let start = Instant::now();
    run_all(&args.data_dir);
    let duration = start.elapsed();
    print!("Done! 🎉");
    println!(" -- Elapsed time: {:?} s", duration.as_secs());
}
