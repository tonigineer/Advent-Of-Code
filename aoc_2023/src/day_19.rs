use common::{ Answer, Solution };

pub struct Day19;

impl Solution for Day19 {
    fn name(&self) -> &'static str {
        "Lavaduct Lagoon"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(_input: &str, _part2: bool) -> u64 {
    return 0
}

// #[cfg(test)]
// mod tests {
//     use indoc::indoc;
//     use super::solve;

//     const SAMPLE: &str = indoc! {"2413432311323
// 3215453535623
// 3255245654254
// 3446585845452
// 4546657867536
// 1438598798454
// 4457876987766
// 3637877979653
// 4654967986887
// 4564679986453
// 1224686865563
// 2546548887735
// 4322674655533"};

//     #[test]
//     fn example_part1() {
//         assert_eq!(solve(SAMPLE, false), 102);
//     }

//     #[test]
//     fn example_part2() {
//         assert_eq!(solve(SAMPLE, true), 94);
//     }
// }