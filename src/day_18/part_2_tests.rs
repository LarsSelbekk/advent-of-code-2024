use crate::day_18::part_2::{brute_force, brute_force_exponential};

#[test]
fn sample() {
    assert_eq!(brute_force("5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
", 7), "6,1")
}

#[test]
fn sample_exponential() {
    assert_eq!(brute_force_exponential("5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
", 7), "6,1")
}
