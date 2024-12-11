use iter_tools::Itertools;

pub fn solve(input: &str) -> usize {
    let mut stones = input
        .split(' ')
        .map(|word| word.parse().unwrap())
        .collect_vec();

    for _ in 0..25 {
        #[cfg(debug_assertions)]
        eprintln!("{}", stones.iter().map(|n: &usize| n.to_string()).join(" "));

        stones = stones
            .iter()
            .flat_map(|stone| step_stone(*stone))
            .collect_vec();
    }

    stones.len()
}

pub fn step_stone(stone: usize) -> Vec<usize> {
    let num_digits = if stone == 0 { 0 } else { stone.ilog10() + 1 };

    match stone {
        0 => vec![1],
        _ if num_digits % 2 == 0 => vec![
            stone / 10_usize.pow(num_digits / 2),
            stone % 10_usize.pow(num_digits / 2),
        ],
        _ => vec![stone * 2024],
    }
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")))
}
