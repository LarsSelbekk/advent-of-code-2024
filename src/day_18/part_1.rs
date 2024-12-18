use crate::day_06::part_1::Square::{Clear, Obstacle};
use crate::day_16::part_1::add_position;
use crate::day_16::part_1::Direction::{East, North, South, West};
use crate::day_16::part_2::print_map;
use iter_tools::Itertools;
use ndarray::Array2;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

pub fn solve(input: &str, dim: usize, num_falling: usize) -> Option<usize> {
    let mut map = Array2::from_shape_fn((dim, dim), |(_, _)| Clear);
    let obstacles = parse_obstacles(input);

    for i in 0..num_falling {
        map[obstacles[i % obstacles.len()]] = Obstacle;
    }

    let start_pos = (0, 0);
    let end_pos = (dim - 1, dim - 1);

    let mut to_explore = BinaryHeap::from([(Reverse(0), start_pos)]);
    let mut visited = HashSet::<(usize, usize)>::from([start_pos]);

    loop {
        let (cost, position) = to_explore.pop()?;
        #[cfg(debug_assertions)]
        print_map(&map, &visited, Some((obstacles[(num_falling - 1) % obstacles.len()], North)));

        if position == end_pos {
            break Some(cost.0);
        }

        for direction in [North, West, South, East] {
            if let Some(new_position) = add_position(position, direction, dim, &map) {
                if !visited.contains(&new_position) {
                    to_explore.push((Reverse(cost.0 + 1), new_position));
                    visited.insert(new_position);
                }
            }
        }
    }
}

pub fn parse_obstacles(input: &str) -> Vec<(usize, usize)> {
    let obstacles: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|coord| coord.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_vec();
    obstacles
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt"), 71, 1024).unwrap())
}
