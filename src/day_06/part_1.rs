use iter_tools::Itertools;
use ndarray::{Array2, Axis};
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use std::ops::Add;

pub enum Square {
    Obstacle,
    Clear,
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Square::Obstacle => f.write_char('#'),
            Square::Clear => f.write_char('.'),
        }
    }
}

pub fn solve(input: &str) -> u32 {
    let dim = input.find('\n').unwrap();

    let map = parse_map(input, dim);

    let mut pos = get_start_pos(input, dim);
    let mut direction = Direction::Up;
    let mut visited_positions = HashSet::new();

    #[cfg(debug_assertions)]
    print_map(&map, pos, direction, &visited_positions);

    loop {
        visited_positions.insert(pos);
        if let Some(new_pos) = pos + direction {
            match map.get(new_pos) {
                Some(Square::Obstacle) => direction = direction.rotate(),
                Some(Square::Clear) => pos = new_pos,
                None => break,
            }
        } else {
            break;
        }

        #[cfg(debug_assertions)]
        print_map(&map, pos, direction, &visited_positions);
    }
    visited_positions.len() as _
}

pub fn get_start_pos(input: &str, dim: usize) -> (usize, usize) {
    let index = input.find('^').unwrap();
    let index = index - index / (dim + 1);
    (index / dim, index % dim)
}

pub(crate) fn parse_map(input: &str, dim: usize) -> Array2<Square> {
    let map: Array2<Square> = Array2::from_shape_vec(
        (dim, dim),
        input
            .lines()
            .flat_map(|l| {
                l.chars().map(|c| match c {
                    '#' => Square::Obstacle,
                    '.' | '^' => Square::Clear,
                    _ => unreachable!(),
                })
            })
            .collect_vec(),
    )
    .unwrap();
    map
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub(crate) fn rotate(&self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Add<Direction> for (usize, usize) {
    type Output = Option<(usize, usize)>;

    fn add(self, rhs: Direction) -> Self::Output {
        let tup = match rhs {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };
        (self.0 as i32)
            .checked_add(tup.0)
            .and_then(|x| (self.1 as i32).checked_add(tup.1).map(|y| (x as _, y as _)))
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => f.write_char('^'),
            Direction::Down => f.write_char('v'),
            Direction::Left => f.write_char('<'),
            Direction::Right => f.write_char('>'),
        }
    }
}

#[cfg(debug_assertions)]
fn print_map(
    map: &Array2<Square>,
    pos: (usize, usize),
    direction: Direction,
    visited_positions: &HashSet<(usize, usize)>,
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
                    } else if visited_positions.contains(&(y, x)) {
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
    println!("{}", solve(include_str!("input.txt")));
}
