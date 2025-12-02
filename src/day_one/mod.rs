use log::info;

#[derive(Debug)]
enum Rotation {
    Right(i32),
    Left(i32),
}

pub fn solution(input: String) -> u64 {
    let mut position: i32 = 50;
    let mut count: i32 = 0;

    let instructions = input.lines().map(|l| get_instruction_from_line(l));

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

    count.try_into().unwrap()
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
