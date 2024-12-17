use common::{Answer, Solution};
use itertools::Itertools;

pub struct Day17;

impl Solution for Day17 {
    fn name(&self) -> &'static str {
        "Chronospatial Computer"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve2(input).into()
    }
}

fn solve(input: &str) -> String {
    let mut computer = input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|w| !w.is_empty())
        .map(|w| w.parse::<usize>().unwrap());

    let (mut a, mut b, mut c) = computer.next_tuple().unwrap();
    let instructions = computer.collect::<Vec<_>>();

    let mut pointer = 0;
    let mut ans = Vec::new();

    while pointer < instructions.len() {
        let opcode = instructions[pointer];
        let operand = instructions[pointer + 1];
        let combo = [operand, operand, operand, operand, a, b, c, 0][operand];

        match opcode {
            0 => a = a >> combo,
            1 => b = b ^ operand,
            2 => b = combo % 8,
            3 => {
                if a != 0 {
                    pointer = operand;
                    continue;
                }
            }
            4 => b = b ^ c,
            5 => ans.push(combo % 8),
            6 => b = a >> combo,
            7 => c = a >> combo,
            _ => unreachable!(),
        }
        pointer += 2;
    }

    ans.iter().join(",")
}

fn solve2(input: &str) -> usize {
    let mut computer = input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter(|w| !w.is_empty())
        .map(|w| w.parse::<usize>().unwrap());

    let (_, _, _) = computer.next_tuple().unwrap();
    let instructions = computer.collect::<Vec<_>>();

    reverse_engineered(instructions, 0).unwrap()
}

fn reverse_engineered(inst: Vec<usize>, ans: usize) -> Option<usize> {
    if inst.len() == 0 {
        return Some(ans);
    }

    for i in 0..8 {
        // Needed operations are derived from the input instructions.
        let a = (ans << 3) + i;
        let mut b = a % 8;
        b = b ^ 2;
        let c = a >> b;
        b = b ^ c;
        b = b ^ 7;
        if b % 8 == inst[inst.len() - 1] {
            if let Some(sub) = reverse_engineered(inst[0..inst.len() - 1].to_vec(), a) {
                return Some(sub);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE), "4,6,3,5,6,3,5,2,1,0");
    }

    //     const SAMPLE2: &str = indoc::indoc! {"
    //         Register A: 2024
    //         Register B: 0
    //         Register C: 0

    //         Program: 0,3,5,4,3,0
    // "};

    //     #[test]
    //     fn example_part2() {
    //         assert_eq!(solve2(SAMPLE), 117440);
    //     }
}
