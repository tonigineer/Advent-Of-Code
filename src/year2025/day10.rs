//! # Factory
//!
//! Refactor and optimize.
//!
//! TODO: Get rid of external library by using Gaussian Elimination to reduze solution space for Ax=b and then DFS it.

use good_lp::{Expression, Solution, SolverModel, coin_cbc, constraint, variable, variables};
use std::collections::{BinaryHeap, HashSet};

use crate::common::parse::ParseInteger;

pub fn parse(input: &str) -> Vec<Machine> {
    input.trim().lines().map(|l| Machine::new(l)).collect()
}

#[derive(Debug)]
pub struct Machine {
    indicator_lights: u16,
    buttons: Vec<u16>,
    buttons2: Vec<Vec<usize>>,
    joltage: Vec<u32>,
}

impl Machine {
    fn new(line: &str) -> Self {
        let mut indicator_lights = 0;
        let mut buttons = Vec::new();
        let mut buttons2 = Vec::new();

        // Indicator lights
        let (head, tail) = line.split_once("] (").unwrap();

        for (idx, char) in head.chars().skip(1).enumerate() {
            if char == '#' {
                indicator_lights ^= 1 << idx;
            }
        }

        // Buttons
        let (head, tail) = tail.split_once(") {").unwrap();
        for button_str in head.split_whitespace() {
            let mut b = 0;
            let mut b2 = Vec::new();

            for value in button_str.parse_uint_iter::<u16>() {
                b ^= 1 << value;
                b2.push(value as usize);
            }
            buttons.push(b);
            buttons2.push(b2);
        }

        // Joltage
        let joltage = tail.parse_uint_iter().collect();

        Self { indicator_lights, buttons, buttons2, joltage }
    }
}

pub fn part1(machines: &Vec<Machine>) -> i16 {
    let mut result = 0;

    for machine in machines {
        let mut seen = HashSet::new();

        let mut heap: BinaryHeap<(i16, u16)> = BinaryHeap::new();
        heap.push((0, 0));

        while let Some((steps, indicator)) = heap.pop() {
            if indicator == machine.indicator_lights {
                result -= steps;
                break;
            }

            if !seen.insert(indicator) {
                continue;
            }

            for b in &machine.buttons {
                heap.push((steps - 1, indicator ^ b));
            }
        }
    }

    result
}

pub fn part2(machines: &Vec<Machine>) -> u32 {
    // Example 1:
    //
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    //
    // Equations to solve:
    //
    // b4 * x4 + b5 * x5            = 3
    // b1 * x1 + b5 * x5            = 5
    // b2 * x2 + b3 * x3 + b4 * x4  = 4
    // b0 * x0 + b1 * x1 + b3 * x3  = 7
    //
    // Sum expressions defined for solver look like:
    //
    // &sum = v5 + v4
    // &sum = v1 + v5
    // &sum = v4 + v3 + v2
    // &sum = v1 + v0 + v3

    let mut result = 0;

    for machine in machines {
        let mut vars = variables!();

        // Init a variable for each button
        let buttons = (0..machine.buttons2.len())
            .map(|_| vars.add(variable().min(0).integer()))
            .collect::<Vec<_>>();

        // Minimize for sum of all buttons
        let buttons_sum = buttons.iter().sum::<Expression>();
        let mut solver = vars.minimise(buttons_sum).using(coin_cbc);

        // Build constraints with buttons that affect joltage to meet target joltage
        for (idx, target) in machine.joltage.iter().enumerate() {
            let joltage = (machine.buttons2.iter().enumerate())
                .filter(|(_, x)| x.contains(&idx))
                .map(|(i, _)| buttons[i])
                .sum::<Expression>();

            solver.add_constraint(constraint!(joltage == *target));
        }

        let solution = solver.solve().unwrap();
        result += (buttons.iter()).map(|x| solution.value(*x) as u32).sum::<u32>();
    }

    result
}
