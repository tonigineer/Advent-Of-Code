use std::{fs, io};

mod answer;
mod solution;

pub use answer::Answer;
pub use solution::Solution;

pub fn load_input(day: u32, year: u32) -> io::Result<String> {
    let input_path = format!("aoc_{}/inputs/{:0>2}.in", year, day);
    return fs::read_to_string(input_path)
}