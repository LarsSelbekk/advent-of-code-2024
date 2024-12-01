#[allow(unused_imports)]
use crate::day_01::part_2::solve;

#[test]
fn test_example_input() {
    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
    assert_eq!(solve(INPUT), 31);
}