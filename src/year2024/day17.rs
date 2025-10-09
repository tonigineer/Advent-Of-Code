//! Chronospatial Computer
//!
//! Summary: The `a` register is composed of multiple 8-bit values that are shifted left by 3 bits on
//! each computer execution step. By reconstructing each bit of the current program step in reverse,
//! we can determine the initial value of the `a` register. This solution only works for specific
//! inputs whose final values end with zeros.

use crate::common::parse::*;

pub fn parse(input: &str) -> Vec<u64> {
    input.trim().parse_uint_iter().collect()
}

pub fn part1(input: &[u64]) -> String {
    let register_a = input[0];

    let mut computer = Computer::new(input, register_a);
    let mut values = Vec::new();

    while let Some(value) = computer.execute() {
        values.push(value);
    }

    values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")
}

fn reverse_find(input: &[u64], check_idx: usize, a: u64) -> Option<u64> {
    if check_idx == 2 {
        return Some(a);
    }

    (0..8).find_map(|t| {
        let new_a = (a << 3) | t;
        let mut computer = Computer::new(input, new_a);
        let result = computer.execute().unwrap();

        if result == input[check_idx] { reverse_find(input, check_idx - 1, new_a) } else { None }
    })
}
pub fn part2(input: &[u64]) -> u64 {
    assert!(*input.last().unwrap() == 0, "Implemented approach does not work for such an input.");

    reverse_find(input, input.len() - 1, 0).unwrap()
}

struct Computer<'a> {
    a: u64,
    b: u64,
    c: u64,
    program: &'a [u64],
    ip: usize,
}

impl Computer<'_> {
    fn new(input: &[u64], a: u64) -> Computer<'_> {
        Computer { a, b: 0, c: 0, program: &input[3..], ip: 0 }
    }

    fn execute(&mut self) -> Option<u64> {
        while self.ip < self.program.len() {
            let literal = || self.program[self.ip + 1];
            let combo = || match self.program[self.ip + 1] {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                7 => unimplemented!(),
                _ => unreachable!(),
            };

            match self.program[self.ip] {
                0 => self.a >>= combo(),
                1 => self.b ^= literal(),
                2 => self.b = combo() % 8,
                3 => {
                    if self.a != 0 {
                        self.ip = literal() as usize;
                        continue;
                    }
                }
                4 => self.b ^= self.c,
                5 => {
                    let ret = combo() % 8;
                    self.ip += 2;
                    return Some(ret);
                }
                6 => self.b = self.a >> combo(),
                7 => self.c = self.a >> combo(),
                _ => unreachable!(),
            }

            self.ip += 2;
        }

        None
    }
}
