use crate::day_10::part_1::solve;

#[test]
fn sample_2() {
    assert_eq!(
        solve(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
        ),
        36
    )
}

#[test]
fn sample_simple() {
    assert_eq!(
        solve(
            "0123
1234
8765
9876"
        ),
        1
    )
}
