use crate::day_12::part_2::solve;

#[test]
fn sample() {
    assert_eq!(
        solve(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
"
        ),
        1206
    );
}
