#[allow(unused_imports)]
use crate::day_02::part_1::solve;

#[test]
fn test_example_input() {
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(solve(INPUT), 2);
}
