use dbgprint::dbgeprintln;
use iter_tools::Itertools;
use std::cmp::Ordering;

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| {
            let mut readings = line
                .split(' ')
                .map(|v| v.parse::<i32>().unwrap())
                .multipeek();
            let first_4 = [
                *readings.peek().unwrap(),
                *readings.peek().unwrap(),
                *readings.peek().unwrap(),
                *readings.peek().unwrap(),
            ];
            let sign = if first_4
                .iter()
                .zip(first_4.iter().skip(1))
                .map(|(l, r)| r - l)
                .filter(|diff| diff > &0)
                .count()
                >= 2
            {
                1
            } else {
                -1
            };

            let mut iter = readings.clone().zip(readings.skip(1)).peekable();
            let condition =
                |diff: i32| -> bool { diff.signum() == sign && 1 <= diff.abs() && diff.abs() <= 3 };
            let mut skipped = false;
            let mut previous_left: Option<i32> = None;
            let mut override_left: Option<i32> = None;
            loop {
                let next = iter.next();
                if next.is_none() {
                    break;
                }
                let (left, right) = next.unwrap();
                let left = override_left.take().unwrap_or(left);

                let diff = right - left;
                if condition(diff) {
                    previous_left = Some(left);
                    continue;
                }
                if skipped {
                    dbgeprintln!(
                        "{}\n  Already skipped then got {} with sign {}",
                        line, diff, sign
                    );
                    assert_eq!(brute_force(line), 0);
                    return false;
                }
                // skip first, skip middle/end
                match iter.peek() {
                    None => continue,
                    Some((_, next_right)) => {
                        // 8 4 5 7
                        // L R -
                        // 2 8 4 5 7
                        // * L R -
                        if (previous_left
                            .map(|previous_left| condition(right - previous_left))
                            .unwrap_or(true))
                            && condition(next_right - right)
                        {
                            // skip left
                            iter.next();
                            skipped = true;
                        // 4 9 7 8
                        // L R -
                        // 2 4 3 5 8
                        // * L R -
                        } else if condition(next_right - left) {
                            // skip right
                            override_left = Some(left);
                            skipped = true;
                        } else {
                            assert_eq!(brute_force(line), 0);
                            return false;
                        }
                    }
                }
            }
            assert_eq!(brute_force(line), 1);
            return true;
        })
        .count() as u32
}

pub fn brute_force(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| {
            let readings: Vec<_> = line.split(' ').map(|v| v.parse::<i32>().unwrap()).collect();
            for remove_index in -1..readings.len() as i32 {
                for sign in [Ordering::Less, Ordering::Greater] {
                    let it = {
                        readings.iter().enumerate().filter_map(|(i, x)| {
                            if remove_index == -1 || i != remove_index as _ {
                                Some(*x)
                            } else {
                                None
                            }
                        })
                    };
                    if it.clone().zip(it.skip(1)).all(|(left, right)| {
                        right.abs_diff(left) >= 1
                            && right.abs_diff(left) <= 3
                            && right.cmp(&left) == sign
                    }) {
                        dbgeprintln!(
                            "{}\n  true by skipping {}, sign {:?}",
                            line, remove_index, sign
                        );
                        assert!(verify_skip_solves_reading(&readings, remove_index, sign));
                        return true;
                    }
                }
            }
            dbgeprintln!("{}\n  false", line);
            return false;
        })
        .count() as u32
}

fn verify_skip_solves_reading(reading: &Vec<i32>, remove_index: i32, sign: Ordering) -> bool {
    let mut left = if remove_index == 0 {
        None
    } else {
        reading.get(0).copied()
    };
    for i in 1..reading.len() {
        if i == remove_index as usize {
            continue;
        }
        let right = reading[i];
        if let Some(left) = left {
            if right.cmp(&left) != sign || left.abs_diff(right) > 3 || left.abs_diff(right) < 1 {
                return false;
            }
        }
        left = Some(right);
    }
    true
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
