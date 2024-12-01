use std::collections::HashMap;
use std::ops::AddAssign;

pub fn solve(raw_input: &str) -> u32 {
    let mut right_counts = HashMap::<u32, u32>::new();
    let mut lefts = Vec::<u32>::new();
    for line in raw_input.lines() {
        let (left, right) = line.split_once("   ").expect("Bad line");
        lefts.push(left.parse().expect("Bad number"));
        let parsed_right = right.parse().expect("Bad number");
        right_counts.entry(parsed_right).or_insert(0).add_assign(1);
    }
    lefts
        .into_iter()
        .map(|left| left * right_counts.get(&left).unwrap_or(&0))
        .sum()
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}