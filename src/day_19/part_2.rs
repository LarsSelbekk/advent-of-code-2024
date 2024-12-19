use crate::day_19::part_1;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let (patterns, designs) = part_1::parse_input(input);

    designs
        .iter()
        .map(|design| num_solutions_memo(&patterns, design, &mut HashMap::new()))
        .sum()
}

fn num_solutions_memo<'a>(
    patterns: &Vec<&str>,
    design: &'a str,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    memo.get(design).copied().unwrap_or_else(|| {
        if design.is_empty() {
            1
        } else {
            let num_solutions = patterns
                .iter()
                .map(|pattern| {
                    if design.starts_with(pattern) {
                        num_solutions_memo(patterns, &design[design.len()..], memo)
                    } else {
                        0
                    }
                })
                .sum();
            memo.insert(design, num_solutions);
            num_solutions
        }
    })
}

pub fn brute_force(input: &str) -> usize {
    let (patterns, designs) = part_1::parse_input(input);

    designs
        .par_iter()
        .map(|design| dbg!(num_solutions_brute_force(&patterns, design)))
        .sum()
}

fn num_solutions_brute_force(patterns: &Vec<&str>, design: &str) -> usize {
    if design.is_empty() {
        1
    } else {
        patterns
            .iter()
            .filter_map(|pattern| {
                Some(num_solutions_brute_force(
                    patterns,
                    design.strip_prefix(pattern)?,
                ))
            })
            .sum()
    }
}

#[allow(unused)]
pub fn print_answer_brute_force() {
    println!("{}", brute_force(include_str!("input.txt")));
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
