use regex::Regex;

pub fn solve(input: &str) -> u32 {
    let mut sum = 0;
    let mut enabled = true;

    for captures in Regex::new(r"mul\((?<mul>[0-9]+,[0-9]+)\)|(?<toggle>do\(\)|don't\(\))")
        .unwrap()
        .captures_iter(input)
    {
        if let Some(mul_capture) = captures.name("mul") {
            if enabled {
                sum += mul_capture
                    .as_str()
                    .split_once(",")
                    .map(|(left, right)| {
                        left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap()
                    })
                    .unwrap();
            }
        } else if let Some(toggle_capture) = captures.name("toggle") {
            enabled = toggle_capture.as_str() == "do()";
        }
    }
    sum
}

pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
