use common::{grid::Grid, Answer, Solution};
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};

pub struct Day16;

impl Solution for Day16 {
    fn name(&self) -> &'static str {
        "Reindeer Maze"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, true).into()
    }
}

fn solve(input: &str, part2: bool) -> usize {
    let grid: Grid<char> = Grid::from(input);

    let (r, c) = (0..grid.rows)
        .cartesian_product(0..grid.cols)
        .find(|&(r, c)| grid.data[r][c] == 'S')
        .unwrap();

    let mut pq: PriorityQueue<(isize, isize, isize, isize, Vec<(isize, isize)>), isize> =
        PriorityQueue::new();
    pq.push((r as isize, c as isize, 0, 1, Vec::new()), 0);

    let mut seen = HashMap::new();
    let mut tiles = HashSet::new();

    let mut ans = 0;

    while let Some((task, priority)) = pq.pop() {
        let (r, c, dr, dc, path) = task;

        if seen.contains_key(&(r, c)) {
            let v = seen.get(&(r, c)).unwrap();
            if *v < path.len() {
                continue;
            }
        }

        seen.insert((r, c), path.len());

        if grid.data[r as usize][c as usize] == 'E' {
            if !part2 {
                return (priority * -1) as usize;
            }

            if ans == 0 {
                ans = path.len();
            }

            if ans == path.len() {
                for tile in path.clone().into_iter() {
                    tiles.insert(tile);
                }
            } else {
                return tiles.len() + 1; // add start position
            }
        }

        for (ndr, ndc) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let (nr, nc) = (r + ndr, c + ndc);

            if grid.data[nr as usize][nc as usize] != '#' {
                let mut path_new = path.clone();
                path_new.push((nr, nc));

                pq.push(
                    (nr, nc, ndr, ndc, path_new),
                    priority + if (dr, dc) == (ndr, ndc) { -1 } else { -1001 },
                );
            }
        }
    }

    unreachable!()
}

#[cfg(test)]

mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        ###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############
    "};

    const SAMPLE2: &str = indoc::indoc! {"
        #################
        #...#...#...#..E#
        #.#.#.#.#.#.#.#.#
        #.#.#.#...#...#.#
        #.#.#.#.###.#.#.#
        #...#.#.#.....#.#
        #.#.#.#.#.#####.#
        #.#...#.#.#.....#
        #.#.#####.#.###.#
        #.#.#.......#...#
        #.#.###.#####.###
        #.#.#...#.....#.#
        #.#.#.#####.###.#
        #.#.#.........#.#
        #.#.#.#########.#
        #S#.............#
        #################
    "};

    #[test]
    fn example_part1a() {
        assert_eq!(solve(SAMPLE, false), 7036);
    }

    #[test]
    fn example_part1b() {
        assert_eq!(solve(SAMPLE2, false), 11048);
    }

    #[test]
    fn example_part2a() {
        assert_eq!(solve(SAMPLE, true), 45);
    }
}
