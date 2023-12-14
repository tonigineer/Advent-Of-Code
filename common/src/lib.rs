use std::{io, fs};
use std::collections::VecDeque;
use itertools::Itertools;


mod answer;
mod solution;

pub use answer::Answer;
pub use solution::Solution;


// ======================================================
// ------------------------ MISC ------------------------
// ======================================================
pub fn load_input(day: u32, year: u32) -> io::Result<String> {
    let input_path = format!("aoc_{}/inputs/{:0>2}.in", year, day);
    return fs::read_to_string(input_path)
}


// ======================================================
// ------------------------ GRID ------------------------
// ======================================================
#[derive(Debug,Clone)]
pub struct Grid {
    pub grid: VecDeque<VecDeque<char>>
}

impl From <&str> for Grid {
    fn from(input: &str) -> Grid {
        Grid {
            grid: input.lines()
                    .map(|line| {
                        line.chars()
                            .map(|c| c)
                            .collect()
                    })
                    .collect()
        }
    }
}

impl Grid {
    /// Output `Grid` as joined strings for each row.
    /// 
    /// EXAMPLE
    /// -------
    /// 
    /// ```
    /// let mut g: Grid = "x..x\n.xx.".into();
    /// 
    /// println!("{:?}", g.grid);
    /// [['x', '.', '.', 'x'], ['.', 'x', 'x', '.']]
    /// 
    /// g.pretty_print();
    /// x..x
    /// .xx.
    /// ```
    pub fn pretty_print(&self) {
        for r in self.grid.iter() {
            println!("{}", r.iter().join(""));
        }
    }

    /// Transpose `Grid` in place.
    /// 
    /// EXAMPLE
    /// -------
    /// 
    /// ```
    /// let mut g: Grid = "x..x\n.xx.".into();
    /// 
    /// g.pretty_print();
    /// x..x
    /// .xx.
    /// 
    /// g.transpose();
    /// g.pretty_print();
    /// x.
    /// .x
    /// .x
    /// x.
    /// ```
    pub fn transpose(&mut self) {
        self.grid = (0..self.grid[0].len())
            .map(|c| {
                self.grid
                    .iter()
                    .map(|row| row[c])
                    .collect()
            })
            .collect()
    }
}