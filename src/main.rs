use aoc::common::ansi::*;
use aoc::common::input::read_puzzle_input;
use aoc::common::parse::*;
use aoc::*;

use std::env::args;
use std::iter::empty;
use std::time::{Duration, Instant};

struct Puzzle {
    day: u32,
    year: u32,
    wrapper: fn(String) -> (String, String),
}

fn main() {
    let joined = args().collect::<Vec<_>>().join("");
    let joined = joined.as_str();
    let mut iter = joined.parse_uint_iter::<u32>();
    let (year, day) = (iter.next(), iter.next());

    let (stars, duration) = empty()
        .chain(year2015())
        .chain(year2024())
        .chain(year2025())
        .filter(|puzzle| year.is_none_or(|y| y == puzzle.year))
        .filter(|puzzle| day.is_none_or(|d| d == puzzle.day))
        .fold((0, Duration::ZERO), |(stars, duration), Puzzle { year, day, wrapper }| {
            if let Ok(data) = read_puzzle_input(&day, &year) {
                let instant = Instant::now();
                let (part1, part2) = wrapper(data);
                let elapsed = instant.elapsed();

                output_summary_line(&year, &day, &elapsed, &part1, &part2);
                (stars + 2, duration + elapsed)
            } else {
                (stars, duration)
            }
        });

    println!(
        "\n\n   {BOLD}SUMMARY   {YELLOW}{}{RESET} Stars in {BOLD}{GREEN}{}{RESET} ms",
        stars,
        duration.as_millis()
    );
}

fn output_summary_line(year: &u32, day: &u32, elapsed: &Duration, part1: &str, part2: &str) {
    print!(
        "{} Day {MAGENTA}{:0>2}{RESET} {}",
        year,
        day,
        match elapsed.as_micros() {
            0..1_000 => format!("{GREEN}{: >4}{RESET} µs", elapsed.as_micros()),
            1_000..1_000_000 => format!("{YELLOW}{: >4}{RESET} ms", elapsed.as_millis()),
            _ => format!("{RED}{: >4}{RESET} s ", elapsed.as_secs()),
        }
    );
    print!(" [{BOLD}{part1}{RESET}/{BOLD}{part2}{RESET}]");
    println!();
}

macro_rules! season {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Puzzle> {
            vec![$({
                let year: u32 = stringify!($year).parse_uint::<u32>();
                let day: u32 = stringify!($day).parse_uint::<u32>();
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
    day01, day02, day03, day04, day05, day06, day07, day08,
    day09, day10, day11, day12, day13, day14, day15, day16,
    day17, day18, day19, day20, day21, day22, day23, day24,
    day25
);

season!(year2024
    day01, day02, day03, day04, day05, day06, day07, day08,
    day09, day10, day11, day12, day13, day14, day15, day16,
    day17, day18, day19, day20, day21, day22, day23, day24,
    day25
);

season!(year2025
    day01, day02, day03
);
