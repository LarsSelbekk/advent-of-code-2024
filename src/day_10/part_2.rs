use crate::day_10::part_1::parse_graph;
use iter_tools::Itertools;

pub fn solve(input: &str) -> usize {
    let (graph, zeros, nines) = parse_graph(input);
    zeros
        .iter()
        .cartesian_product(nines.iter())
        .map(|(zero, nine)| {
            petgraph::algo::all_simple_paths::<Vec<_>, &petgraph::Graph<usize, ()>>(
                &graph, *zero, *nine, 0, None,
            )
            .count()
        })
        .sum()
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
