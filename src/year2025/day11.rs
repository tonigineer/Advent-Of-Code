//! # Reactor
//!
// TODO: Avoid using string slices.

use std::collections::HashMap;

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
    dfs(&mut HashMap::new(), nodes, "you", true, true)
}

pub fn part2(nodes: &HashMap<&str, Vec<&str>>) -> u64 {
    dfs(&mut HashMap::new(), nodes, "svr", false, false)
}

fn dfs<'a>(
    seen: &mut HashMap<(&'a str, bool, bool), u64>,
    nodes: &HashMap<&'a str, Vec<&'a str>>,
    node: &'a str,
    mut fft: bool,
    mut dac: bool,
) -> u64 {
    if node == "out" {
        if fft && dac {
            return 1;
        }
        return 0;
    }

    if let Some(&r) = seen.get(&(node, fft, dac)) {
        return r;
    }

    fft |= node == "fft";
    dac |= node == "dac";

    let result = nodes.get(node).unwrap().iter().map(|n| dfs(seen, nodes, n, fft, dac)).sum();
    seen.insert((node, fft, dac), result);

    result
}
