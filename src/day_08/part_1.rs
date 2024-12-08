use iter_tools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> usize {
    let dim = input.find('\n').unwrap();
    let mut antinodes = HashSet::new();
    
    for (frequency, locations) in parse_map(input) {
        #[cfg(debug_assertions)]
        eprintln!("Checking frequency '{}'", frequency);
        
        for locs in locations.iter().combinations(2) {
            let first = **locs.iter().min().unwrap();
            let last = **locs.iter().max().unwrap();
            let distance = tup_distance(first, last);
            
            #[cfg(debug_assertions)]
            eprintln!("  Checking positions {:?} and {:?} with distance {:?}", first, last, distance);
            
            for antinode in [(first, tup_neg(distance)), (last, distance)]
                .iter()
                .filter_map(|(start, offset)| tup_add_within_bounds(*start, *offset, dim))
            {
                #[cfg(debug_assertions)]
                eprintln!("      Antinode at {:?}", antinode);
                
                antinodes.insert(antinode);
            }
        }
    }
    antinodes.len()
}

fn tup_neg(arg: (isize, isize)) -> (isize, isize) {
    (-arg.0, -arg.1)
}

fn tup_add_within_bounds(left: (usize, usize), right: (isize, isize), dim: usize) -> Option<(usize, usize)> {
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
