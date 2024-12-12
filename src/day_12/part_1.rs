use colored::{ColoredString, Colorize};
use iter_tools::Itertools;
use ndarray::Array2;
use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    #[cfg(debug_assertions)]
    eprintln!("{}", input);

    let (map, _) = parse_map(input);
    let zones = create_zones(&map);

    print_map(&map, &zones);

    calculate_price(&zones)
}

pub(crate) fn create_zones(map: &Array2<char>) -> Vec<HashSet<(usize, usize)>> {
    let mut zones: Vec<HashSet<(usize, usize)>> = vec![];

    for ((y, x), c) in map.indexed_iter() {
        let [up_pos, left_pos] = [(-1, 0), (0, -1)]
            .map(|(dy, dx)| (y.wrapping_add_signed(dy), x.wrapping_add_signed(dx)));
        let [up_equal, left_equal] =
            [up_pos, left_pos].map(|pos| map.get(pos).map(|c2| c2 == c).unwrap_or_default());

        let [up_zone_index, left_zone_index] = [(up_equal, up_pos), (left_equal, left_pos)]
            .map(|(equal, pos)| equal.then(|| {}).and_then(|_| get_zone_index(&zones, &pos)));

        match (up_zone_index, left_zone_index) {
            (Some(up_i), Some(left_i)) if up_i != left_i => {
                let min = left_i.min(up_i);
                let max = left_i.max(up_i);

                let (slice_1, slice_2) = zones.as_mut_slice().split_at_mut(max);
                let winner = slice_1.get_mut(min).unwrap();
                let loser = slice_2.first_mut().unwrap();

                winner.extend(loser.drain().chain([(y, x)]));

                zones.remove(max);
            }
            (None, Some(i)) | (Some(i), None) | (Some(i), Some(_)) => {
                zones[i].insert((y, x));
            }
            (None, None) => {
                zones.push(HashSet::from([(y, x)]));
            }
        }
    }
    zones
}

pub fn get_zone_index(zones: &Vec<HashSet<(usize, usize)>>, pos: &(usize, usize)) -> Option<usize> {
    zones
        .iter()
        .find_position(|g| g.contains(&pos))
        .map(|(i, _)| i)
}

fn calculate_price(zones: &Vec<HashSet<(usize, usize)>>) -> usize {
    zones
        .iter()
        .map(|zone| {
            let mut area = 0;
            let mut perimeter = 0;
            for &(y, x) in zone.iter() {
                area += 1;
                let num_boundary_sides = [(1, 0), (-1, 0), (0, 1), (0, -1)]
                    .map(|(dy, dx)| {
                        zone.contains(&(y.wrapping_add_signed(dy), x.wrapping_add_signed(dx)))
                    })
                    .iter()
                    .filter(|&&is_same_zone| !is_same_zone)
                    .count();
                perimeter += num_boundary_sides;
            }
            area * perimeter
        })
        .sum()
}

pub fn parse_map(input: &str) -> (Array2<char>, usize) {
    let dim = input.find('\n').unwrap();
    (
        Array2::from_shape_vec(
            (dim, dim),
            input.lines().flat_map(|line| line.chars()).collect_vec(),
        )
        .unwrap(),
        dim,
    )
}

fn print_map(map: &Array2<char>, zones: &Vec<HashSet<(usize, usize)>>) {
    #[cfg(debug_assertions)]
    {
        dbg!(zones.len());
        for (i, c) in map.indexed_iter() {
            eprint!("{}{}", if i.1 == 0 { "\n" } else { "" }, color(c, i, zones))
        }
        eprintln!();
    }
}

fn color(c: &char, i: (usize, usize), zones: &Vec<HashSet<(usize, usize)>>) -> ColoredString {
    let s = c.to_string();
    if let Some((i, _)) = zones.iter().find_position(|zone| zone.contains(&i)) {
        match i {
            0 => s.blue(),
            1 => s.green(),
            2 => s.red(),
            3 => s.yellow(),
            4 => s.bright_purple(),
            5 => s.magenta(),
            6 => s.cyan(),
            7 => s.dimmed(),
            8 => s.bright_blue(),
            9 => s.bright_green(),
            10 => s.bright_red(),
            11 => s.bright_yellow(),
            12 => s.bright_magenta(),
            13 => s.bright_cyan(),
            14 => s.italic(),
            15 => s.bold(),
            16 => s.on_blue(),
            17 => s.on_green(),
            18 => s.on_red(),
            19 => s.on_yellow(),
            20 => s.on_magenta(),
            21 => s.on_cyan(),
            22 => s.on_purple(),
            _ => panic!(),
        }
    } else {
        s.bright_white()
    }
}

pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
