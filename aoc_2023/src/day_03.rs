use common::{Answer, Solution, Grid};
use std::collections::HashSet;

pub struct Day03;

impl Solution for Day03 {
    fn name(&self) -> &'static str {
        "Gear Ratios"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve_part2(input).into();
    }
}


fn solve(input: &str) -> u32 {
    let g: Grid = input.into();
    let mut coordinates: HashSet<(usize, usize)> = HashSet::new();

    for (r, row) in g.grid.iter().enumerate() {
        for (c, chr) in row.iter().enumerate() {
            if chr.is_ascii_digit() || chr == &'.' { continue }

            for dr in [r-1, r, r+1].into_iter() {
                for mut dc in [c-1, c, c+1].into_iter() {
                    if dr >= g.grid.len() || dc >= row.len() { continue }

                    if g.grid[dr][dc].is_ascii_digit() {
                        while dc > 0 && g.grid[dr][dc - 1].is_ascii_digit() {
                            dc -= 1;
                        }
                        coordinates.insert((dr, dc));
                     }
                }
            }
        }
    }

    let mut result = 0;
    for (r, mut c) in coordinates.iter() {
        let mut number: String = String::new();
        while c < g.grid[*r].len() && g.grid[*r][c].is_ascii_digit() {
            number.push_str(&g.grid[*r][c].to_string());
            c += 1;
        }
        result += number.parse::<u32>().unwrap();
    }
    return result;
}

fn solve_part2(input: &str) -> u32 {
    let g: Grid = input.into();
    let mut coordinates: Vec<HashSet<(usize, usize)>> = Vec::new();

    for (r, row) in g.grid.iter().enumerate() {
        for (c, chr) in row.iter().enumerate() {
            if chr != &'*' { continue }

            let mut pair: HashSet<(usize, usize)> = HashSet::new();
            for dr in [r-1, r, r+1].into_iter() {
                for mut dc in [c-1, c, c+1].into_iter() {
                    if dr >= g.grid.len() || dc >= row.len() { continue }

                    if g.grid[dr][dc].is_ascii_digit() {
                        while dc > 0 && g.grid[dr][dc - 1].is_ascii_digit() {
                            dc -= 1;
                        }
                        pair.insert((dr, dc));
                     }
                }
            }
            if pair.len() == 2 {
                coordinates.push(pair);
            }
        }
    }

    let mut result = 0;
    for pair in coordinates.iter() {
        let mut ratio = 1;
        for (r, mut c) in pair.iter() {
            let mut number: String = String::new();
            while c < g.grid[*r].len() && g.grid[*r][c].is_ascii_digit() {
                number.push_str(&g.grid[*r][c].to_string());
                c += 1;
            }
            ratio *= number.parse::<u32>().unwrap();
        }
        result += ratio;
    }

    return result;
}



#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::{solve, solve_part2};

    const SAMPLE: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE), 4361);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(SAMPLE), 467835);
    }
}