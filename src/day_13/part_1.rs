use iter_tools::Itertools;
use regex::Regex;

pub type Coord = (usize, usize);

pub fn solve(input: &str) -> usize {
    let coords_regex = Regex::new(r".*: X.(\d+), Y.(\d+)").unwrap();
    let mut problems = input.lines().filter(|l| !l.is_empty());

    let mut res = 0;
    while let Some((u, v, r)) = next_problem(&mut problems, &coords_regex) {
        res += solve_problem(u, v, r);
    }
    res
}

pub fn solve_problem(u: Coord, v: Coord, r: Coord) -> usize {
    #[cfg(debug_assertions)]
    dbg!((u, v, r));

    // u_0*a + v_0*b = r_0
    // u_1*a + v_1*b = r_1
    /*
     * [[ u_0 v_0 ]    [[a]   = [[r_0]
     *  [ u_1 v_1 ]]    [b]]  =  [r_1]]
     */

    let u_0 = u.0 as f64;
    let u_1 = u.1 as f64;
    let v_0 = v.0 as f64;
    let v_1 = v.1 as f64;
    let r_0 = r.0 as f64;
    let r_1 = r.1 as f64;

    let mult = v_0 / v_1;
    let a = (r_0 - mult * r_1) / (u_0 - mult * u_1);
    let b = (r_1 - u_1 * a) / v_1;

    if a < 0. || b < 0. {
        return 0;
    }

    let a = a.round() as usize;
    let b = b.round() as usize;

    #[cfg(debug_assertions)]
    dbg!((a, b));
    #[cfg(debug_assertions)]
    dbg!(a * u.0 + b * v.0, r.0, a * u.1 + b * v.1, r.1);

    if a * u.0 + b * v.0 == r.0 && a * u.1 + b * v.1 == r.1 {
        3 * a + b
    } else {
        0
    }
}

pub fn next_problem<'a>(
    input: &mut impl Iterator<Item = &'a str>,
    coords_regex: &Regex,
) -> Option<(Coord, Coord, Coord)> {
    input
        .take(3)
        .map(|coords_line| {
            coords_regex
                .captures(coords_line)
                .unwrap()
                .iter()
                .skip(1)
                .take(2)
                .map(|coord| coord.unwrap().as_str().parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect_tuple()
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
