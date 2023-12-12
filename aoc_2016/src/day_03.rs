use common::{Answer, Solution};
use itertools::Itertools;

pub struct Day03;

impl Solution for Day03 {
    fn name(&self) -> &'static str {
        "Squares With Three Sides"
    }

    fn part1(&self, input: &str) -> Answer {
        let triangles: Vec<Vec<i32>> = input
            .lines()
            .map(|l| l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec()
            ).collect_vec();
        return part1(triangles).into();
    }

    fn part2(&self, input: &str) -> Answer {
        let triangles: Vec<Vec<i32>> = input
            .lines()
            .map(|l| l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec()
            ).collect_vec();
        return part2(triangles).into();
    }
}

fn part1(mut input: Vec<Vec<i32>>) -> i32 {
    let mut valid_triangle = 0;

    for item in input.iter_mut() {
        item.sort();
        if item[0] + item[1] > item[2] {
            valid_triangle += 1;
        }
    }

    return valid_triangle;
}

fn part2(input: Vec<Vec<i32>>) -> i32 {
    let mut valid_triangle = 0;

    for i in (0..input.len()).step_by(3) {
        for k in 0..=2 {
            valid_triangle += (
                input[i+0][k] + input[i+1][k] > input[i+2][k] &&
                input[i+0][k] + input[i+2][k] > input[i+1][k] &&
                input[i+2][k] + input[i+1][k] > input[i+0][k]
            ) as i32;
        }
    }

    return valid_triangle;
}