#[allow(unused_imports)]
use crate::day_02::part_2::{solve};

#[test]
fn test_example_input() {
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(solve(INPUT), 4);
}

#[test]
fn test_1() {
    const INPUT: &str = "1 2 3 4 5";
    assert_eq!(solve(INPUT), 1);
}

#[test]
fn test_2() {
    const INPUT: &str = "1 2 8 4 5";
    assert_eq!(solve(INPUT), 1);
}

#[test]
fn test_3() {
    const INPUT: &str = "1 2 2 4 5";
    assert_eq!(solve(INPUT), 1);
}

#[test]
fn test_4() {
    const INPUT: &str = "1 2 7 4 5";
    assert_eq!(solve(INPUT), 1);
}

#[test]
fn test_5() {
    const INPUT: &str = "1 2 14 47 5";
    assert_eq!(solve(INPUT), 0);
}

#[test]
fn test_6() {
    const INPUT: &str = "1 2 14 47 5
4 92 7 9 12
92 9 6 3 0";
    assert_eq!(solve(INPUT), 2);
}

#[test]
fn test_7() {
    const INPUT: &str = "99 5 96 93 90 89";
    assert_eq!(solve(INPUT), 1);
}

#[test]
fn test_random() {
    let num = rand::random_range(1..100);
    let mut input = "".to_string();
    for _ in 0..num {
        let start = rand::random_range(0..100);
        let sign = rand::random_range(0..=1) * 2 - 1;
        let diffs = (0..rand::random_range(5..10)).map(|_| rand::random_range(1..3));
        let mut v = vec![start];
        for diff in diffs {
            v.push(sign * diff + v.last().unwrap());
        }
        input.push_str(
            &*(v.iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(" ")
                .to_string()
                + "\n"),
        );
    }
    assert_eq!(solve(&input), num);
}

#[test]
fn test_random_almost_fail() {
    let num = rand::random_range(1..100);
    let mut input = "".to_string();
    for _ in 0..num {
        let start = rand::random_range(0..100);
        let sign = rand::random_range(0..=1) * 2 - 1;
        let diffs: Vec<_> = (0..rand::random_range(5..10))
            .map(|_| rand::random_range(1..3))
            .collect();
        let mut v = vec![start];
        for diff in diffs {
            v.push(sign * diff + v.last().unwrap());
        }
        let messup_index = rand::random_range(0..v.len());
        let f = v
            .iter()
            .take(messup_index)
            .chain([4000].iter())
            .chain(v.iter().skip(messup_index));
        input.push_str(
            &*(f.map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(" ")
                .to_string()
                + "\n"),
        );
    }
    assert_eq!(solve(&input), num);
}

#[test]
fn test_error_start() {
    assert_eq!(solve("4000 4 5 8 11"), 1)
}

#[test]
fn test_simple() {
    assert_eq!(solve("82 86 83 84 87"), 1)
}

#[test]
fn test_more() {
    assert_eq!(solve("81 82 83 84 89"), 1)
}