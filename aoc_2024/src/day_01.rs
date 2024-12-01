use common::{Answer, Solution};

pub struct Day01;

impl Solution for Day01 {
    fn name(&self) -> &'static str {
        "Historian Hysteria"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input, false).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve(input, true).into();
    }
}

fn solve(input: &str, part2: bool) -> u32 {
    let num_lines = input.lines().count();

    let mut left: Vec<u32> = Vec::with_capacity(num_lines);
    let mut right: Vec<u32> = Vec::with_capacity(num_lines);

    for line in input.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse::<u32>().unwrap());
        right.push(r.parse::<u32>().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    if part2 {
        return left
            .iter()
            .map(|l| *l * right.iter().filter(|v| **v == *l).count() as u32)
            .sum::<u32>();
    }

    return left
        .into_iter()
        .zip(right)
        .map(|(l, r)| if l > r { l - r } else { r - l })
        .sum::<u32>();
}

#[cfg(test)]
mod tests {
    use super::solve;

    const SAMPLE: &str = indoc::indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE, false), 11);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve(SAMPLE, true), 31);
    }
}
