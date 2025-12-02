use log::info;

use clap::Parser;

mod day_one;
mod day_two;

#[derive(Parser)]
struct Args {
    problem: String,
    path: std::path::PathBuf,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    info!("problem: {:?}, path: {:?}", args.problem, args.path);

    let input =
        std::fs::read_to_string(args.path).expect("Expected file at path to contain a string");

    let answer: u64 = match args.problem.as_str() {
        "dayOne" => day_one::solution(input),
        "dayTwo" => day_two::solution(input),
        _ => panic!("problem solution not found"),
    };

    println!("Answer to {} is {}", args.problem, answer);
}
