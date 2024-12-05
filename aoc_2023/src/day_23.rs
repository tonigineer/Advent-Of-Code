use common::{grid::Grid, Answer, Solution};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day23;

impl Solution for Day23 {
    fn name(&self) -> &'static str {
        "A Long Walk"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(input: &str, part2: bool) -> u64 {
    let g: Grid<char> = input.into();

    let sr = 0;
    let sc = g.data[sr].iter().position(|&r| r == '.').unwrap();

    let er = g.data.len() - 1;
    let ec = g.data[er].iter().position(|&r| r == '.').unwrap();

    let mut junctions = HashSet::<(usize, usize)>::new();
    junctions.insert((sr, sc));
    junctions.insert((er, ec));

    for (r, row) in g.data.iter().enumerate() {
        for (c, ch) in row.iter().enumerate() {
            if ch == &'#' {
                continue;
            }
            let mut neighbors = 0;
            for (nr, nc) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
                if nr >= g.data.len() || nc >= g.data[0].len() {
                    continue;
                }
                if g.data[nr][nc] == '#' {
                    continue;
                }
                neighbors += 1;
            }
            if neighbors >= 3 {
                junctions.insert((r, c));
            }
        }
    }

    let mut graph = HashMap::<(usize, usize), HashMap<(usize, usize), usize>>::new();
    for p in junctions.iter() {
        graph.insert((p.0, p.1), HashMap::new());
    }

    for (sr, sc) in junctions.iter() {
        let mut stack = VecDeque::from([(*sr, *sc, 0)]);
        let mut seen = HashSet::<(usize, usize)>::new();
        seen.insert((*sr, *sc));

        while !stack.is_empty() {
            let (r, c, dist) = stack.pop_front().unwrap();

            if dist != 0 && junctions.contains(&(r, c)) {
                graph.entry((*sr, *sc)).and_modify(|hm| {
                    hm.insert((r, c), dist);
                });
                continue;
            }

            for (nr, nc) in directions(&g, (r, c), part2) {
                if nr >= g.data.len() || nc >= g.data[0].len() {
                    continue;
                }
                if g.data[nr][nc] == '#' {
                    continue;
                }
                if seen.contains(&(nr, nc)) {
                    continue;
                }

                stack.push_back((nr, nc, dist + 1));
                seen.insert((nr, nc));
            }
        }
    }

    return dfs(
        &graph,
        &mut HashSet::<(usize, usize)>::new(),
        &(er, ec),
        &(sr, sc),
    );
}

fn dfs(
    graph: &HashMap<(usize, usize), HashMap<(usize, usize), usize>>,
    seen: &mut HashSet<(usize, usize)>,
    end: &(usize, usize),
    pos: &(usize, usize),
) -> u64 {
    if pos == end {
        return 0;
    }

    let mut m = u64::MIN;
    seen.insert(*pos);

    for (next, dist) in graph[pos].iter() {
        if seen.contains(next) {
            continue;
        }
        m = m.max(dfs(&graph, seen, end, &next) + *dist as u64)
    }
    seen.remove(pos);

    return m;
}

fn directions(g: &Grid<char>, position: (usize, usize), part2: bool) -> Vec<(usize, usize)> {
    let r = position.0;
    let c = position.1;

    if part2 {
        return [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)].to_vec();
    }

    return match g.data[position.0][position.1] {
        '.' => [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)].to_vec(),
        '^' => [(r - 1, c)].to_vec(),
        'v' => [(r + 1, c)].to_vec(),
        '<' => [(r, c - 1)].to_vec(),
        '>' => [(r, c + 1)].to_vec(),
        '#' => [].to_vec(),
        _ => panic!("Not supposed to happen, ffs"),
    };
}

// #[cfg(test)]
// mod tests {
//     use indoc::indoc;
//     use super::solve;

//     const SAMPLE: &str = indoc! {
// "#.#####################
// #.......#########...###
// #######.#########.#.###
// ###.....#.>.>.###.#.###
// ###v#####.#v#.###.#.###
// ###.>...#.#.#.....#...#
// ###v###.#.#.#########.#
// ###...#.#.#.......#...#
// #####.#.#.#######.#.###
// #.....#.#.#.......#...#
// #.#####.#.#.#########v#
// #.#...#...#...###...>.#
// #.#.#v#######v###.###v#
// #...#.>.#...>.>.#.###.#
// #####v#.#.###v#.#.###.#
// #.....#...#...#.#.#...#
// #.#########.###.#.#.###
// #...###...#...#...#.###
// ###.###.#.###v#####v###
// #...#...#.#.>.>.#.>.###
// #.###.###.#.###.#.#v###
// #.....###...###...#...#
// #####################.#"};

//     #[test]
//     fn example_part1() {
//         assert_eq!(solve(SAMPLE, false), 94);
//     }

//     #[test]
//     fn example_part2() {
//         assert_eq!(solve(SAMPLE, true), 154);
//     }
// }
