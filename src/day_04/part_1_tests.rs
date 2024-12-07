use crate::day_04::part_1::solve;
use iter_tools::Itertools;
use ndarray::Array2;
use regex::Regex;

#[test]
fn test_sample() {
    assert_eq!(
        solve(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
        ),
        18
    )
}

#[test]
fn test_regex() {
    assert_eq!(
        Regex::new(r"XMAS")
            .unwrap()
            .captures_iter("kjsfXMASalXMASsdf")
            .count(),
        2
    )
}

#[test]
fn test_matrix() {
    let _matrix = Array2::<char>::from_shape_vec(
        (140, 140),
        include_str!("input.txt")
            .lines()
            .flat_map(|line| line.chars())
            .collect_vec(),
    )
    .unwrap();
}
