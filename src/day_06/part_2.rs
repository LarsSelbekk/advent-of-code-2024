use crate::day_06::part_1::{get_start_pos, parse_map, Direction, Square};
use iter_tools::Itertools;
use ndarray::{Array2, Axis};
use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let dim = input.find('\n').unwrap();
    let map = parse_map(input, dim);
    let start_pos = get_start_pos(input, dim);

    get_positions_to_try(&map, start_pos)
        .into_iter()
        .filter(|&pos_to_replace| will_get_stuck(&map, start_pos, pos_to_replace))
        .count()
}

fn will_get_stuck(
    map: &Array2<Square>,
    start_pos: (usize, usize),
    pos_to_replace: (usize, usize),
) -> bool {
    #[cfg(debug_assertions)]
    eprintln!("Trying with obstacle at {:?}", pos_to_replace);

    let mut pos = start_pos;
    let mut visited_positions = HashMap::new();
    let mut direction = Direction::Up;

    let will_get_stuck = loop {
        #[cfg(debug_assertions)]
        print_map(&map, pos, direction, &visited_positions, pos_to_replace);

        let previous_visits = visited_positions.entry(pos).or_insert(vec![]);
        if previous_visits.contains(&direction) {
            break true;
        } else {
            previous_visits.push(direction);
        }

        if let Some(new_pos) = pos + direction {
            let square = if new_pos == pos_to_replace {
                Some(&Square::Obstacle)
            } else {
                map.get(new_pos)
            };
            match square {
                Some(Square::Obstacle) => direction = direction.rotate(),
                Some(Square::Clear) => pos = new_pos,
                None => break false,
            }
        } else {
            break false;
        }
    };

    #[cfg(debug_assertions)]
    eprintln!(
        "{}\n",
        if will_get_stuck {
            ":) :) :) Got stuck :) :) :)"
        } else {
            ":( :( :( Exceeded bounds :( :( :("
        }
    );

    will_get_stuck
}

fn get_positions_to_try(map: &Array2<Square>, start_pos: (usize, usize)) -> Vec<(usize, usize)> {
    map.indexed_iter()
        .filter_map(|(square_pos, square)| {
            if matches!(square, Square::Clear) && square_pos != start_pos {
                Some(square_pos)
            } else {
                None
            }
        })
        .collect_vec()
}

#[cfg(debug_assertions)]
fn print_map(
    map: &Array2<Square>,
    pos: (usize, usize),
    direction: Direction,
    visited_positions: &HashMap<(usize, usize), Vec<Direction>>,
    pos_to_replace: (usize, usize),
) {
    use Direction::*;
    eprintln!(
        "{}\n",
        map.axis_iter(Axis(0))
            .enumerate()
            .map(|(y, tiles)| tiles
                .iter()
                .enumerate()
                .map(|(x, tile)| {
                    if pos_to_replace == (y, x) {
                        '\u{2591}'.to_string()
                    } else if pos == (y, x) {
                        direction.to_string()
                    } else if let Some(directions) = visited_positions.get(&(y, x)) {
                        match directions.iter().sorted().collect_vec()[..] {
                            [Up] => '\u{2191}',
                            [Right] => '\u{2192}',
                            [Down] => '\u{2193}',
                            [Left] => '\u{2190}',
                            [Up, Right] => '\u{2197}',
                            [Up, Down] => '\u{2195}',
                            [Up, Left] => '\u{2196}',
                            [Right, Down] => '\u{2198}',
                            [Right, Left] => '\u{2194}',
                            [Down, Left] => '\u{2199}',
                            [Up, Right, Down] => '\u{21a6}',
                            [Up, Right, Left] => '\u{21a5}',
                            [Up, Down, Left] => '\u{21a4}',
                            [Right, Down, Left] => '\u{21a7}',
                            [Up, Right, Down, Left] => '+',
                            _ => unreachable!(),
                        }
                        .to_string()
                    } else {
                        tile.to_string()
                    }
                })
                .join(" "))
            .join("\n")
    )
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")))
}
