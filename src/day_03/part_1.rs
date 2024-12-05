use regex::Regex;

pub fn solve(input: &str) -> u32 {
    Regex::new(r"mul\(([0-9]+,[0-9]+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|captures| captures.get(1).unwrap().as_str().split_once(",").unwrap())
        .map(|(left, right)| left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap())
        .sum()
}

#[allow(dead_code)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
