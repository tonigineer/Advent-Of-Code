use clap::Parser;
use common::input::load_input;
use std::time::Instant;

use clap::Subcommand;

#[derive(Parser, Debug)]

pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Solve a puzzles for a day.")]
    Solve { year: u32, day: u32 },
    #[command(about = "List results for a year.")]
    List { year: u32 },
}

/// Main entry point of the Advent of Code solver.
///
/// This program parses command-line arguments to either solve a specific day's puzzle or list all
/// solutions for a particular year. It supports years 2015, 2016, 2023, and 2024.
///
/// # Commands
///
/// - `solve`: Solves the puzzle for a specified `year` and `day`.
///   - Loads the input for the given day and year.
///   - Executes `part1` and `part2` of the solution.
///   - Prints the results for both parts.
/// - `list`: Lists all solutions for the specified `year`.
///   - Iterates over all implemented solutions for that year.
///   - Loads inputs and executes both parts of each solution.
///   - Measures and displays the execution time for each day's solution.
///
/// # Usage
///
/// To solve a specific day's puzzle:
/// ```bash
/// cargo run --release solve YEAR DAY
/// ```
///
/// To list all solutions for a year:
/// ```bash
/// cargo run --release list YEAR
/// ```
///
/// # Examples
///
/// Solve Day 1 of 2023:
/// ```bash
/// cargo run --release solve 2023 1
/// ```
///
/// List all solutions for 2023:
/// ```bash
/// cargo run --release list 2023
/// ```
fn main() {
    let args = Args::parse();

    let solutions = match &args.command {
        Commands::Solve { year, .. } | Commands::List { year } => match year {
            2015 => aoc_2015::ALL,
            2016 => aoc_2016::ALL,
            2023 => aoc_2023::ALL,
            2024 => aoc_2024::ALL,
            _ => {
                println!("No solutions implemented for Year: {}.", year);
                return;
            }
        },
    };

    match args.command {
        Commands::Solve { year, day } => {
            if let Some(solution) = solutions.get((day - 1) as usize) {
                let input = load_input(day, year).unwrap_or_else(|_| {
                    panic!("Failed to load input for Day: {} of Year: {}.", day, year)
                });
                let input = input.trim();

                println!(
                    "* {year}-{:0>2}  󰎤 {}  󰎧 {}",
                    day,
                    solution.part1(input),
                    solution.part2(input)
                );
            } else {
                println!(
                    "No solution implemented for Day: {} of Year: {}.",
                    day, year
                );
            }
        }

        Commands::List { year } => {
            println!("AOC {}", year);
            for (idx, solution) in solutions.iter().enumerate() {
                let day = idx as u32 + 1;
                let input = load_input(day, year).unwrap_or_else(|_| {
                    panic!("Failed to load input for Day: {} of Year: {}.", day, year)
                });
                let input = input.trim();

                let start = Instant::now();
                let part1 = solution.part1(input);
                let part2 = solution.part2(input);
                let elapsed = start.elapsed();
                let time_ms = elapsed.as_secs_f64() * 1000.0;

                println!(
                    "{} Day {:0>2}   󰎤 {: <16}   󰎧 {: <16}    {: >11.6} ms",
                    if day == solutions.len() as u32 {
                        "└"
                    } else {
                        "├"
                    },
                    day,
                    part1.to_string(),
                    part2.to_string(),
                    time_ms
                );
            }
        }
    }
}
