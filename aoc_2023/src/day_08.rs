use common::{Answer, Solution};
use std::collections::{HashMap, HashSet};

pub struct Day08;

impl Solution for Day08 {
    fn name(&self) -> &'static str {
        "Camel Cards"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input).into();
    }

    fn part2(&self, input: &str) -> Answer {
        return solve2(input).into();
    }
}

fn solve(input: &str) -> u32 {
    let (cmd, nodes_raw) = input.split_once("\n\n").unwrap();
    let nodes_mod = nodes_raw.replace("(", "").replace(")", "");

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for node in nodes_mod.lines() {
        let (node, dirs) = node.split_once(" = ").unwrap();
        let (l, r) = dirs.split_once(", ").unwrap();
        nodes.insert(node.to_string(), (l.to_string(), r.to_string()));
    }

    let mut current = "AAA".to_string();
    let mut iteration = 0;

    loop {    
        current = match cmd.chars().nth(iteration % cmd.len()).unwrap() {
            'L' => nodes.get(&current).unwrap().0.clone(),
            'R' => nodes.get(&current).unwrap().1.clone(),
            _ => unreachable!("Only left and right possible.")
        };
        iteration += 1;

        if current == "ZZZ" { break }
    }

    return iteration as u32;
}

fn solve2(input: &str) -> u64 {
    let (cmd, nodes_raw) = input.split_once("\n\n").unwrap();
    let nodes_mod = nodes_raw.replace("(", "").replace(")", "");

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();
    for node in nodes_mod.lines() {
        let (node, dirs) = node.split_once(" = ").unwrap();
        let (l, r) = dirs.split_once(", ").unwrap();
        nodes.insert(node.to_string(), (l.to_string(), r.to_string()));
    }

    let ghosts: HashSet<&String> = nodes
        .keys()
        .filter(|k| k.chars().nth(2).unwrap() == 'A')
        .collect();

    let mut results: Vec<usize> = Vec::new();

    for g in ghosts.iter() {
        let mut current = g.to_string();
        let mut iteration = 0;
    
        loop {    
            current = match cmd.chars().nth(iteration % cmd.len()).unwrap() {
                'L' => nodes.get(&current).unwrap().0.clone(),
                'R' => nodes.get(&current).unwrap().1.clone(),
                _ => unreachable!("Only left and right possible.")
            };
            iteration += 1;

            if current.chars().nth(2).unwrap() == 'Z' { break }
        }

        results.push(iteration);
    }
    return lcm(&results) as u64;
}

// SRC: https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::{solve, solve2};

    const SAMPLE: &str = indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE), 6);
    }

    const SAMPLE2: &str = indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    fn example_part2() {
        assert_eq!(solve2(SAMPLE2), 6);
    }
}

