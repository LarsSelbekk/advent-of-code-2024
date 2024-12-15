use crate::day_14::part_1::parse_robot_start_conditions;
use iter_tools::Itertools;
use num_integer::Integer;
use regex::Regex;
use std::collections::HashSet;
use wyhash2::WyHash;

pub fn solve(input: &str, width: isize, height: isize) {
    let regex = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let start_poses_and_velocities = input
        .lines()
        .map(|line| parse_robot_start_conditions(line, &regex))
        .collect_vec();

    let christmas_tree_regex = Regex::new("\u{2593}{10,}").unwrap();
    for step in 0.. {
        let positions = start_poses_and_velocities
            .iter()
            .map(|(start_pos, velocity)| {
                (
                    (start_pos.0 + velocity.0 * step).mod_floor(&width),
                    (start_pos.1 + velocity.1 * step).mod_floor(&height),
                )
            })
            .collect::<HashSet<_, WyHash>>();

        let size = (width as usize * 2 + 1) * height as usize + 1 + 20;
        let mut s = String::with_capacity(size);
        s.push_str(&step.to_string());
        s.push('\n');
        for y in 0..height {
            for x in 0..width {
                s.push(if positions.contains(&(x, y)) {
                    '\u{2593}'
                } else {
                    '\u{2022}'
                });
            }
            s.push('\n');
        }

        if christmas_tree_regex.is_match(&s) {
            println!("{}", s);
            return;
        }
    }
}

#[allow(unused)]
pub fn run() {
    solve(include_str!("input.txt"), 101, 103);
}
