use common::{Answer, Solution};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub struct Day18;

impl Solution for Day18 {
    fn name(&self) -> &'static str {
        "RAM Run"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, true).into()
    }
}

fn solve(inp: &str, part2: bool) -> String {
    // Differentiate between tests and puzzle
    let dim = if inp.lines().count() == 25 { 6 } else { 70 };
    let mut n = if inp.lines().count() == 25 { 12 } else { 1024 };

    let mut corrupt: VecDeque<(i64, i64)> = VecDeque::new();

    for l in inp.lines() {
        let (c, r) = l
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect_tuple()
            .unwrap();
        corrupt.push_back((r, c));
    }

    'outer: loop {
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();

        q.push_back((0, 0, vec![]));

        while !q.is_empty() {
            let (r, c, path) = q.pop_front().unwrap();

            if (r, c) == (dim, dim) {
                if !part2 {
                    return path.len().to_string();
                }

                while !path.contains(corrupt.iter().take(n).last().unwrap()) {
                    n += 1;
                }

                continue 'outer;
            }

            if !visited.insert((r, c)) {
                continue;
            }

            for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (nr, nc) = (r + dr, c + dc);

                if corrupt.iter().take(n).contains(&&(nr, nc)) {
                    continue;
                }

                if nr < 0 || nr > dim || nc < 0 || nc > dim {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push((nr, nc));

                q.push_back((nr, nc, new_path));
            }
        }

        let point = corrupt.iter().take(n).last().unwrap();
        return format!("{},{}", point.1, point.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), "22");
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), "6,1");
    }
}
