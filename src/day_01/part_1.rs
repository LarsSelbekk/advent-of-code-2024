use std::collections::BinaryHeap;

pub fn solve(input: &str) -> u32 {
    let mut lefts = BinaryHeap::<u32>::new();
    let mut rights = BinaryHeap::<u32>::new();
    for line in input.lines() {
        let (x, y) = line.split_once("   ").expect("Bad line");
        lefts.push(x.parse().expect("Bad number"));
        rights.push(y.parse().expect("Bad number"));
    }
    lefts
        .into_sorted_vec()
        .into_iter()
        .zip(rights.into_sorted_vec().into_iter())
        .map(|(x, y)| {
            dbg!((x, y));
            dbg!(x.abs_diff(y))
        })
        .sum()
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}