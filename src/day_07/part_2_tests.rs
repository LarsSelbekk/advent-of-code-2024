use crate::day_07::part_2::{get_trit, solve};

#[test]
fn sample() {
    assert_eq!(solve("190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"), 11387)
}

#[test]
fn test_get_trit() {
    assert_eq!(get_trit(23, 1), 1);
    assert_eq!(get_trit(9, 2), 1);
    assert_eq!(get_trit(3, 1), 1);
    assert_eq!(get_trit(0, 1), 0);
    assert_eq!(get_trit(0, 0), 0);
    assert_eq!(get_trit(10, 2), 1);
    assert_eq!(get_trit(18, 2), 2);
}