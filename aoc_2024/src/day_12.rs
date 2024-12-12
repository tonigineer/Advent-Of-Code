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
            .map(|g| count_sides(g, &grid) * g.len() as u32)
            .sum();
    }

    gardens
        .iter()
        .map(|g| count_perimiter(g, &grid) * g.len() as u32)
        .sum()
}

fn count_perimiter(garden: &HashSet<(usize, usize)>, grid: &Grid<char>) -> u32 {
    let mut perimiter = 0;

    for (c, r) in garden {
        perimiter += 4; // Adjancent_cardinals does not provide the point outside the grid

        for (dc, dr) in grid.adjacent_cardinals(*c, *r) {
            if garden.contains(&(dc, dr)) {
                perimiter -= 1;
            }
        }
    }

    perimiter
}

fn count_sides(garden: &HashSet<(usize, usize)>, grid: &Grid<char>) -> u32 {
    let mut corners = 0;

    for (c, r) in garden {
        for (cardinal1, cardinal2, diagonal) in [
            (Direction::Top, Direction::Right, Direction::TopRight),
            (Direction::Right, Direction::Bottom, Direction::BottomRight),
            (Direction::Bottom, Direction::Left, Direction::BottomLeft),
            (Direction::Left, Direction::Top, Direction::TopLeft),
        ] {
            let (dc2, dr2) = cardinal1.as_delta();
            let (dc3, dr3) = cardinal2.as_delta();

            let mut foreigners = 2;

            if grid.in_bounds(*c as isize + dc2, *r as isize + dr2) {
                let cc = *c as isize + dc2;
                let rr = *r as isize + dr2;
                if garden.contains(&(cc as usize, rr as usize)) {
                    foreigners -= 1;
                }
            }

            if grid.in_bounds(*c as isize + dc3, *r as isize + dr3) {
                let cc = *c as isize + dc3;
                let rr = *r as isize + dr3;
                if garden.contains(&(cc as usize, rr as usize)) {
                    foreigners -= 1;
                }
            }

            if foreigners == 2 {
                corners += 1;
            }

            if foreigners == 0 {
                let (dc4, dr4) = diagonal.as_delta();
                if grid.in_bounds(*c as isize + dc4, *r as isize + dr4) {
                    let cc = *c as isize + dc4;
                    let rr = *r as isize + dr4;

                    if !garden.contains(&(cc as usize, rr as usize)) {
                        corners += 1;
                    }
                }
            }
        }
    }

    corners as u32
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
