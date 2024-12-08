use common::{grid::Grid, Answer, Solution};
use std::collections::{HashMap, HashSet};

pub struct Day08;

impl Solution for Day08 {
    fn name(&self) -> &'static str {
        "Resonant Collinearity"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, true).into()
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    let grid: Grid<char> = input.into();
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if grid.data[r][c] == '.' {
                continue;
            }

            antennas
                .entry(grid.data[r][c])
                .and_modify(|v| v.push((c as isize, r as isize)))
                .or_insert(vec![(c as isize, r as isize)]);
        }
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    if part2 {
        for locations in antennas.values() {
            for (c1, r1) in locations.iter() {
                for (c2, r2) in locations.iter() {
                    if (c1, r1) == (c2, r2) {
                        continue;
                    }

                    // NOTE: Since each pair is looped twice, only one direction
                    // is checked here.
                    let mut c = c1.clone();
                    let mut r = r1.clone();
                    let dr = r2 - r1;
                    let dc = c2 - c1;

                    while c >= 0 && c < (grid.cols as isize) && r >= 0 && r < (grid.rows as isize) {
                        antinodes.insert((c.clone(), r.clone()));
                        c = dc + c;
                        r = dr + r;
                    }
                }
            }
        }

        return antinodes.len() as u32;
    }

    for locations in antennas.values() {
        for (i, (c1, r1)) in locations.into_iter().enumerate() {
            for (c2, r2) in locations.into_iter().skip(i + 1) {
                antinodes.insert((2 * c1 - c2, 2 * r1 - r2));
                antinodes.insert((2 * c2 - c1, 2 * r2 - r1));
            }
        }
    }

    antinodes
        .iter()
        .filter(|(c, r)| {
            c >= &0 && c < &(grid.cols as isize) && r >= &0 && r < &(grid.rows as isize)
        })
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::solve;

    const SAMPLE: &str = indoc::indoc! {"
        ............
        ........0...
        .....0......
        .......0....
        ....0.......
        ......A.....
        ............
        ............
        ........A...
        .........A..
        ............
        ............
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 14);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 34);
    }
}
