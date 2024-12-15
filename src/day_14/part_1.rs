use crate::day_14::part_1::Quadrant::{BottomLeft, BottomRight, TopLeft, TopRight};
use iter_tools::Itertools;
use num_integer::Integer;
use regex::Regex;
use std::cmp::Ordering;
use Ordering::{Greater, Less};

pub fn solve(input: &str, width: isize, height: isize) -> usize {
    const STEPS: isize = 100;
    let regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let counts = input
        .lines()
        .filter_map(|line| {
            let (start_pos, velocity) = parse_robot_start_conditions(line, &regex);

            let end_pos = (
                (start_pos.0 + velocity.0 * STEPS).mod_floor(&width),
                (start_pos.1 + velocity.1 * STEPS).mod_floor(&height),
            );

            #[cfg(debug_assertions)]
            dbg!(end_pos);

            match (end_pos.0.cmp(&(width / 2)), end_pos.1.cmp(&(height / 2))) {
                (Less, Less) => Some(TopLeft),
                (Greater, Less) => Some(TopRight),
                (Greater, Greater) => Some(BottomRight),
                (Less, Greater) => Some(BottomLeft),
                _ => None,
            }
        })
        .counts();

    #[cfg(debug_assertions)]
    dbg!(&counts);

    counts.values().product()
}

pub fn parse_robot_start_conditions(line: &str, regex: &Regex) -> ((isize, isize), (isize, isize)) {
    regex
        .captures(line)
        .unwrap()
        .iter()
        .skip(1)
        .take(4)
        .map(|i| i.unwrap().as_str().parse::<isize>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|chunk| chunk.collect_tuple::<(_, _)>().unwrap())
        .collect_tuple::<(_, _)>()
        .unwrap()
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt"), 101, 103));
}
