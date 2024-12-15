use common::{grid::Direction, grid::Grid, Answer, Solution};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub struct Day15;

impl Solution for Day15 {
    fn name(&self) -> &'static str {
        "Warehouse Woes"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, true).into()
    }
}

fn solve(input: &str, part2: bool) -> usize {
    let (mut grid_input, moves) = input.split_once("\n\n").unwrap();

    let mut changed_grid = grid_input.to_string();

    if part2 {
        changed_grid = changed_grid
            .replace("#", "##")
            .replace(".", "..")
            .replace("O", "[]")
            .replace("@", "@.");
        grid_input = changed_grid.as_str();
    }

    let mut grid: Grid<char> = Grid::from(grid_input);

    let (mut r, mut c) = (0..grid.rows)
        .cartesian_product(0..grid.cols)
        .find(|&(r, c)| grid.data[r][c] == '@')
        .unwrap();

    'moves_loop: for m in moves.trim().chars() {
        let dir: Direction = match m {
            '<' => Direction::Left,
            '^' => Direction::Top,
            '>' => Direction::Right,
            'v' => Direction::Bottom,
            _ => continue,
        };

        let mut q = VecDeque::from([(c, r)]);
        let mut pushes = HashSet::new();

        while !q.is_empty() {
            let (c, r) = q.pop_front().unwrap();
            let (nc, nr) = dir.as_coordinate(&c, &r);

            if !pushes.insert((c, r)) {
                continue;
            }

            match grid.data[nr as usize][nc as usize] {
                '#' => continue 'moves_loop,
                'O' => q.push_back((nc as usize, nr as usize)),
                '[' => q.extend([(nc as usize, nr as usize), ((nc + 1) as usize, nr as usize)]),
                ']' => q.extend([(nc as usize, nr as usize), ((nc - 1) as usize, nr as usize)]),
                _ => continue,
            }
        }

        while !pushes.is_empty() {
            for (c, r) in pushes.clone().iter() {
                let (nc, nr) = dir.as_coordinate(c, r);
                if !pushes.contains(&(nc as usize, nr as usize)) {
                    grid.data[nr as usize][nc as usize] = grid.data[*r as usize][*c as usize];
                    grid.data[*r as usize][*c as usize] = '.';
                    pushes.remove(&(*c, *r));
                }
            }
        }

        let (nc, nr) = dir.as_coordinate(&c, &r);
        (c, r) = (nc as usize, nr as usize);
    }

    (0..grid.rows)
        .cartesian_product(0..grid.cols)
        .filter(|&(r, c)| (grid.data[r][c] == '[' && part2) || grid.data[r][c] == 'O')
        .map(|(r, c)| r * 100 + c)
        .sum()
}

#[cfg(test)]

mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        ########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<
    "};

    const SAMPLE2: &str = indoc::indoc! {"
        ##########
        #..O..O.O#
        #......O.#
        #.OO..O.O#
        #..O@..O.#
        #O#..O...#
        #O..O..O.#
        #.OO.O.OO#
        #....O...#
        ##########

        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
    "};

    #[test]
    fn example_part1a() {
        assert_eq!(solve(SAMPLE, false), 2028);
    }

    #[test]
    fn example_part1b() {
        assert_eq!(solve(SAMPLE2, false), 10092);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE2, true), 9021);
    }
}
