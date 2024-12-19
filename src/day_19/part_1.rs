pub fn solve(input: &str) -> usize {
    let (patterns, designs) = parse_input(input);

    designs
        .iter()
        .filter(|design| can_build_design(&patterns, design))
        .count()
}

fn can_build_design(patterns: &Vec<&str>, design: &str) -> bool {
    design.is_empty()
        || patterns.iter().any(|pattern| {
            design.starts_with(pattern) && can_build_design(patterns, &design[pattern.len()..])
        })
}

pub(crate) fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut lines = input.lines();
    let patterns = lines.next().unwrap().split(", ").collect();
    let designs = lines.skip(1).collect();
    (patterns, designs)
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
