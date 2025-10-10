//! Doesn't He Have Intern-Elves For This?
//!
//! Summary: Processes both parts in one pass through all lines.
//! The fourth condition and the padding used for `tuple_windows()` could be refined further.

use hashbrown::HashMap;
use itertools::Itertools;

pub fn part1(input: &(usize, usize)) -> usize {
    input.0
}

pub fn part2(input: &(usize, usize)) -> usize {
    input.1
}

pub fn parse(input: &str) -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.trim().lines() {
        let mut first = 0;
        let mut second = false;
        let mut third = true;

        let mut fourth = HashMap::new();
        let mut fifth = false;

        let mut last_pair = (b'0', b'0');

        let padded_line = line.as_bytes().iter().chain([b'^', b'$'].iter());

        for (a, b, c) in padded_line.tuple_windows() {
            if [b'a', b'e', b'i', b'o', b'u'].contains(a) {
                first += 1;
            }

            if *a == *b {
                second = true;
            }

            if (*a == b'a' || *a == b'c' || *a == b'p' || *a == b'x') && *b == *a + 1 {
                third = false;
            }

            let new_pair = (*a, *b);
            if new_pair != last_pair {
                *fourth.entry(new_pair).or_insert(0) += 1;
                last_pair = new_pair;
            } else {
                last_pair = (b'0', b'0');
            }

            if *a == *c {
                fifth = true;
            }
        }

        if first >= 3 && second && third {
            part1 += 1
        }

        if fifth
            && let Some(m) = fourth.values().max()
            && *m >= 2
        {
            part2 += 1;
        }
    }

    (part1, part2)
}
