# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an Advent of Code 2025 solutions repository written in Rust. Each day's solution is implemented as a separate module under `src/day_*/mod.rs`.

## Running Solutions

Build and run a specific day's solution:
```bash
cargo run <dayName> <input_file_path>
```

Examples:
```bash
cargo run dayOne input/dayOne.txt
cargo run dayOne input/dayOneExample.txt
cargo run dayTwo input/dayTwo.txt
```

Enable debug logging:
```bash
RUST_LOG=debug cargo run dayOne input/dayOne.txt
```

Build only:
```bash
cargo build
```

## Architecture

### Entry Point (`src/main.rs`)
- Uses `clap` for CLI argument parsing (problem name and input file path)
- Reads input file as a string
- Dispatches to the appropriate day module's `solution()` function
- All solutions return `u64` and print the answer

### Day Modules (`src/day_*/mod.rs`)
Each day module:
- Exports a public `solution(input: String) -> u64` function
- Parses the input string specific to that day's problem
- Returns the numeric answer as `u64`
- Uses `log::info!()` and `log::debug!()` for tracing execution (controlled by `RUST_LOG` environment variable)

### Input Files (`input/`)
- `day*.txt` - Full problem input
- `day*Example.txt` - Example input from problem description (useful for testing)

## Adding a New Day's Solution

1. Create a new directory: `src/day_<name>/`
2. Create `src/day_<name>/mod.rs` with a `pub fn solution(input: String) -> u64` function
3. Add the module to `src/main.rs`: `mod day_<name>;`
4. Add a match arm in `main()` to route to the new solution
5. Add input files to `input/day<Name>.txt` and optionally `input/day<Name>Example.txt`

Example pattern:
```rust
// In src/main.rs
mod day_four;

// In match statement
let answer: u64 = match args.problem.as_str() {
    "dayOne" => day_one::solution(input),
    "dayTwo" => day_two::solution(input),
    "dayFour" => day_four::solution(input),
    _ => panic!("problem solution not found"),
};
```

## Naming Convention

- Problem names use camelCase: `dayOne`, `dayTwo`, etc.
- Module directories use snake_case: `day_one/`, `day_two/`, etc.
- Input files use camelCase: `dayOne.txt`, `dayOneExample.txt`
