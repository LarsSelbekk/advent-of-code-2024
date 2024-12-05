use iter_tools::Itertools;
use ndarray::{Array2, ArrayView, Ix2};
use regex::Regex;

pub fn solve(input: &str) -> u32 {
    let mas = Regex::new(r"MAS").unwrap();
    let dim = input.lines().next().unwrap().len();
    let matrix = Array2::from_shape_vec(
        (dim, dim),
        input.lines().flat_map(|line| line.chars()).collect_vec(),
    )
    .unwrap();
    matrix
        .windows((3, 3))
        .into_iter()
        .filter(|window| {
            let is_match = all_diagonal_lines()
                .filter(|line| is_line_match(&window, line, 3, &mas))
                .count()
                > 1;
            if is_match {
                eprintln!("Window\n{:?} is match", window);
            }
            is_match
        })
        .count() as u32
}

struct LineDesc {
    start_pos: (usize, usize),
    step: (i32, i32),
}

pub fn all_diagonal_lines() -> impl Iterator<Item = LineDesc> {
    const DIM: i32 = 3;
    (-1..=1)
        .step_by(2)
        .cartesian_product((-1..=1).step_by(2))
        .map(|(x_step, y_step)| {
            let (x, y) = match (x_step, y_step) {
                (1, 1) => (0, 0),               // ltr ttb
                (-1, 1) => (DIM - 1, 0),        // rtl ttb
                (1, -1) => (0, DIM - 1),        // ltr btt
                (-1, -1) => (DIM - 1, DIM - 1), // rtl btt
                _ => unreachable!(),
            };
            LineDesc {
                start_pos: (x as _, y as _),
                step: (x_step, y_step),
            }
        })
}

fn is_line_match(
    mat: &&ArrayView<char, Ix2>,
    line_desc: &LineDesc,
    dim: usize,
    xmas: &Regex,
) -> bool {
    let (x_step, y_step) = line_desc.step;
    let (start_x, start_y) = line_desc.start_pos;

    let mut x = start_x;
    let mut y = start_y;

    let mut s = String::with_capacity(dim);

    while let Some(res) = mat.get((y, x)) {
        s.push(*res);
        x = (x as i32 + x_step) as _;
        y = (y as i32 + y_step) as _;
    }
    xmas.is_match(&s)
}

pub(crate) fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
