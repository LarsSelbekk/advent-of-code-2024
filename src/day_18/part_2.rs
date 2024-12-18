use crate::day_18::part_1;

/// 10.1ms
#[allow(unused)]
pub fn brute_force_exponential(input: &str, dim: usize) -> String {
    let obstacles = part_1::parse_obstacles(input);
    let mut known_good = 1;
    let mut known_bad = None;
    loop {
        if known_bad == Some(known_good + 1) {
            let (x, y) = obstacles[known_good % obstacles.len()];
            #[cfg(debug_assertions)]
            dbg!(known_good);
            return format!("{x},{y}");
        }
        let next = next_index_exponential(known_good, known_bad);
        if part_1::solve(input, dim, next).is_none() {
            known_bad = Some(next);
        } else {
            known_good = next;
        }
    }
}

fn next_index_exponential(known_good: usize, known_bad: Option<usize>) -> usize {
    if let Some(known_bad) = known_bad {
        (known_good + known_bad) / 2
    } else {
        known_good * 2
    }
}

/// 1.4s
#[allow(unused)]
pub fn brute_force(input: &str, dim: usize) -> String {
    let obstacles = part_1::parse_obstacles(input);
    for i in 1.. {
        if part_1::solve(input, dim, i).is_none() {
            let (x, y) = obstacles[(i - 1) % obstacles.len()];
            #[cfg(debug_assertions)]
            dbg!(i);
            return format!("{x},{y}");
        }
    }

    unreachable!()
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", brute_force_exponential(include_str!("input.txt"), 71));
}
