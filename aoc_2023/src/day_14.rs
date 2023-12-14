use common::{Answer, Solution, Grid};
// use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

pub struct Day14;

impl Solution for Day14 {
    fn name(&self) -> &'static str {
        "Parabolic Reflector Dish"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn rotate_right(grid: VecDeque<VecDeque<char>>) -> VecDeque<VecDeque<char>> {
    let mut new_grid: VecDeque<VecDeque<char>> = VecDeque::new();
    for c in 0..grid[0].len() {
        let mut new_row: VecDeque<char> = VecDeque::new();
        for r in (0..grid.len()).into_iter().rev() {
            new_row.push_back(grid[r][c]);
        }
        new_grid.push_back(new_row);
    }
    return new_grid;
}

// fn grid_pretty_print(grid: &Vec<Vec<char>>) {
//     for r in grid.iter() {
//         println!("{:?}", r.iter().join(""));
//     }
// }

fn roll_rocks(grid: &mut VecDeque<VecDeque<char>>) {
    for r in 1..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 'O' {
                for i in 1.. {
                    if i > r { break }
                    if "O#".contains(grid[r-i][c]) { break }

                    grid[r-i][c] = 'O';
                    grid[r-i+1][c] = '.';
                }
            }
        }
    }
}

fn solve(input: &str, part2: bool) -> u64 {
    let mut g: Grid = input.into();

    let mut cache: HashMap<VecDeque<VecDeque<char>>, u64> = HashMap::new();
    let mut it = 0;

    const CYCLES: u64 = 1000000000;

    if part2 {
        loop {
            it += 1;

            for _ in 0..4 {
                roll_rocks(&mut g.grid);
                g.grid = rotate_right(g.grid);
            }

            if cache.get(&g.grid).is_some() {
                let first_it = cache.get(&g.grid).unwrap();
                it = CYCLES - ((CYCLES - first_it) % (it - first_it));
                cache.clear();
            }

            cache.insert(g.grid.clone(), it as u64);

            if it == CYCLES {
                break;
            }
        }
    } else {
        roll_rocks(&mut g.grid);
    }

    // grid_pretty_print(&grid);

    return g.grid
        .iter()
        .enumerate()
        .map(| (idx, row) |
            row.iter().filter(|c| c == &&'O').count() * (g.grid.len() - idx)
        )
        .sum::<usize>() as u64;

}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 136);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 64);
    }
}