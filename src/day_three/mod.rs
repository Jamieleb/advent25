use log::debug;

pub fn solution(input: String, num_batteries: usize) -> u64 {
    let lines = input.lines();

    lines
        .map(|l| get_large_jolts_for_line(l, num_batteries))
        .sum()
}

fn get_large_jolts_for_line(line: &str, num_batteries: usize) -> u64 {
    debug!("line: {}, length: {}", line, line.len());
    let mut total: u64 = 0;
    let mut window_start = 0;

    let digits: Vec<u64> = line
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    for exponent in (0..=num_batteries - 1).rev() {
        let mut max: u64 = 0;
        let mut max_idx = 0;

        let window = &digits[window_start..(digits.len() - exponent)];

        for (i, digit) in window.iter().enumerate() {
            if *digit > max {
                max = *digit;
                max_idx = i;
            }
        }

        window_start += max_idx + 1;

        total += max * 10_u64.pow(exponent as u32);
    }

    debug!("---------");
    debug!("line {:?} has total {}", line, total);
    debug!("---------");

    total
}
