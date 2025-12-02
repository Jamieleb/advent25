use std::vec;

use log::debug;

pub fn solution(input: String) -> u64 {
    let ranges = input.trim().split(",");

    let invalid_ids = ranges.flat_map(get_invalid_ids_in_range);

    invalid_ids.sum()
}

fn get_invalid_ids_in_range(range: &str) -> Vec<u64> {
    let (start, finish) = range.split_once('-').unwrap();
    debug!("start: {}, finish: {}", start, finish);
    let start_num: u64 = start
        .parse()
        .expect("found range start that coulnt not be parsed as u64");
    let finish_num: u64 = finish
        .parse()
        .expect("found range finish that could not be parse as u64");

    let mut invalid_ids: Vec<u64> = vec![];
    for num in start_num..=finish_num {
        let num_str = num.to_string();

        let length = num_str.len();

        for i in 1..length / 2 + 1 {
            let pattern = &num_str[..i];
            let mut chunks = num_str.as_bytes().chunks(pattern.len());

            if chunks.all(|c| std::str::from_utf8(c).unwrap() == pattern) {
                invalid_ids.push(num);
                break;
            }
        }
    }

    invalid_ids
}
