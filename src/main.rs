use core::str;
use log::info;
use std::str::Lines;

use clap::Parser;

#[derive(Parser)]
struct Args {
    problem: String,
    path: std::path::PathBuf,
}

#[derive(Debug)]
enum Rotation {
    Right(i32),
    Left(i32),
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    info!("problem: {:?}, path: {:?}", args.problem, args.path);

    let input =
        std::fs::read_to_string(args.path).expect("Expected file at path to contain a string");

    let lines = input.lines();

    let answer: i32 = match args.problem.as_str() {
        "dayOne" => day_one(lines),
        _ => panic!("problem solution not found"),
    };

    println!("Answer to {} is {}", args.problem, answer);
}

fn day_one(lines: Lines) -> i32 {
    let mut position: i32 = 50;
    let mut count: i32 = 0;

    let instructions = lines.map(|l| get_instruction_from_line(l));

    info!("Position starts at {}", position);

    for instruction in instructions {
        let steps = match instruction {
            Rotation::Right(s) => s,
            Rotation::Left(s) => -s,
        };

        let start = position;
        let end = (position + steps).rem_euclid(100);

        let abs_steps = steps.abs();
        let dist_to_first_zero = match (steps, start) {
            (_, 0) => 100,
            (steps, start) if steps > 0 => 100 - start,
            (steps, start) if steps < 0 => start,
            _ => 0,
        };

        let zero_crossings = if abs_steps >= dist_to_first_zero {
            1 + ((abs_steps - dist_to_first_zero) / 100)
        } else {
            0
        };

        count += zero_crossings;
        position = end;

        info!(
            "steps {}, start {}, end {}, zero_crossings {}, total count {}",
            steps, start, position, zero_crossings, count
        )
    }

    count
}

fn get_instruction_from_line(line: &str) -> Rotation {
    info!("instruction {}", line);
    let mut chars = line.chars();

    let head = chars.nth(0);
    let tail: String = chars.collect();

    let parsed_tail: i32 = tail.parse().expect("could not parse step as i32");

    match head {
        Some('R') => Rotation::Right(parsed_tail),
        Some('L') => Rotation::Left(parsed_tail),
        _ => panic!("Invalid rotation encountered"),
    }
}
