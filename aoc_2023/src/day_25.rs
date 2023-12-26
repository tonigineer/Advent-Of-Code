use common::{ Answer, Solution };

use std::collections::{HashSet, HashMap};
// use std::fs::File;
// use std::io::Write;

use petgraph::graph::{NodeIndex, UnGraph};
// use petgraph::dot::{Dot, Config};

use rustworkx_core;
use rustworkx_core::connectivity::stoer_wagner_min_cut;

pub struct Day25;

impl Solution for Day25 {
    fn name(&self) -> &'static str {
        "Snowverload"
    }

    fn part1(&self, input: &str) -> Answer {
        return solve(input).into();
    }

    fn part2(&self, _input: &str) -> Answer {
        return "NO PUZZLE".into();
    }
}

fn solve(input: &str) -> usize {
    let mut graph:  UnGraph::<&str, u32> = UnGraph::default();

    let mut nodes = HashSet::<&str>::new();
    let mut node_ids = HashMap::<&str, NodeIndex>::new();
    let mut edges = HashSet::<(&str, &str)>::new();

    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        nodes.insert(left);
        for node in right.split(" ") {
            nodes.insert(node);
            edges.insert((left, node));
        }
    }

    for (idx, node) in nodes.iter().enumerate() {
        node_ids.insert(node, NodeIndex::new(idx));
        graph.add_node(node);
    }

    for edge in edges.iter() {
        graph.add_edge(*node_ids.get(edge.0).unwrap(), *node_ids.get(edge.1).unwrap(), 1);
    }

    // let dot_txt = format!(
    //     "{:?}",
    //     Dot::with_config(&graph, &[Config::EdgeNoLabel])
    // );
    // let mut file = File::create("graph.dot").unwrap();
    // file.write_all(dot_txt.as_bytes()).unwrap();

    // VS-Code extension: Graphviz Interactive Preview

    let cut: rustworkx_core::Result<Option<(usize, Vec<NodeIndex>)>,> = stoer_wagner_min_cut(
        &graph, |_| Ok(1)
    );
    let (cut_size, nodes_in_partition) = cut.unwrap().unwrap();

    assert_eq!(cut_size, 3);
    return (nodes.len() - nodes_in_partition.len()) * nodes_in_partition.len();

    // return 0;
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::solve;

    const SAMPLE: &str = indoc! {"jqt: rhn xhk nvd
    rsh: frs pzl lsr
    xhk: hfx
    cmg: qnr nvd lhk bvb
    rhn: xhk bvb hfx
    bvb: xhk hfx
    pzl: lsr hfx nvd
    qnr: nvd
    ntq: jqt hfx bvb xhk
    nvd: lhk
    lsr: lhk
    rzs: qnr cmg lsr rsh
    frs: qnr lhk lsr"};

    #[test]
    fn example_part1() {
        assert_eq!(solve(SAMPLE), 54);
    }
}