use std::ops::RangeInclusive;

use log::debug;

pub fn solution(input: String, part: usize) -> u64 {
    let (ranges, ids) = extract_ranges_and_ids(&input).unwrap();

    let answer = match part {
        1 => {
            let mut count: u64 = 0;
            for id in ids {
                if ranges.iter().any(|r| r.contains(&id)) {
                    count += 1;
                }
            }
            count
        }
        2 => count_unique_ids_in_ranges(ranges),
        _ => panic!("invalid problem part"),
    };

    answer
}

fn extract_ranges_and_ids(input: &str) -> Option<(Vec<RangeInclusive<u64>>, Vec<u64>)> {
    let lines: Vec<&str> = input.lines().collect();
    if let Some(empty_line_idx) = lines.iter().position(|l| l.is_empty()) {
        let range_strs = &lines[..empty_line_idx];
        let ids_strs = &lines[empty_line_idx + 1..];

        debug!("ranges: {:?}, ids: {:?}", range_strs, ids_strs);

        let ids: Vec<u64> = ids_strs
            .iter()
            .map(|s| s.trim().parse().expect("failed to unwrap id"))
            .collect();

        Some((parse_ranges(range_strs), ids))
    } else {
        None
    }
}

fn parse_ranges(range_strs: &[&str]) -> Vec<RangeInclusive<u64>> {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    for range_str in range_strs {
        let (start, end) = range_str.trim().split_once('-').unwrap();
        let range: RangeInclusive<u64> = start.parse().unwrap()..=end.parse().unwrap();
        ranges.push(range);
    }

    ranges
}

fn count_unique_ids_in_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> u64 {
    if ranges.is_empty() {
        return 0;
    }

    // Sort ranges by start position
    ranges.sort_by_key(|r| *r.start());

    let mut merged: Vec<RangeInclusive<u64>> = Vec::new();
    let mut current = ranges[0].clone();

    for range in ranges.into_iter().skip(1) {
        // Check if ranges overlap or are adjacent
        if *range.start() <= *current.end() + 1 {
            let new_end = (*current.end()).max(*range.end());
            current = *current.start()..=new_end;
        } else {
            merged.push(current);
            current = range;
        }
    }
    merged.push(current);

    merged.iter().map(|r| r.end() - r.start() + 1).sum()
}
