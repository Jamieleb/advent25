use log::info;

use clap::Parser;

mod day_five;
mod day_four;
mod day_one;
mod day_three;
mod day_two;

#[derive(Parser)]
struct Args {
    problem: String,
    path: std::path::PathBuf,
    #[arg(short = 'b', long, default_value = "2")]
    num_batteries: usize,
    #[arg(short = 'p', long, default_value = "1")]
    part: usize,
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
        "dayThree" => day_three::solution(input, args.num_batteries),
        "dayFour" => day_four::solution(input),
        "dayFive" => day_five::solution(input, args.part),
        _ => panic!("problem solution not found"),
    };

    println!("Answer to {} is {}", args.problem, answer);
}
