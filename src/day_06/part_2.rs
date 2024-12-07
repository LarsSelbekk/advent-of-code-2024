use crate::day_06::part_1::{get_start_pos, parse_map, Direction, Square};
use iter_tools::Itertools;
use ndarray::{Array2, Axis};
use std::collections::HashMap;

pub fn solve(input: &str) -> u32 {
    let dim = input.find('\n').unwrap();
    let mut map = parse_map(input, dim);
    let start_pos = get_start_pos(input, dim);

    #[cfg(debug_assertions)]
    print_map(&map, start_pos, Direction::Up, &HashMap::<_, ()>::new());

    get_positions_to_try(&map, start_pos)
        .into_iter()
        .filter(|&pos_to_replace| {
            #[cfg(debug_assertions)]
            eprintln!("Trying {:?}", pos_to_replace);

            will_get_stuck(&mut map, start_pos, pos_to_replace)
        })
        .count() as _
}

fn will_get_stuck(map: &mut Array2<Square>, start_pos: (usize, usize), pos_to_replace: (usize, usize)) -> bool {
    let mut pos = start_pos;
    let mut visited_positions = HashMap::new();
    let mut direction = Direction::Up;

    map[pos_to_replace] = Square::Obstacle;

    let got_stuck = loop {
        let previous_visits = visited_positions.entry(pos).or_insert(vec![]);
        if previous_visits.contains(&direction) {
            break true;
        } else {
            previous_visits.push(direction);
        }
        if let Some(new_pos) = pos + direction {
            match map.get(new_pos) {
                Some(Square::Obstacle) => direction = direction.rotate(),
                Some(Square::Clear) => pos = new_pos,
                None => break false,
            }
        } else {
            break false;
        }

        #[cfg(debug_assertions)]
        print_map(&map, pos, direction, &visited_positions);
    };

    map[pos_to_replace] = Square::Clear;
    got_stuck
}

fn get_positions_to_try(
    map: &Array2<Square>,
    start_pos: (usize, usize),
) -> Vec<(usize, usize)> {
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
fn print_map<T>(
    map: &Array2<Square>,
    pos: (usize, usize),
    direction: Direction,
    visited_positions: &HashMap<(usize, usize), T>,
) {
    eprintln!(
        "{}\n",
        map.axis_iter(Axis(0))
            .enumerate()
            .map(|(y, tiles)| tiles
                .iter()
                .enumerate()
                .map(|(x, tile)| {
                    if pos == (y, x) {
                        direction.to_string()
                    } else if visited_positions.contains_key(&(y, x)) {
                        "X".to_string()
                    } else {
                        tile.to_string()
                    }
                })
                .join(" "))
            .join("\n")
    )
}

pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")))
}
