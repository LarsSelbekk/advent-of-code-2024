use crate::day_11::part_1::step_stone;
use std::collections::HashMap;

pub fn solve(input: &str, depth: usize) -> usize {
    let stones = input.split(' ').map(|word| word.parse::<usize>().unwrap());

    let mut cache = HashMap::new();

    stones
        .map(|stone| count_stones_recursive(stone, depth, &mut cache))
        .sum()
}

fn count_stones_recursive(
    stone: usize,
    depth: usize,
    mut cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if depth == 0 {
        return 1;
    }

    if let Some(cached) = cache.get(&(stone, depth)) {
        return *cached;
    }

    let num_sub_stones = step_stone(stone)
        .into_iter()
        .map(|sub_stone| count_stones_recursive(sub_stone, depth - 1, &mut cache))
        .sum();
    cache.insert((stone, depth), num_sub_stones);
    num_sub_stones
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt"), 75))
}
