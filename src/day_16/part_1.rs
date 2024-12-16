use crate::day_06::part_1::Square;
use crate::day_06::part_1::Square::{Clear, Obstacle};
use crate::day_16::part_1::Direction::*;
use iter_tools::Itertools;
use ndarray::Array2;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

pub fn solve(input: &str) -> usize {
    let (map, dim, start_pos, end_pos) = parse_map(input);
    let mut to_explore = BinaryHeap::from([(Reverse(0), start_pos, East)]);
    let mut visited = HashSet::<((usize, usize), Direction)>::new();

    loop {
        let (cost, position, direction) = to_explore
            .pop()
            .expect("Should have found the end before running out of tiles to try");

        if position == end_pos {
            break cost.0;
        }

        if let Some(new_position) = add_position(position, direction, dim, &map) {
            if !visited.contains(&(new_position, direction)) {
                to_explore.push((Reverse(cost.0 + 1), new_position, direction));
                visited.insert((new_position, direction));
            }
        }

        for new_direction in [
            direction.rotate_clockwise(),
            direction.rotate_anticlockwise(),
        ] {
            if !visited.contains(&(position, new_direction)) {
                to_explore.push((Reverse(cost.0 + 1000), position, new_direction));
                visited.insert((position, new_direction));
            }
        }
    }
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_tuple(self) -> (isize, isize) {
        match self {
            North => (-1, 0),
            East => (0, 1),
            South => (1, 0),
            West => (0, -1),
        }
    }

    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub fn rotate_anticlockwise(&self) -> Direction {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}

pub fn add_position(
    pos: (usize, usize),
    mov: Direction,
    dim: usize,
    map: &Array2<Square>,
) -> Option<(usize, usize)> {
    let dir_tup = mov.to_tuple();
    let next_position = (pos.0 as isize + dir_tup.0, pos.1 as isize + dir_tup.1);
    if next_position.0 < 0
        || next_position.1 < 0
        || next_position.0 >= dim as _
        || next_position.1 >= dim as _
    {
        return None;
    }

    let next_position = (next_position.0 as _, next_position.1 as _);
    if *map.get(next_position)? == Clear {
        Some(next_position)
    } else {
        None
    }
}

pub fn parse_map(input: &str) -> (Array2<Square>, usize, (usize, usize), (usize, usize)) {
    let dim = input.find('\n').unwrap();
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    let map = Array2::from_shape_vec(
        (dim, dim),
        input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| l.chars().enumerate().map(move |(x, c)| ((y, x), c)))
            .map(|(pos, char)| match char {
                '#' => Obstacle,
                '.' => Clear,
                'S' => {
                    start_pos = pos;
                    Clear
                }
                'E' => {
                    end_pos = pos;
                    Clear
                }
                _ => unreachable!(),
            })
            .collect_vec(),
    )
    .unwrap();

    (map, dim, start_pos, end_pos)
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
