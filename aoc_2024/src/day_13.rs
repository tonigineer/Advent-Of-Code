use common::{Answer, Solution};
use itertools::Itertools;

pub struct Day13;

impl Solution for Day13 {
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

fn solve(input: &str, part2: bool) -> u64 {
    let mut token = 0;
    for game in input.split("\n\n") {
        let (ax, ay, bx, by, mut rx, mut ry) = game
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i64>().unwrap())
            .collect_tuple()
            .unwrap();

        if part2 {
            rx += 10_000_000_000_000;
            ry += 10_000_000_000_000;
        }
        let (a, b) = solve_lineq(ax, bx, ay, by, rx, ry);
        token += 3 * a + b;
    }

    token as u64
}

fn solve_lineq(x1: i64, y1: i64, x2: i64, y2: i64, z1: i64, z2: i64) -> (i64, i64) {
    // Cramer's Rule
    //  Multiply:
    //      x1 * a + y1 * b = z1;  * x2
    //      x2 * a + y2 * b = z2;  * x1
    //  Substract:
    //      x2 * x1 * a + x2 * y1 * b = x2 * z1  -  x1 * x2 * a + x1 * y2 * b = x1 * z2
    //      (x1 * y2 - x2 * y1) * b = x1 * z2 - x2 * z1

    let b = (z2 * x1 - z1 * x2) / (y2 * x1 - y1 * x2);
    let a = (z1 - b * y1) / x1;

    if (x1 * a + y1 * b, x2 * a + y2 * b) != (z1, z2) {
        return (0, 0);
    }

    (a, b)
}

#[cfg(test)]

mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Button A: X+26, Y+66
        Button B: X+67, Y+21
        Prize: X=12748, Y=12176

        Button A: X+17, Y+86
        Button B: X+84, Y+37
        Prize: X=7870, Y=6450

        Button A: X+69, Y+23
        Button B: X+27, Y+71
        Prize: X=18641, Y=10279
    "};

    #[test]
    fn example_part1() {
        assert_eq!(Day13::part1(&Day13, SAMPLE), common::Answer::Number(480));
    }

    #[test]
    fn example_part2() {
        assert_eq!(
            Day13::part2(&Day13, SAMPLE),
            common::Answer::Number(875318608908)
        );
    }
}
