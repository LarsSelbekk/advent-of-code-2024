use crate::day_13::part_1;
use regex::Regex;

const PART_2_PENALTY: usize = 10_000_000_000_000;

pub fn solve(input: &str) -> usize {
    let coords_regex = Regex::new(r".*: X.(\d+), Y.(\d+)").unwrap();
    let mut problems = input.lines().filter(|l| !l.is_empty());

    let mut res = 0;
    while let Some((u, v, r)) = part_1::next_problem(&mut problems, &coords_regex) {
        let harder_r = (r.0 + PART_2_PENALTY, r.1 + PART_2_PENALTY);
        res += part_1::solve_problem(u, v, harder_r);
    }
    res
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
