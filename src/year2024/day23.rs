//! LAN Party
//!
//! Summary: Since the input is already structured in a certain `cluster`, the
//! second part can be solved with a primitve approach. The intended approach,
//! according to the Reddit seems to be [Bron–Kerbosch algorithm](https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm).
//! Building indices from two lettered names for direct indexing.

use hashbrown::HashMap;

type ParsedInput = (HashMap<usize, Vec<usize>>, Vec<[bool; 676]>);

fn as_usize(u: u8) -> usize {
    (u - b'a').into()
}

fn as_char(u: usize) -> char {
    ((u as u8) + b'a').into()
}

pub fn parse(input: &str) -> ParsedInput {
    let mut nodes: HashMap<_, Vec<_>> = HashMap::new();
    let mut edges = vec![[false; 676]; 676];

    let to_index = |b: &[u8]| as_usize(b[0]) * 26 + as_usize(b[1]);

    for edge in input.trim().as_bytes().chunks(6) {
        let from = to_index(&edge[0..2]);
        let to = to_index(&edge[3..]);

        nodes.entry(from).or_insert_with(Vec::new).push(to);
        nodes.entry(to).or_insert_with(Vec::new).push(from);

        edges[from][to] = true;
        edges[to][from] = true;
    }

    (nodes, edges)
}

pub fn part1(input: &ParsedInput) -> u32 {
    let (nodes, edges) = input;

    let mut triangles = 0;
    let mut seen = vec![false; 676];

    for node1 in 494..520 {
        if let Some(connections) = nodes.get(&node1) {
            seen[node1] = true;

            for (idx, &node2) in connections.iter().enumerate() {
                for &node3 in connections.iter().skip(idx) {
                    if !seen[node2] && !seen[node3] && edges[node2][node3] {
                        triangles += 1;
                    }
                }
            }
        }
    }

    triangles
}

pub fn part2(input: &ParsedInput) -> String {
    let (nodes, edges) = input;

    let mut cluster = Vec::new();
    let mut largest_cluster = Vec::new();
    let mut seen = vec![false; 676];

    for (&node1, connections) in nodes {
        if !seen[node1] {
            cluster.clear();
            cluster.push(node1);

            for node2 in connections {
                if cluster.iter().all(|&n| edges[*node2][n]) {
                    cluster.push(*node2);
                    seen[*node2] = true;
                }
            }

            if cluster.len() > largest_cluster.len() {
                largest_cluster = cluster.clone();
            }
        }
    }

    largest_cluster.sort_unstable();

    largest_cluster
        .iter()
        .map(|node| format!("{}{}", as_char(node / 26), as_char(node % 26)))
        .collect::<Vec<_>>()
        .join(",")
}
