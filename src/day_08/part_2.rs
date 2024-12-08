use iter_tools::Itertools;
use std::collections::{HashMap, HashSet};
use num_integer::Integer;

pub fn solve(input: &str) -> usize {
    let dim = input.find('\n').unwrap();
    let mut antinodes = HashSet::new();

    for (frequency, locations) in parse_map(input) {
        #[cfg(debug_assertions)]
        eprintln!("Checking frequency '{}'", frequency);

        for locs in locations.iter().combinations(2) {
            let start = **locs.iter().min().unwrap() ;
            let end = **locs.iter().max().unwrap();
            #[cfg(debug_assertions)]
            eprintln!( "  Checking positions {:?} and {:?}", start, end );

            for antinode in Antinodes::new(start, end, dim) {
                #[cfg(debug_assertions)]
                eprintln!("      Antinode at {:?}", antinode);

                antinodes.insert(antinode);
            }
        }
    }
    antinodes.len()
}

struct Antinodes {
    start: (usize, usize),
    dim: usize,
    distance: (isize, isize),
    direction: isize,
    i: usize,
}

impl Antinodes {
    pub fn new(start: (usize, usize), end: (usize, usize), dim: usize) -> Antinodes {
        let distance = tup_reduce(tup_distance(start, end));
        Antinodes {
            start,
            dim,
            distance,
            direction: -1,
            i: 0,
        }
    }
}

fn tup_reduce(tup: (isize, isize)) -> (isize, isize) {
    let divisor = tup.0.gcd(&tup.1);
    (tup.0 / divisor, tup.1 / divisor)
}

impl Iterator for Antinodes {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(antinode) = tup_add_within_bounds(
            self.start,
            tup_mult(self.distance, self.direction * (self.i as isize)),
            self.dim,
        ) {
            self.i += 1;
            Some(antinode)
        } else if self.direction == -1 {
            self.direction = 1;
            self.i = 0;
            self.next()
        } else {
            None
        }
    }
}

fn tup_mult(tup: (isize, isize), scalar: isize) -> (isize, isize) {
    (tup.0 * scalar, tup.1 * scalar)
}

fn tup_add_within_bounds(
    left: (usize, usize),
    right: (isize, isize),
    dim: usize,
) -> Option<(usize, usize)> {
    #[cfg(debug_assertions)]
    eprintln!(
        "    Trying {:?}",
        (left.0 as isize + right.0, left.1 as isize + right.1)
    );
    let (new_left, new_right) = (
        left.0.checked_add_signed(right.0)?,
        left.1.checked_add_signed(right.1)?,
    );
    if new_left >= dim || new_right >= dim {
        None
    } else {
        Some((new_left, new_right))
    }
}

fn tup_distance(left: (usize, usize), right: (usize, usize)) -> (isize, isize) {
    (
        right.0 as isize - left.0 as isize,
        right.1 as isize - left.1 as isize,
    )
}

fn parse_map(input: &str) -> HashMap<char, HashSet<(usize, usize)>> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '.' => None,
                _ => Some((c, (y, x))),
            })
        })
        .into_grouping_map()
        .collect()
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")))
}
