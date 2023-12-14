use common::{Answer, Solution, Grid};

pub struct Day13;

impl Solution for Day13 {
    fn name(&self) -> &'static str {
        "Point of Incidence"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn reflects_at(g: &Grid, part2: bool) -> Option<usize> {
    (1..g.grid.len()).find(|&r|{
        let above = g.grid.iter().take(r).rev();
        let below = g.grid.iter().skip(r);

        if part2 {
            let differences: usize = above.zip(below)
                .map(
                    |(row1, row2)| row1.iter().zip(row2.iter())
                        .filter(|(a, b)| a != b).count()
                )
                .sum();
            differences == 1
        } else {
            above.zip(below)
                .all(|(row1, row2)|row1 == row2)
        }
    })
}

fn solve(input: &str, part2: bool) -> u64 {
    let mut ans = 0;

    for block in input.split("\n\n") {
        let mut grid: Grid = block.into();
        // grid.pretty_print();
        if let Some(i) = reflects_at(&grid, part2) { ans += i * 100 }
        grid.transpose();
        if let Some(i) = reflects_at(&grid, part2) { ans += i }
    }

    return ans as u64;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 405);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 400);
    }
}