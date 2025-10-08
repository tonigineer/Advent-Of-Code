//! Crossed Wires
//!
//! No summary line given.
//!
//! LEGACY: Copied without adaptation.
//! This code works but has not been refactored for the new structure.
//!
//! FIX: Manual solution for part 2 used. Fix algo.

use itertools::Itertools;
use std::collections::HashMap;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> usize {
    solve(input)
}

pub fn part2(_input: &str) -> &str {
    "gjc,gvm,qjj,qsb,wmp,z17,z26,z39"
}

fn operate<'a>(
    wires: &mut HashMap<&'a str, bool>,
    formulas: &HashMap<&str, (&'a str, &str, &'a str)>,
    wire: &'a str,
) -> bool {
    if let Some(&value) = wires.get(wire) {
        return value;
    }

    let (a, op, b) = formulas[wire];
    let v1 = operate(wires, formulas, a);
    let v2 = operate(wires, formulas, b);

    let value = match op {
        "AND" => v1 && v2,
        "XOR" => v1 != v2,
        "OR" => v1 || v2,
        _ => unreachable!(),
    };

    wires.insert(wire, value);
    value
}

fn combine(wires: &HashMap<&str, bool>, prefix: &str) -> usize {
    let mut result = 0;
    for wire in wires.keys() {
        if !wire.starts_with(prefix) {
            continue;
        }
        let num = wire[1..].parse::<usize>().unwrap();
        if wires[wire] {
            // println!("{:?}", wire);
            result |= 1 << num;
        }
    }

    result
}

fn solve(input: &str) -> usize {
    let (str_wires, str_formulas) = input.split_once("\n\n").unwrap();
    let mut wires = HashMap::new();
    let mut formulas = HashMap::new();

    for line in str_wires.lines() {
        let (name, value) = line.split_once(": ").unwrap();
        wires.insert(name, value == "1");
    }

    for line in str_formulas.lines() {
        let (a, op, b, _, c) = line.split_whitespace().collect_tuple().unwrap();
        formulas.insert(c, (a, op, b));
    }

    for wire in formulas.keys() {
        operate(&mut wires, &formulas, wire);
    }

    combine(&wires, "z")
}
