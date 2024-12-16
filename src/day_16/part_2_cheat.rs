use std::iter::once;
use iter_tools::Itertools;
use petgraph::prelude::*;
use crate::day_06::part_1::Square::Clear;
use crate::day_16::part_1;
use crate::day_16::part_1::Direction::{East, North, South, West};
use crate::day_16::part_2::State;

pub fn solve(input: &str) -> usize {
    let (map, dim, start_pos, end_pos) = part_1::parse_map(input);
    let mut graph = DiGraph::<State, usize>::new();
    for (pos, tile) in map.indexed_iter() {
        if tile == &Clear {
            for state in once(pos).cartesian_product([East, West, North, South]) {
                graph.add_node(state);
            }
        }
    }
        2
}
