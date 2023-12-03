use std::{fs, io};

mod answer;
mod solution;

pub use answer::Answer;
pub use solution::Solution;

pub fn load_input(day: u32, year: u32) -> io::Result<String> {
    let input_path = format!("aoc_{}/inputs/{:0>2}.in", year, day);
    return fs::read_to_string(input_path)
}

pub fn string_to_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid:Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for chr in line.chars() {
            row.push(chr);
        }
        grid.push(row)
    }
    return grid;
}