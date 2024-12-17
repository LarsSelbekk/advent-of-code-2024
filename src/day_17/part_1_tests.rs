use crate::day_17::part_1::solve;

#[test]
fn sample(){
    assert_eq!(solve("Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"), "4,6,3,5,6,3,5,2,1,0")
}