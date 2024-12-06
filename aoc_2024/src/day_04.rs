use common::{grid, grid::Direction, Answer, Solution};

pub struct Day04;

impl Solution for Day04 {
    fn name(&self) -> &'static str {
        "Ceres Search"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, true).into()
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    let g: grid::Grid<char> = input.into();
    let mut ans = 0;

    if part2 {
        for r in 1..g.rows - 1 {
            for c in 1..g.cols - 1 {
                if g.data[r][c] != 'A' {
                    continue;
                }

                let lt = g.data[r - 1][c + 1];
                let rt = g.data[r + 1][c + 1];
                let lb = g.data[r - 1][c - 1];
                let rb = g.data[r + 1][c - 1];

                if !(lt != rb && (lt == 'S' || lt == 'M') && (rb == 'S' || rb == 'M')) {
                    continue;
                }
                if !(rt != lb && (rt == 'S' || rt == 'M') && (lb == 'S' || lb == 'M')) {
                    continue;
                }

                ans += 1
            }
        }

        return ans;
    }

    for (r, row) in g.data.iter().enumerate().map(|(i, v)| (i as isize, v)) {
        for (c, chr) in row.iter().enumerate().map(|(i, v)| (i as isize, v)) {
            if *chr != 'X' {
                continue;
            }

            for (dc, dr) in Direction::all().map(|v| v.as_delta()) {
                if r + dr * 3 < 0 || r + dr * 3 >= g.rows as isize {
                    continue;
                }

                if c + dc * 3 < 0 || c + dc * 3 >= g.cols as isize {
                    continue;
                }

                if g.data[(r + dr) as usize][(c + dc) as usize] == 'M'
                    && g.data[(r + 2 * dr) as usize][(c + 2 * dc) as usize] == 'A'
                    && g.data[(r + 3 * dr) as usize][(c + 3 * dc) as usize] == 'S'
                {
                    ans += 1
                }
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::solve;

    const SAMPLE: &str = indoc::indoc! {"
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 18);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 9);
    }
}
