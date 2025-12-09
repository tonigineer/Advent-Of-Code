//! # Playground
//!
//!

use itertools::Itertools;

use crate::common::parse::ParseInteger;
use std::collections::HashMap;

type ParsedInput = (Vec<(usize, usize)>, Vec<Junction>, usize);

const NUM_PAIRS_PUZZLE: usize = 1000;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Junction {
    id: usize,
    x: i64,
    y: i64,
    z: i64,
}

impl Junction {
    pub fn new(id: usize, line: &str) -> Self {
        let mut it = line.parse_int_iter::<i64>();

        Self { id, x: it.next().unwrap(), y: it.next().unwrap(), z: it.next().unwrap() }
    }

    pub fn distance_squared(&self, other: Junction) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;

        dx * dx + dy * dy + dz * dz
    }
}

pub fn parse(input: &str) -> ParsedInput {
    let junctions: Vec<Junction> =
        input.trim().lines().enumerate().map(|(id, line)| Junction::new(id, line)).collect();

    let pairs: HashMap<(usize, usize), i64> = junctions
        .iter()
        .tuple_combinations()
        .map(|(j1, j2)| ((j1.id, j2.id), j1.distance_squared(*j2)))
        .collect();

    let sorted_pairs = pairs
        .iter()
        .sorted_by_key(|&(_, distance)| distance)
        .map(|(&(j1, j2), _)| (j1, j2))
        .collect();

    (sorted_pairs, junctions, NUM_PAIRS_PUZZLE)
}

pub fn part1(input: &ParsedInput) -> usize {
    let (sorted_pairs, junctions, num_pairs) = input;

    let mut parents = Vec::from_iter(0..junctions.len());
    let mut circuit_sizes = vec![0; parents.len()];

    for (j1, j2) in sorted_pairs.iter().take(*num_pairs) {
        merge(&mut parents, *j1, *j2);
    }

    for idx in 0..parents.len() {
        circuit_sizes[root(&mut parents, idx)] += 1;
    }

    circuit_sizes.sort();
    circuit_sizes.into_iter().rev().take(3).product::<usize>()
}

pub fn part2(input: &ParsedInput) -> i64 {
    let (sorted_pairs, junctions, _) = input;

    let mut parents = Vec::from_iter(0..junctions.len());
    let mut num_circuits = parents.len();

    for &(j1, j2) in sorted_pairs.iter() {
        if root(&mut parents, j1) == root(&mut parents, j2) {
            continue;
        }

        merge(&mut parents, j1, j2);

        num_circuits -= 1;
        if num_circuits == 1 {
            return junctions[j1].x * junctions[j2].x;
        }
    }

    unreachable!()
}

fn root(parents: &mut Vec<usize>, junction: usize) -> usize {
    if parents[junction] == junction {
        return junction;
    }

    parents[junction] = root(parents, parents[junction]);
    parents[junction]
}

fn merge(parents: &mut Vec<usize>, junction_a: usize, junction_b: usize) {
    let root_a = root(parents, junction_a);
    let root_b = root(parents, junction_b);
    parents[root_a] = root_b;
}
