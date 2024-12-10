use iter_tools::Itertools;
use ndarray::Array2;
use petgraph::algo::floyd_warshall;
use petgraph::dot::Dot;
use petgraph::graph::DiGraph;
use petgraph::prelude::*;
use std::fs::File;
use std::io::Write;

pub fn solve(input: &str) -> usize {
    let (graph, zeros, nines) = parse_graph(input);

    // Takes 20s on main input
    // TODO: gradually expanding graphs from all zeros and all nines, meet in middle?
    let distances = floyd_warshall(&graph, |_| 1).unwrap();
    zeros
        .iter()
        .copied()
        .cartesian_product(nines.iter().copied())
        .filter(|zero_nine_pair| distances[zero_nine_pair] != usize::MAX)
        .count()
}

pub fn parse_graph(input: &str) -> (Graph<usize, ()>, Vec<NodeIndex>, Vec<NodeIndex>) {
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
                    graph.add_edge(indices[pos], indices[neighbor_pos], ());
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    writeln!(
        File::create("graph.dot").unwrap(),
        "{:?}",
        Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel])
    )
    .unwrap();

    (graph, zeros, nines)
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")))
}
