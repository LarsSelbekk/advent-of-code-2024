use crate::day_03::part_1::solve;
use regex::Regex;

#[test]
fn sample() {
    assert_eq!(
        solve("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
        161
    )
}

#[test]
fn test_regex() {
    let text = "apple 123 orange 456 apple 789";
    let re = Regex::new(r"(\d+)").unwrap();

    let matches: Vec<String> = re.captures_iter(text)
        .filter_map(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        .collect();

    for mat in matches {
        println!("{}", mat);
    }
}