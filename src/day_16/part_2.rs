use crate::day_06::part_1::Square;
use crate::day_16::part_1;
use crate::day_16::part_1::Direction;
use crate::day_16::part_1::Direction::*;
use colored::Colorize;
use iter_tools::Itertools;
use ndarray::Array2;
use std::cmp::Ordering::{Equal, Greater};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::iter::once;

pub type State = ((usize, usize), Direction);

pub fn solve(input: &str) -> usize {
    let (map, dim, start_pos, end_pos) = part_1::parse_map(input);

    let start_state = (start_pos, East);
    let mut to_explore = BinaryHeap::from([(Reverse(0), start_state)]);
    let mut least_cost_to =
        HashMap::<State, (usize, Vec<State>)>::from([(start_state, (0, vec![]))]);

    loop {
        let opt = to_explore.pop();
        if opt.is_none() {
            break;
        };
        let (cost, (position, direction)) = opt.unwrap();
        
        #[cfg(debug_assertions)]
        print_map(
            &map,
            &HashSet::from_iter(least_cost_to.keys().map(|(pos, _)| *pos)),
            Some((position, direction)),
        );

        if let Some(new_position) = part_1::add_position(position, direction, dim, &map) {
            add_or_update(
                &mut to_explore,
                &mut least_cost_to,
                (position, direction),
                (new_position, direction),
                cost.0 + 1,
            );
        }

        for new_direction in [
            direction.rotate_clockwise(),
            direction.rotate_anticlockwise(),
        ] {
            add_or_update(
                &mut to_explore,
                &mut least_cost_to,
                (position, direction),
                (position, new_direction),
                cost.0 + 1000,
            );
        }
    }
    
    let end_states = once(end_pos)
        .cartesian_product([East, West, North, South]).map(|state| 
            (state, least_cost_to.get(&state).unwrap().0)).collect_vec();
    let least_cost = end_states.iter().min_by_key(|(_, cost)| *cost).unwrap().1;
    let positions_on_shortest_paths = end_states
        .iter()
        .filter(|(_, cost)| *cost == least_cost)
        .flat_map(|(state, _)| shortest_paths_leading_to(*state, &least_cost_to))
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>();
    
    #[cfg(debug_assertions)]
    print_map(&map, &positions_on_shortest_paths, None);

    positions_on_shortest_paths.len()
}

fn shortest_paths_leading_to(
    state: State,
    least_costs_to: &HashMap<State, (usize, Vec<State>)>,
) -> HashSet<State> {
    HashSet::from_iter(
        once(state).chain(
            least_costs_to
                .get(&state)
                .map(|(_, neighbors)| neighbors)
                .unwrap_or(&vec![])
                .iter()
                .flat_map(|n| shortest_paths_leading_to(*n, least_costs_to)),
        ),
    )
}

fn add_or_update(
    to_explore: &mut BinaryHeap<(Reverse<usize>, State)>,
    least_cost_to: &mut HashMap<State, (usize, Vec<State>)>,
    neighbor: State,
    new: State,
    new_cost: usize,
) {
    least_cost_to
        .entry(new)
        .and_modify(
            |(old_cost, old_neighbors)| match (*old_cost).cmp(&new_cost) {
                Greater => {
                    *old_cost = new_cost;
                    *old_neighbors = vec![neighbor]
                },
                Equal => old_neighbors.push(neighbor),
                _ => {}
            },
        )
        .or_insert_with(|| {
            to_explore.push((Reverse(new_cost), new));
            (new_cost, vec![neighbor])
        });
}

pub fn print_map(map: &Array2<Square>, visited: &HashSet<(usize, usize)>, cursor: Option<State>) {
    for (pos, square) in map.indexed_iter() {
        if pos.1 == 0 {
            println!();
        }

        if cursor.map(|state| state.0 == pos).unwrap_or_default() {
            print!(
                "{}",
                match cursor.unwrap().1 {
                    North => "^",
                    East => ">",
                    South => "v",
                    West => "<",
                }
                .on_purple()
            )
        } else if visited.contains(&pos) {
            print!("{}", "+".on_blue());
        } else {
            print!(
                "{}",
                match square {
                    Square::Obstacle => " ".on_bright_black(),
                    Square::Clear => "⋅".into(),
                }
            )
        }
    }
    println!();
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
