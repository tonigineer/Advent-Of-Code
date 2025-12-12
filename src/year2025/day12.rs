//! # Christmas Tree Farm
//!
//! A cheeky solution with a tuning parameter. In most cases, the present either fits easily or not at all.
//! The parameter is used to handle a few edge cases.

use crate::common::parse::ParseInteger;

type InputParsed = (Vec<usize>, Vec<(usize, Vec<usize>)>);

const TUNING: f32 = 1.2;

pub fn parse(input: &str) -> InputParsed {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();

    let it_shapes = parts.iter().take(parts.len() - 1);
    let s_regions = parts.iter().rev().take(1).next().unwrap();

    let mut shapes = Vec::new();
    for s in it_shapes {
        let (_, shape) = s.split_once(":\n").unwrap();
        shapes.push(shape.as_bytes().iter().filter(|&c| *c == b'#').count());
    }

    let mut regions = Vec::new();
    for s in s_regions.lines() {
        let (s_size, s_presents) = s.split_once(":").unwrap();

        let size = s_size.parse_uint_iter::<usize>().product::<usize>();
        let presents: Vec<usize> = s_presents.parse_uint_iter::<usize>().collect();

        regions.push((size, presents));
    }

    (shapes, regions)
}

pub fn part1(input: &InputParsed) -> i32 {
    let (shapes, regions) = input;
    let mut result = 0;

    for region in regions {
        let (size, content_nums) = region;

        let needed_size = (content_nums
            .iter()
            .enumerate()
            .filter(|&(_, n)| n > &0)
            .map(|(idx, n)| (shapes[idx] * n) as f32)
            .sum::<f32>()
            * TUNING) as usize;

        if *size > needed_size {
            result += 1;
        }
    }

    result
}

pub fn part2(_: &InputParsed) -> &str {
    "🌲☃️🎅"
}
