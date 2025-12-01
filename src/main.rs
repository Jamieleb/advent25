use core::str;
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
    let args = Args::parse();

    println!("problem: {:?}, path: {:?}", args.problem, args.path);

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

    println!("Position starts at {}", position);

    for instruction in instructions {
        let steps = match instruction {
            Ok(Rotation::Right(s)) => s,
            Ok(Rotation::Left(s)) => -s,
            Err(_) => panic!(),
        };

        let start = position;
        let end = (position + steps).rem_euclid(100);

        let zero_crossings = if steps > 0 {
            let dist_to_first_zero = if start == 0 { 100 } else { 100 - start };

            if steps >= dist_to_first_zero {
                1 + ((steps - dist_to_first_zero) / 100)
            } else {
                0
            }
        } else if steps < 0 {
            let abs_steps = steps.abs();
            let dist_to_first_zero = if start == 0 { 100 } else { start };

            if abs_steps >= dist_to_first_zero {
                1 + ((abs_steps - dist_to_first_zero) / 100)
            } else {
                0
            }
        } else {
            0
        };

        count += zero_crossings;
        position = end;

        println!(
            "steps {}, start {}, end {}, zero_crossings {}, total count {}",
            steps, start, position, zero_crossings, count
        )
    }

    count
}

fn get_instruction_from_line(line: &str) -> Result<Rotation, &str> {
    println!("instruction {}", line);
    let mut chars = line.chars();

    let head = chars.nth(0);
    let tail: String = chars.collect();

    let parsed_tail: i32 = tail.parse().expect("could not parse step as i32");

    match head {
        Some('R') => Ok(Rotation::Right(parsed_tail)),
        Some('L') => Ok(Rotation::Left(parsed_tail)),
        _ => Err("Invalid direction"),
    }
}
