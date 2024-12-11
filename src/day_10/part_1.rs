use iter_tools::Itertools;
use ndarray::Array2;
use petgraph::dot::Dot;
use petgraph::graph::DiGraph;
use petgraph::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use wyhash2::WyHash;

pub fn solve(input: &str) -> usize {
    let (graph, _, nines) = parse_graph(input, false);

    let mut trailheads_reaching: HashMap<_, HashSet<_, WyHash>, _> =
        HashMap::with_capacity_and_hasher(nines.len(), WyHash::default());
    for nine in nines.iter().copied() {
        trailheads_reaching.insert(nine, [nine].iter().copied().collect());
    }
    let mut visited_this_generation: HashSet<_, WyHash> = HashSet::from_iter(nines.iter().copied());

    debug_save_graph_with_reaching(&graph, &trailheads_reaching, &9);

    for generation in (0..=8).rev() {
        let visited_previous_generation = visited_this_generation;
        visited_this_generation = Default::default();

        for source in visited_previous_generation {
            for neighbor in graph.neighbors_directed(source, Outgoing) {
                if graph[neighbor] == generation {
                    visited_this_generation.insert(neighbor);
                    let this_trailheads =
                        trailheads_reaching[&source].iter().copied().collect_vec();
                    trailheads_reaching
                        .entry(neighbor)
                        .or_default()
                        .extend(this_trailheads);
                }
            }
        }

        debug_save_graph_with_reaching(&graph, &trailheads_reaching, &generation);
    }

    visited_this_generation
        .into_iter()
        .map(|zero_index| trailheads_reaching[&zero_index].len())
        .sum()
}

fn debug_save_graph_with_reaching(
    graph: &Graph<usize, ()>,
    trailheads_reaching: &HashMap<NodeIndex, HashSet<NodeIndex, WyHash>, WyHash>,
    generation: &usize,
) {
    #[cfg(debug_assertions)]
    {
        let mut g2 = graph.clone();
        for (i, n) in g2.node_weights_mut().enumerate() {
            *n = *n * 10000
                + trailheads_reaching
                    .get(&NodeIndex::new(i))
                    .map(|h| h.len())
                    .unwrap_or_default();
        }
        debug_save_graph(&g2, &format!("gen_{generation}"));
    }
}

pub fn parse_graph(
    input: &str,
    reverse: bool,
) -> (Graph<usize, ()>, Vec<NodeIndex>, Vec<NodeIndex>) {
    let dim = input.find('\n').unwrap();
    let mut nines = vec![];
    let mut zeros = vec![];
    let map = Array2::from_shape_vec(
        (dim, dim),
        input
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize))
            .collect_vec(),
    )
    .unwrap();

    let mut graph = DiGraph::new();
    let indices = Array2::from_shape_vec(
        (dim, dim),
        map.iter().map(|digit| graph.add_node(*digit)).collect_vec(),
    )
    .unwrap();

    for (pos, digit) in map.indexed_iter() {
        if *digit == 0 {
            zeros.push(indices[pos]);
        }
        if *digit == 9 {
            nines.push(indices[pos]);
            continue;
        }
        for (dy, dx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let neighbor_pos = (pos.0.wrapping_add_signed(dy), pos.1.wrapping_add_signed(dx));
            if let Some(neighbor_digit) = map.get(neighbor_pos) {
                if *neighbor_digit == digit + 1 {
                    if reverse {
                        graph.add_edge(indices[pos], indices[neighbor_pos], ());
                    } else {
                        graph.add_edge(indices[neighbor_pos], indices[pos], ());
                    }
                }
            }
        }
    }

    debug_save_graph(&graph, "init");

    (graph, zeros, nines)
}

fn debug_save_graph(graph: &Graph<usize, ()>, suffix: &str) {
    #[cfg(debug_assertions)]
    writeln!(
        File::create(
            file!().rsplit_once('/').unwrap().0.to_owned() + &format!("/graphs/graph_{suffix}.dot")
        )
        .unwrap(),
        "{:?}",
        Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel])
    )
    .unwrap();
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")))
}
