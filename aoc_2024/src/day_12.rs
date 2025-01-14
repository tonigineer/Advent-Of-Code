use common::{
    grid::{Direction, Grid},
    Answer, Solution,
};
use std::collections::{HashSet, VecDeque};

pub struct Day12;

impl Solution for Day12 {
    fn name(&self) -> &'static str {
        "Garden Groups"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input, false).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve(input, true).into()
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    let grid: Grid<char> = Grid::from(input);

    let mut gardens: Vec<HashSet<(usize, usize)>> = Vec::new();

    for r in 0..grid.rows {
        for c in 0..grid.cols {
            if !gardens.iter().any(|g| g.contains(&(c, r))) {
                fill_garden(&mut gardens, &grid, &r, &c);
            }
        }
    }

    if part2 {
        return gardens
            .iter()
            .map(|g| count_corners(g) * g.len() as u32)
            .sum();
    }

    gardens
        .iter()
        .map(|g| count_perimiter(g, &grid) * g.len() as u32)
        .sum()
}

fn count_perimiter(garden: &HashSet<(usize, usize)>, grid: &Grid<char>) -> u32 {
    garden
        .iter()
        .map(|&(c, r)| {
            4 - grid
                .adjacent_cardinals(c, r)
                .iter()
                .filter(|&x| garden.contains(x))
                .count()
        })
        .sum::<usize>() as u32
}

fn count_corners(garden: &HashSet<(usize, usize)>) -> u32 {
    let mut corners = 0;

    for (c, r) in garden {
        for (cardinal1, cardinal2, diagonal) in [
            (Direction::Top, Direction::Right, Direction::TopRight),
            (Direction::Right, Direction::Bottom, Direction::BottomRight),
            (Direction::Bottom, Direction::Left, Direction::BottomLeft),
            (Direction::Left, Direction::Top, Direction::TopLeft),
        ] {
            let mut foreigners = 0;

            // Note: Converting `isize` to `usize` causes negative values to wrap to large positive numbers.
            // These out-of-bounds values will not exist in the HashMap, eliminating the need for explicit in-bounds checks.
            let (nc, nr) = cardinal1.as_coordinate(c, r);
            if !garden.contains(&(nc as usize, nr as usize)) {
                foreigners += 1;
            }

            let (nc, nr) = cardinal2.as_coordinate(c, r);
            if !garden.contains(&(nc as usize, nr as usize)) {
                foreigners += 1;
            }

            if foreigners == 2 {
                corners += 1;
            }

            if foreigners == 0 {
                let (nc, nr) = diagonal.as_coordinate(c, r);
                if !garden.contains(&(nc as usize, nr as usize)) {
                    corners += 1;
                }
            }
        }
    }

    corners
}

fn fill_garden(
    gardens: &mut Vec<HashSet<(usize, usize)>>,
    grid: &Grid<char>,
    r: &usize,
    c: &usize,
) {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((*c, *r));

    let plant = grid.data[*r][*c];
    let mut garden: HashSet<(usize, usize)> = HashSet::new();

    while !q.is_empty() {
        let (c, r) = q.pop_front().unwrap();

        if garden.contains(&(c, r)) {
            continue;
        }

        garden.insert((c, r));

        for (nc, nr) in grid.adjacent_cardinals(c, r) {
            if grid.data[nr][nc] == plant {
                q.push_back((nc, nr));
            }
        }
    }

    gardens.push(garden);
}

#[cfg(test)]

mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        RRRRIICCFF
        RRRRIICCCF
        VVRRRCCFFF
        VVRCCCJFFF
        VVVVCJJCFE
        VVIVCCJJEE
        VVIIICJJEE
        MIIIIIJJEE
        MIIISIJEEE
        MMMISSJEEE
    "};

    #[test]
    fn example_part1() {
        assert_eq!(Day12::part1(&Day12, SAMPLE), common::Answer::Number(1930));
    }

    #[test]
    fn example_part2() {
        assert_eq!(Day12::part2(&Day12, SAMPLE), common::Answer::Number(1206));
    }
}
