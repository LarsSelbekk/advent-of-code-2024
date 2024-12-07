use iter_tools::Itertools;
use petgraph::algo::toposort;
use petgraph::data::DataMap;
use petgraph::prelude::*;
use std::collections::{HashMap, HashSet};
use std::iter::{Map, Skip};
use std::str::Lines;

pub fn solve(input: &str) -> usize {
    let mut lines = input.lines();

    let requirements = construct_requirements_graph(&mut lines);

    parse_updates(lines)
        .map(|update| {
            if is_already_sorted(input, &update) {
                return 0;
            }

            let sub_graph = filter_graph(&requirements, HashSet::from_iter(update));

            let sorted = dbg!(toposort(&sub_graph, None)
                .unwrap()
                .iter()
                .map(|node| *sub_graph.node_weight(*node).unwrap())
                .collect_vec());

            sorted[sorted.len() / 2]
        })
        .sum()
}

fn parse_updates(lines: Lines) -> Map<Skip<Lines>, fn(&str) -> Vec<usize>> {
    lines.skip(1).map(|line| {
        line.split(",")
            .map(|element| element.parse::<usize>().unwrap())
            .collect_vec()
    })
}

fn construct_requirements_graph(lines: &mut Lines) -> Graph<usize, usize> {
    {
        let mut graph = DiGraph::<usize, usize>::new();

        let requirements = lines
            .take_while_ref(|line| !line.is_empty())
            .map(|line| line.split_once("|").unwrap())
            .map(|(required, requirer)| {
                (
                    required.parse::<usize>().unwrap(),
                    requirer.parse::<usize>().unwrap(),
                )
            })
            .collect_vec();

        let mut weight_to_index = HashMap::<usize, NodeIndex<u32>>::new();
        for weight in requirements
            .iter()
            .flat_map(|(requirer, required)| [requirer, required])
            .collect::<HashSet<_>>()
        {
            weight_to_index.insert(*weight, graph.add_node(*weight));
        }
        for (requirer, required) in requirements {
            graph.add_edge(weight_to_index[&requirer], weight_to_index[&required], 0);
        }
        graph
    }
}

fn filter_graph(
    requirements: &Graph<usize, usize>,
    update_set: HashSet<usize>,
) -> Graph<usize, usize> {
    let mut sub_graph = requirements.clone();

    sub_graph.retain_edges(|graph, index| {
        let (start, end) = graph.edge_endpoints(index).unwrap();
        update_set.contains(graph.node_weight(start).unwrap())
            && update_set.contains(graph.node_weight(end).unwrap())
    });
    sub_graph.retain_nodes(|graph, index| update_set.contains(graph.node_weight(index).unwrap()));
    sub_graph
}

fn is_already_sorted(input: &str, update: &Vec<usize>) -> bool {
    super::part_1::solve(
        &(input
            .lines()
            .take_while_ref(|line| !line.is_empty())
            .join("\n")
            + "\n\n"
            + &update.iter().map(|element| element.to_string()).join(",")),
    ) != 0
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")))
}
