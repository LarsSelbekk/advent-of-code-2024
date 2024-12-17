use crate::day_17::part_1;
use crate::day_17::part_2::{output_single_value, brute_force_solve};
use iter_tools::Itertools;

#[test]
fn sample() {
    assert_eq!(
        brute_force_solve(
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
        ),
        117_440
    );
}

#[test]
fn solution() {
    assert_eq!(
        part_1::solve(
            "Register A: 37221270076916
Register B: 0
Register C: 0

Program: 2,4,1,2,7,5,4,5,1,3,5,5,0,3,3,0"
        ),
        "2,4,1,2,7,5,4,5,1,3,5,5,0,3,3,0"
    );
}

#[test]
fn test_sample_solution() {
    assert_eq!(
        part_1::solve(
            "Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
        ),
        "0,3,5,4,3,0"
    );
}

#[test]
fn test_output_single_value() {
    let expected_output = part_1::solve(
        "Register A: 117440
Register B: 0
Register C: 0

Program: 2,4,1,2,7,5,4,5,1,3,5,5,0,3,3,0",
    );
    let mut output = vec![];
    let mut a = 117440;
    while a != 0 {
        output.push(output_single_value(a));
        a >>= 3;
    }
    assert_eq!(output.iter().join(","), expected_output)
}