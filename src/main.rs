use std::time::Instant;
use clap::Parser;
use args::{Args, Commands};

mod args;

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Solve { year, day } => {
            let solutions = match year {
                2015 => aoc_2015::ALL,
                2023 => aoc_2023::ALL,
                _ => unimplemented!()
            };

            let solution =  match solutions.get(day as usize - 1) {
                Some(s) => s,
                None => {
                    println!("There is not solution implemented for Day: {} of Year: {}.", day, year);
                    return
                }
            };

            let input = common::load_input(day, year).unwrap();
            let input = input.trim();

            println!(
                "* {year}-{:0>2}  󰎤 {}  󰎧 {}",
                day,
                solution.part1(&input),
                solution.part2(&input)
            );
        },
        Commands::List { year } => {
            let solutions = match year {
                2015 => aoc_2015::ALL,
                2023 => aoc_2023::ALL,
                _ => unimplemented!()
            };

            println!("AOC {}", year);
            for (idx, solution) in solutions.iter().enumerate() {
                let day = idx as u32 + 1;
                let input = common::load_input(day, year).unwrap();
                let input = input.trim();

                let start = Instant::now();
                let part1 = solution.part1(&input);
                let part2 = solution.part2(&input);
                let time = start.elapsed().as_nanos() as f32 / 1e6;

                println!(
                    "{} Day {:0>2}   󰎤 {: <13}   󰎧 {: <13}    {: >13}ms",
                    if day == solutions.len() as u32 {
                        "└"
                    } else {
                        "├"
                    },
                    day.to_string(),
                    part1.to_string(),
                    part2.to_string(),
                    format!("{:4.6}", time)
                );
            };
        }
    }
}