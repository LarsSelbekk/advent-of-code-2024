#[allow(unused_imports)]
use crate::day_03::part_2::solve;

#[test]
fn sample() {
    assert_eq!(
        solve("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
        48
    )
}