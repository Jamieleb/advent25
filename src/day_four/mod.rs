use log::debug;

pub fn solution(input: String) -> u64 {
    let mut lines: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();

    let mut count = 0;
    let mut should_retry = true;

    while should_retry {
        let (rolls_removed, new_state) = extract_available_rolls(&lines);
        count += rolls_removed;

        if rolls_removed > 0 {
            lines = new_state;
        } else {
            should_retry = false;
        }
    }

    count
}

fn is_roll_accessible(window: &[Vec<char>]) -> bool {
    let count_of_rolls = window.iter().flatten().filter(|c| **c == '@').count();

    debug!("window: {:?}, count: {}", window, count_of_rolls);
    count_of_rolls < 5
}

fn extract_available_rolls(roll_state: &Vec<Vec<char>>) -> (u64, Vec<Vec<char>>) {
    let mut count = 0;
    let mut new_state = roll_state.clone();

    for (i, row) in roll_state.iter().enumerate() {
        for (j, position) in row.iter().enumerate() {
            if *position == '@' {
                // Extract 3x3 window centered at (i, j)
                let mut window: Vec<Vec<char>> = Vec::new();
                for row_idx in i.saturating_sub(1)..=i.saturating_add(1).min(roll_state.len() - 1) {
                    let row_slice: Vec<char> = roll_state[row_idx][j.saturating_sub(1)
                        ..=j.saturating_add(1).min(roll_state[row_idx].len() - 1)]
                        .to_vec();
                    window.push(row_slice);
                }

                if is_roll_accessible(&window) {
                    count += 1;
                    new_state[i][j] = '.';
                }
            }
        }
    }

    (count, new_state)
}
