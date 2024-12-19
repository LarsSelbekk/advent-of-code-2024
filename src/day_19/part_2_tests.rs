use crate::day_19::part_2::{brute_force, solve};

#[test]
fn sample() {
    assert_eq!(
        solve(
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
        ),
        16
    )
}

#[test]
fn sample_brute_force() {
    assert_eq!(
        brute_force(
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
        ),
        16
    )
}
