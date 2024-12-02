pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| {
            let lefts = line.split(' ').map(|v| v.parse::<u32>().unwrap());
            let mut zip = lefts.clone().zip(lefts.skip(1)).peekable();
            let ordering = {
                let (left, right) = zip.peek().unwrap();
                left.cmp(&right)
            };
            zip.all(|(left, right)| {
                left.cmp(&right) == ordering
                    && left.abs_diff(right) >= 1
                    && left.abs_diff(right) <= 3
            })
        })
        .count() as u32
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
