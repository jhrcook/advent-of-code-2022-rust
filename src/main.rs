use advent_of_code_2022_rust::run_all;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Data directory.
    #[arg(short, long)]
    data_dir: String,
}

fn main() {
    println!("Running all solutions.");
    let args = Args::parse();
    run_all(&args.data_dir);
    println!("Done! 🎉")
}
