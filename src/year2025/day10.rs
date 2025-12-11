//! # Factory
//!
//! TODO: Avoid using external library.
//!
//! Linear problem Ax = b can be solved by reducing solution space via Gaussian Elimination and then brute force
//! all remaining solutions for find optimum.

use good_lp::{Expression, Solution, SolverModel, constraint, microlp, variable, variables};
use std::collections::{HashSet, VecDeque};

use crate::common::parse::ParseInteger;

pub fn parse(input: &str) -> Vec<Machine> {
    input.trim().lines().map(Machine::new).collect()
}

#[derive(Debug)]
pub struct Machine {
    indicator_lights: u16,
    buttons: Vec<Vec<usize>>,
    button_masks: Vec<u16>,
    joltage: Vec<u32>,
}

impl Machine {
    fn new(line: &str) -> Self {
        let mut indicator_lights = 0;
        let mut buttons = Vec::new();
        let mut button_masks = Vec::new();

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
            let mut mask = 0;
            let mut button = Vec::new();

            for value in button_str.parse_uint_iter::<u16>() {
                mask ^= 1 << value;
                button.push(value as usize);
            }

            button_masks.push(mask);
            buttons.push(button);
        }

        // Joltage
        let joltage = tail.parse_uint_iter().collect();

        Self { indicator_lights, button_masks, buttons, joltage }
    }
}

pub fn part1(machines: &Vec<Machine>) -> i16 {
    let mut result = 0;

    for machine in machines {
        let mut seen = HashSet::new();

        let mut q: VecDeque<(i16, u16)> = VecDeque::new();
        q.push_back((0, 0));

        while let Some((steps, indicator)) = q.pop_front() {
            if indicator == machine.indicator_lights {
                result -= steps;
                break;
            }

            if !seen.insert(indicator) {
                continue;
            }

            for mask in &machine.button_masks {
                q.push_back((steps - 1, indicator ^ mask));
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
    // Sum expressions (joltage) defined for solver look like:
    //
    // &sum = v5 + v4
    // &sum = v1 + v5
    // &sum = v4 + v3 + v2
    // &sum = v1 + v0 + v3

    let mut result = 0;

    for machine in machines {
        let mut vars = variables!();

        // Init a variable for each button
        let buttons = (0..machine.buttons.len())
            .map(|_| vars.add(variable().min(0).integer()))
            .collect::<Vec<_>>();

        // Minimize for sum of all buttons
        let buttons_sum = buttons.iter().sum::<Expression>();
        let mut solver = vars.minimise(buttons_sum).using(microlp);

        // Build constraints with buttons that affect joltage to meet target joltage
        for (idx, target) in machine.joltage.iter().enumerate() {
            let joltage = (machine.buttons.iter().enumerate())
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
