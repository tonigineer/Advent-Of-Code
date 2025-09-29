//! # I Was Told There Would Be No Math
//!
//! Straight forward OOP approach.

struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn new(dims: Vec<u32>) -> Self {
        let l = dims[0];
        let w = dims[1];
        let h = dims[2];

        Present { l, w, h }
    }

    fn paper_needed(&self) -> u32 {
        return 3 * self.l * self.w + 2 * self.l * self.h + 2 * self.w * self.h;
    }

    fn ribbon_needed(&self) -> u32 {
        return 2 * self.l + 2 * self.w + self.l * self.h * self.w;
    }
}

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> u32 {
    let mut paper: u32 = 0;

    for line in input.lines() {
        let mut dims: Vec<u32> = line.split('x').map(|item| item.parse().unwrap()).collect();
        dims.sort();

        let p = Present::new(dims);
        paper += p.paper_needed();
    }

    return paper;
}
pub fn part2(input: &str) -> u32 {
    let mut ribbon: u32 = 0;

    for line in input.lines() {
        let mut dims: Vec<u32> = line.split('x').map(|item| item.parse().unwrap()).collect();
        dims.sort();

        let p = Present::new(dims);
        ribbon += p.ribbon_needed();
    }

    return ribbon;
}
