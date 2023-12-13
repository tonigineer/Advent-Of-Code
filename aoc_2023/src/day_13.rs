use common::{Answer, Solution};

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

fn find_mirror(grid: Vec<Vec<char>>, part2: bool) -> usize {
    for r in 1..grid.len() {
        let mut above = &grid[..r];
        let mut below = &grid[r..];

        // ffs, how to reverse above as in python with [::-1]?
        let mut aa: Vec<Vec<char>> = Vec::new();
        for a in above.into_iter().rev() {
            aa.push(a.to_vec());
        }
        above = &aa[..];

        above = &above[..below.len().min(above.len())];
        below = &below[..above.len().min(below.len())];
    
        if part2 {
            let mut diffs = 0;
            for r in 0..above.len() {
                for c in 0..above[0].len() {
                    if above[r][c] != below[r][c] {
                        diffs += 1;
                    }
                }
            }
            if diffs == 1 { return r }
            continue
        }

        if above.eq(below) { return r }
    }
    return 0;
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn solve(input: &str, part2: bool) -> u64 {
    let mut result = 0;

    for block in input.split("\n\n") {
        let grid = common::string_to_grid(block);

        result += find_mirror(grid.clone(), part2) * 100;
        let grid_transposed = transpose2(grid.clone());
        result += find_mirror(grid_transposed, part2);

    }
    return result as u64;
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