use common::{Answer, Solution};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day23;

impl Solution for Day23 {
    fn name(&self) -> &'static str {
        "LAN Party"
    }

    fn part1(&self, input: &str) -> Answer {
        solve(input).into()
    }

    fn part2(&self, input: &str) -> Answer {
        solve2(input).into()
    }
}

// Solution taken from
fn bron_kerbosch<'a>(
    connections: &HashMap<&'a str, HashSet<&'a str>>,
    required: &mut HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    cliques: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() {
        if x.is_empty() {
            cliques.push(required.clone());
        }
        return;
    }

    while let Some(n) = p.iter().copied().next() {
        let neighbours = &connections[n];
        let p2 = p.intersection(neighbours).copied().collect();
        let x2 = x.intersection(neighbours).copied().collect();

        required.insert(n);

        bron_kerbosch(connections, required, p2, x2, cliques);

        required.remove(n);
        p.remove(n);
        x.insert(n);
    }
}

fn solve2(input: &str) -> String {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (left, right) = line.split_once("-").unwrap();
        connections.entry(left).or_default().insert(right);
        connections.entry(right).or_default().insert(left);
    }

    let mut cliques = Vec::new();

    bron_kerbosch(
        &connections,
        &mut HashSet::new(),
        connections.keys().copied().collect(),
        HashSet::new(),
        &mut cliques,
    );
    cliques
        .iter()
        .max_by_key(|c| c.len())
        .unwrap()
        .iter()
        .sorted()
        .join(",")
}

fn solve(input: &str) -> usize {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (left, right) = line.split_once("-").unwrap();
        connections.entry(left).or_default().insert(right);
        connections.entry(right).or_default().insert(left);
    }

    let mut triplets = HashSet::new();

    for &first in connections.keys() {
        if !first.starts_with('t') {
            continue;
        }

        for &second in &connections[first] {
            for &third in connections[first].intersection(&connections[second]) {
                let mut triplet = [first, second, third];
                triplet.sort();
                triplets.insert(triplet);
            }
        }
    }

    triplets.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = indoc::indoc! {"
        kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn
    "};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE), 7);
    }

    #[test]
    fn example_part2() {
        assert_eq!(solve2(SAMPLE), "co,de,ka,ta");
    }
}
