use aoc::common::ansi::*;
use aoc::common::input::read_puzzle_input;
use aoc::*;

use std::env::args;
use std::iter::empty;
use std::time::{Duration, Instant};

struct Puzzle {
    year: u32,
    day: u32,
    wrapper: fn(String) -> (String, String),
}

fn main() {
    let mut args = args().skip(1);

    let joined = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    let day: Option<u32> = joined.rsplit_once("day").and_then(|(_, tail)| {
        let digits: String = tail.chars().take_while(|c| c.is_ascii_digit()).collect();
        if digits.is_empty() {
            None
        } else {
            digits.parse::<u32>().ok()
        }
    });
    let year: Option<u32> = joined.rsplit_once("year").and_then(|(_, tail)| {
        let digits: String = tail.chars().take_while(|c| c.is_ascii_digit()).collect();
        if digits.is_empty() {
            None
        } else {
            digits.parse::<u32>().ok()
        }
    });

    let (stars, duration) = empty()
        .chain(year2015())
        .chain(year2024())
        .filter(|puzzle| year.is_none_or(|y| y == puzzle.year))
        .filter(|puzzle| day.is_none_or(|d| d == puzzle.day))
        .fold(
            (0, Duration::ZERO),
            |(stars, duration), Puzzle { year, day, wrapper }| {
                if let Ok(data) = read_puzzle_input(&day, &year) {
                    let instant = Instant::now();
                    let (part1, part2) = wrapper(data);
                    let elapsed = instant.elapsed();

                    print!(
                        " {YELLOW}{}{RESET} Day {GREEN}{:0>2}{RESET} {: >7} µs",
                        year,
                        day,
                        elapsed.as_micros()
                    );
                    print!(" [{BOLD}{part1}{RESET}/{BOLD}{part2}{RESET}]");
                    println!();

                    (stars + 2, duration + elapsed)
                } else {
                    (stars, duration)
                }
            },
        );

    println!(
        "\n\n   {BOLD}SUMMARY   {YELLOW}{}{RESET} Stars in {BOLD}{GREEN}{}{RESET} ms",
        stars,
        duration.as_millis()
    );
}

macro_rules! season {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Puzzle> {
            vec![$({
                // TODO: parse function for strings to get one or all signed and unsigned
                let year: u32 = stringify!($year)["year".len()..].parse().unwrap();
                let day: u32 = stringify!($day)["day".len()..].parse().unwrap();
                let wrapper = |data: String| {
                    use $year::$day::*;

                    let input = parse(&data);
                    let part1 = part1(&input);
                    let part2 = part2(&input);

                    (part1.to_string(), part2.to_string())
                };

                Puzzle { year, day, wrapper }
            },)*]
        }
    }
}

season!(year2015
    day01, day02, day03
);

season!(year2024
    day01
);
