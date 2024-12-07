use iter_tools::Itertools;
use regex::Regex;

pub fn solve(input: &str) -> u32 {
    let xmas = Regex::new(r"XMAS").unwrap();
    let dim = input.lines().next().unwrap().len();
    let matrix = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    (-1_i32..=1)
        .cartesian_product(-1_i32..=1)
        .filter(|&(x, y)| x != 0 || y != 0)
        .map(|(x_step, y_step)| {
            if x_step.abs() == y_step.abs() {
                let sum = (0..dim * 2 - 1)
                    .map(|count| {
                        let first = if count < dim { count } else { dim - 1 };
                        let second = if count >= dim { count + 1 - dim } else { 0 };
                        let (x, y) = match (x_step, y_step) {
                            (1, 1) => (second, dim - 1 - first),   // ltr ttb
                            (-1, 1) => (first, second),            // rtl ttb
                            (1, -1) => (second, first),            // ltr btt
                            (-1, -1) => (first, dim - 1 - second), // rtl btt
                            _ => unreachable!(),
                        };
                        sum_over_line(&matrix, (x, y), (x_step, y_step), dim, &xmas)
                    })
                    .sum::<u32>();
                dbg!(x_step, y_step, sum);
                dbg!();
                sum
            } else {
                let sum = (0..dim)
                    .map(|count| {
                        let (x, y) = match (x_step, y_step) {
                            (1, 0) => (0, count),
                            (0, 1) => (count, 0),
                            (-1, 0) => (dim - 1, count),
                            (0, -1) => (count, dim - 1),
                            _ => unreachable!(),
                        };
                        sum_over_line(&matrix, (x, y), (x_step, y_step), dim, &xmas)
                    })
                    .sum();
                dbg!(x_step, y_step, sum);
                dbg!();
                sum
            }
        })
        .sum::<u32>()
}

fn sum_over_line(
    matrix: &Vec<Vec<char>>,
    start_pos: (usize, usize),
    step: (i32, i32),
    dim: usize,
    xmas: &Regex,
) -> u32 {
    let (x_step, y_step) = step;
    let (start_x, start_y) = start_pos;

    let mut x = start_x;
    let mut y = start_y;

    let mut s = String::with_capacity(dim);

    while let Some(res) = matrix.get(y).map(|row| row.get(x)).flatten() {
        s.push(*res);
        x = (x as i32 + x_step) as _;
        y = (y as i32 + y_step) as _;
    }
    xmas.find_iter(&s).count() as u32
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
