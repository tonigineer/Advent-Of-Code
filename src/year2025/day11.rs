//! # Reactor
//!
//

use std::collections::{HashMap, VecDeque};

pub fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (entry, outs) = l.split_once(": ").unwrap();
            let outs = outs.split_whitespace().collect::<Vec<&str>>();

            (entry, outs)
        })
        .collect()
}

pub fn part1(nodes: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut result = 0;

    let start = "you";
    let mut q = VecDeque::from([start]);

    while let Some(n) = q.pop_front() {
        if n == "out" {
            result += 1;
            continue;
        }

        nodes.get(n).unwrap().iter().for_each(|o| q.push_back(o));
    }

    result
}

pub fn part2(nodes: &HashMap<&str, Vec<&str>>) -> u64 {
    let start = "svr";
    let mut seen = HashMap::new();
    let visited = [false, false];

    solve(&mut seen, nodes, start, visited)
}

fn solve<'a>(
    seen: &mut HashMap<(&'a str, [bool; 2]), u64>,
    nodes: &HashMap<&'a str, Vec<&'a str>>,
    node: &'a str,
    mut visited: [bool; 2],
) -> u64 {
    if node == "out" {
        if visited[0] && visited[1] {
            return 1;
        }
        return 0;
    }

    if node == "fft" {
        visited[0] = true;
    }
    if node == "dac" {
        visited[1] = true;
    }

    if let Some(&r) = seen.get(&(node, visited)) {
        return r;
    }

    let result = nodes.get(node).unwrap().iter().map(|n| solve(seen, nodes, n, visited)).sum();
    seen.insert((node, visited), result);

    result
}
