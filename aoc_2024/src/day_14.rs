use common::{Answer, Solution};
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day14;

impl Solution for Day14 {
    fn name(&self) -> &'static str {
        "Restroom Redoubt"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, 101, 103, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, 101, 103, true).into()
    }
}

fn solve(input: &str, width: isize, height: isize, part2: bool) -> usize {
    let robots = input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|w| !w.is_empty())
        .map(|w| w.parse::<isize>().unwrap())
        .tuples()
        .collect::<Vec<_>>();

    let mut tiles: HashMap<(isize, isize), usize> = HashMap::new();

    if !part2 {
        for (x, y, vx, vy) in robots.iter() {
            tiles
                .entry((
                    (x + vx * 100).rem_euclid(width),
                    (y + vy * 100).rem_euclid(height),
                ))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        let mut quadrants: Vec<usize> = vec![0, 0, 0, 0];

        for (tile, num_robots) in tiles.into_iter() {
            if tile.0 == width / 2 || tile.1 == height / 2 {
                continue;
            }

            let x = (tile.0 < width / 2) as usize;
            let y = (tile.1 < height / 2) as usize;

            quadrants[x * 2 + y] += num_robots;
        }

        return quadrants.iter().product();
    }

    for i in 1..100_000 {
        tiles.clear();

        for (x, y, vx, vy) in robots.iter() {
            tiles
                .entry((
                    (x + vx * i).rem_euclid(width),
                    (y + vy * i).rem_euclid(height),
                ))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        if tiles.values().all(|&v| v == 1) {
            return i as usize;
        }
    }

    panic!("Something went wrong.");
}

#[cfg(test)]

mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        p=0,4 v=3,-3
        p=6,3 v=-1,-3
        p=10,3 v=-1,2
        p=2,0 v=2,-1
        p=0,0 v=1,3
        p=3,0 v=-2,-2
        p=7,6 v=-1,-3
        p=3,0 v=-1,-2
        p=9,3 v=2,3
        p=7,3 v=-1,2
        p=2,4 v=2,-3
        p=9,5 v=-3,-3
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, 11, 7, false), 12);
    }
}
