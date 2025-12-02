//! # Secret Entrance
//!
//!

use crate::common::parse::ParseInteger;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .fold((50, 0), |mut acc, l| {
            let dir = l.as_bytes()[0];
            let num = l.parse_int::<i32>();

            acc.0 = match dir {
                b'L' => acc.0 - num,
                b'R' => acc.0 + num,
                _ => unreachable!(),
            };

            acc.0 = acc.0.rem_euclid(100);

            if acc.0 == 0 {
                acc.1 += 1;
            }

            acc
        })
        .1
}

pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .fold((50, 0), |mut acc, l| {
            let dir = l.as_bytes()[0];
            let num = l.parse_int::<i32>();

            let div = num.div_euclid(100);
            let rem = num.rem_euclid(100);

            (acc.0, acc.1) = match dir {
                b'L' => {
                    if acc.0 - rem < 0 && acc.0 != 0 {
                        acc.1 += 1;
                    }

                    (acc.0 - num, acc.1 + div)
                }
                b'R' => {
                    if acc.0 + rem > 100 {
                        acc.1 += 1;
                    }

                    (acc.0 + num, acc.1 + div)
                }
                _ => unreachable!(),
            };

            acc.0 = acc.0.rem_euclid(100);

            if acc.0 == 0 {
                acc.1 += 1;
            }

            acc
        })
        .1
}
