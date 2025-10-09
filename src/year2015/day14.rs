//! Reindeer Olympics
//!
//! Summary: Nothing fancy here. Simlation function to get time specific results.

use crate::common::parse::*;
use itertools::Itertools;

type Reindeer = [i32; 3];

pub fn parse(input: &str) -> Vec<Reindeer> {
    input
        .trim()
        .parse_int_iter::<i32>()
        .tuples::<(i32, i32, i32)>()
        .map(|(a, b, c)| [a, b, c])
        .collect()
}

pub fn part1(input: &[Reindeer]) -> i32 {
    part1_for_test(input, 2503)
}

pub fn part2(input: &[Reindeer]) -> i32 {
    part2_for_test(input, 2503)
}

pub fn part1_for_test(input: &[Reindeer], t_sim: i32) -> i32 {
    input.iter().map(|&r| simulate(r, t_sim)).max().unwrap()
}

pub fn part2_for_test(input: &[Reindeer], t_sim: i32) -> i32 {
    let mut scores = vec![0; input.len()];
    let mut distances = vec![0; input.len()];

    for dt in 0..t_sim {
        let mut max_distance = 0;

        for (idx, &reindeer) in input.iter().enumerate() {
            let next = simulate(reindeer, dt + 1);
            max_distance = max_distance.max(next);
            distances[idx] = next;
        }

        for (score, &distance) in scores.iter_mut().zip(&distances) {
            if distance == max_distance {
                *score += 1;
            }
        }
    }

    *scores.iter().max().unwrap()
}

fn simulate([v, t_fly, t_rest]: Reindeer, t_sim: i32) -> i32 {
    let t_cycle = t_fly + t_rest;
    let cycles_completed = t_sim / t_cycle;
    let t_remain = (t_sim % t_cycle).min(t_fly);

    v * (cycles_completed * t_fly + t_remain)
}
