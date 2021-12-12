use std::collections::{HashMap, HashSet};

use adventofcode2021::prelude::*;
use petgraph::dot::{Config, Dot};

fn main() {
    let input = include_str!("../inputs/input12.txt").lines();

    let mut graph = Graph::new_undirected();
    let mut nodes = HashSet::new();
    let mut edges = HashSet::new();
    for s in input {
        let (p1, p2) = s.split_once('-').unwrap();
        nodes.insert(p1);
        nodes.insert(p2);
        edges.insert((p1, p2));
    }
    let nodes = nodes
        .into_iter()
        .map(|s| (s, graph.add_node((s, CaveType::from_str(s)))))
        .collect::<HashMap<_, _>>();
    for (p1, p2) in edges {
        let p1 = nodes[p1];
        let p2 = nodes[p2];
        graph.add_edge(p1, p2, ());
    }

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    let start = nodes["start"];
    let mut visited_small = HashSet::new();
    dbg!(recursedive(start, &graph, &mut visited_small));
}

#[derive(Debug)]
enum CaveType {
    Start,
    End,
    Big,
    Small,
}

impl CaveType {
    fn from_str(s: &str) -> Self {
        match s {
            "start" => Self::Start,
            "end" => Self::End,
            _ if s.chars().all(char::is_lowercase) => Self::Small,
            _ => Self::Big,
        }
    }
}

fn recursedive(
    node: NodeIndex,
    graph: &UnGraph<(&str, CaveType), ()>,
    visited_small: &mut HashSet<NodeIndex>,
) -> usize {
    let mut cnt = 0;
    let mut walker = graph.neighbors(node).detach();
    while let Some((_, n)) = walker.next(&graph) {
        match graph.node_weight(n).unwrap().1 {
            CaveType::Small => {
                if visited_small.insert(n) {
                    cnt += recursedive(n, graph, visited_small);
                    visited_small.remove(&n);
                }
            }
            CaveType::Big => {
                cnt += recursedive(n, graph, visited_small);
            }
            CaveType::End => {
                cnt += 1;
            }
            _ => {}
        }
    }
    cnt
}
