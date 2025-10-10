//! Crossed Wires
//!
//! Summary: In Part 1 the wire names are converted into an index for faster lookup
//! in an array. Index is build from three 6 bit parts for each char.
//! Solution for Part 2 from `github.com/maneatingape/advent-of-code-rust`

use crate::common::parse::*;

use hashbrown::HashSet;
use std::collections::VecDeque;

type InputParsed<'a> = (&'a str, Vec<[&'a str; 5]>);

pub fn parse(input: &'_ str) -> InputParsed<'_> {
    let (s1, s2) = input.split_once("\n\n").unwrap();

    let tokens: Vec<&str> = s2.split_whitespace().collect();
    let gates =
        tokens.chunks(5).map(|chunk| [chunk[0], chunk[1], chunk[2], chunk[3], chunk[4]]).collect();

    (s1, gates)
}

pub fn part1(input: &InputParsed) -> u64 {
    let (s1, gates) = input;

    let mut wires = [u8::MAX; 1 << 18];

    let as_index = |s: &str| {
        let b = s.as_bytes();
        ((b[0] as usize & 63) << 12) + ((b[1] as usize & 63) << 6) + (b[2] as usize & 63)
    };

    for line in s1.lines() {
        let wire_id = as_index(&line[..3]);
        let value = &line[5..];
        wires[wire_id] = value.parse_uint::<u8>();
    }

    let mut q: VecDeque<_> = gates.iter().collect();

    while let Some(gate @ &[left, gate_type, right, _, to]) = q.pop_front() {
        let left_value = wires[as_index(left)];
        let right_value = wires[as_index(right)];

        // Both input wires are needed. Try again later.
        if left_value == u8::MAX || right_value == u8::MAX {
            q.push_back(gate);
            continue;
        }

        wires[as_index(to)] = match gate_type {
            "AND" => left_value & right_value,
            "OR" => left_value | right_value,
            "XOR" => left_value ^ right_value,
            _ => unreachable!(),
        }
    }

    let mut result = 0_u64;
    for wire_idx in (as_index("z00")..as_index("z63")).rev() {
        let value = wires[wire_idx];
        if value != u8::MAX {
            result = (result << 1) | (value as u64);
        }
    }

    result
}

pub fn part2(input: &InputParsed) -> String {
    let (_, gates) = input;

    let mut outputs = HashSet::new();
    let mut swapped = HashSet::new();

    for [left, gate_type, right, _, _] in gates.clone() {
        outputs.insert((left, gate_type));
        outputs.insert((right, gate_type));
    }

    for [left, gate_type, right, _, to] in gates.clone() {
        match gate_type {
            "AND" => {
                // Check that all AND gates point to an OR, except for first AND.
                if left != "x00" && right != "x00" && !outputs.contains(&(to, "OR")) {
                    swapped.insert(to);
                }
            }
            "OR" => {
                // Check that only XOR gates point to output, except for last carry which is OR.
                if to.starts_with('z') && to != "z45" {
                    swapped.insert(to);
                }
                // OR can never point to OR.
                if outputs.contains(&(to, "OR")) {
                    swapped.insert(to);
                }
            }
            "XOR" => {
                if left.starts_with('x') || right.starts_with('x') {
                    // Check that first level XOR points to second level XOR, except for first XOR.
                    if left != "x00" && right != "x00" && !outputs.contains(&(to, "XOR")) {
                        swapped.insert(to);
                    }
                } else {
                    // Second level XOR must point to output.
                    if !to.starts_with('z') {
                        swapped.insert(to);
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    let mut result: Vec<_> = swapped.into_iter().collect();
    result.sort_unstable();
    result.join(",")
}
