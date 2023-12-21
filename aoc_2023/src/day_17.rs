use common::{Answer, Solution, Grid };

use std::collections::{HashSet, BinaryHeap};
use std::cmp::Reverse;

pub struct Day17;

impl Solution for Day17 {
    fn name(&self) -> &'static str {
        "The Floor Will Be Lava"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(input: &str, part2: bool) -> u64 {
    let g: Grid = input.into();

    let mut hq: BinaryHeap<(Reverse<usize>, isize, isize, isize, isize, usize)> = BinaryHeap::new();
    let mut seen: HashSet<(isize, isize, isize, isize, usize)> = HashSet::new();

    hq.push((Reverse(0), 0, 0, 0, 0, 0));

    loop {
        let (hl, r, c, dr, dc, n) = hq.pop().unwrap();

        if (r as usize, c as usize) == (g.grid.len() - 1, g.grid[0].len() - 1) { return hl.0 as u64 }

        if seen.contains(&(r, c, dr, dc, n)) { continue }
        seen.insert((r, c, dr, dc, n));

        if n < {if part2 { 10 } else { 3 }} {
            let nr = r + dr;
            let nc = c + dc;

            if nr >= 0 && nr < g.grid.len() as isize && nc >= 0 &&  nc < g.grid[0].len() as isize {
                hq.push((
                    Reverse(&hl.0 + g.grid[nr as usize][nc as usize].to_digit(10).unwrap() as usize),
                    nr , nc, dr, dc, n + 1
                ))
            }
        }

        if part2 && n < 4 && (dr, dc) != (0, 0) { continue }

        for (ndr, ndc) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if (ndr, ndc) != (dr, dc) && (ndr, ndc) != (-dr, -dc) {  // not back or same again
                let nr: isize = r + ndr;
                let nc = c + ndc;

                if nr >= 0 && nr < g.grid.len() as isize && nc >= 0 &&  nc < g.grid[0].len() as isize {
                    hq.push((
                        Reverse(&hl.0 + g.grid[nr as usize][nc as usize].to_digit(10).unwrap() as usize),
                        nr , nc, ndr, ndc, 1
                    ))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 102);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 94);
    }
}