use crate::day_12::part_1;
use crate::day_12::part_1::get_zone_index;
use colored::{ColoredString, Colorize};
use iter_tools::{repeat_n, Itertools};
use ndarray::{Array2, Axis};
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> usize {
    let (map, dim) = part_1::parse_map(input);
    let zones = part_1::create_zones(&map);
    let mut perimeters = HashMap::<usize, usize>::new();

    part_1::print_map(&map, &zones);
    print_zones(&map, &zones);

    let zone_map = Array2::from_shape_fn((dim, dim), |pos| get_zone_index(&zones, &pos).unwrap());
    for (rotated, reverse) in [false, true]
        .iter()
        .copied()
        .cartesian_product([false, true])
    {
        #[cfg(debug_assertions)]
        dbg!(rotated, reverse);

        let axis = Axis(if rotated { 1 } else { 0 });
        let edge = zone_map.index_axis(axis, if reverse { dim - 1 } else { 0 });
        let pre_step = edge.iter().copied().zip(repeat_n(None, dim));

        print_step(pre_step.clone(), &map, &zones);
        let counts_from_first_edge = count_edges(pre_step);
        print_perimeters(&counts_from_first_edge, &zones, &map);
        add_counts(&mut perimeters, &counts_from_first_edge);

        for major in 1..dim {
            #[cfg(debug_assertions)]
            dbg!(major);

            let secondary_major = major - 1;

            let step = (0..dim)
                .map(|minor| match (rotated, reverse) {
                    (false, false) => ((major, minor), (secondary_major, minor)),
                    (true, false) => ((minor, major), (minor, secondary_major)),
                    (true, true) => ((minor, dim - major - 1), (minor, dim - secondary_major - 1)),
                    (false, true) => ((dim - major - 1, minor), (dim - secondary_major - 1, minor)),
                })
                .map(|(primary, secondary)| (zone_map[primary], Some(zone_map[secondary])));

            print_step(step.clone(), &map, &zones);
            let step_counts = count_edges(step);
            print_perimeters(&step_counts, &zones, &map);
            add_counts(&mut perimeters, &step_counts);
        }
    }

    print_zone_costs(&map, &zones, &mut perimeters);

    perimeters
        .iter()
        .map(|(i, &perim)| perim * zones[*i].len())
        .sum()
}

fn print_zone_costs(
    map: &Array2<char>,
    zones: &Vec<HashSet<(usize, usize)>>,
    perimeters: &mut HashMap<usize, usize>,
) {
    #[cfg(not(debug_assertions))]
    return;

    for (zone_id, &perimeter) in perimeters.iter() {
        let area = zones[*zone_id].len();
        eprintln!(
            "{}: {} * {} = {}",
            get_zone_name(zone_id, &map, &zones),
            area,
            perimeter,
            perimeter * area
        )
    }
}

fn print_zones(map: &Array2<char>, zones: &Vec<HashSet<(usize, usize)>>) {
    #[cfg(debug_assertions)]
    eprintln!(
        "{}",
        (0..zones.len())
            .map(|zone_id| format!("{}: {}", zone_id, get_zone_name(&zone_id, &map, &zones)))
            .join("\n")
    );
}

fn print_step(
    step: impl Iterator<Item = (usize, Option<usize>)>,
    map: &Array2<char>,
    zones: &Vec<HashSet<(usize, usize)>>,
) {
    #[cfg(not(debug_assertions))]
    return;

    let (primary_zones_ids, secondary_zone_ids): (Vec<usize>, Vec<Option<usize>>) = step.unzip();

    eprintln!(
        "{} (primary)",
        primary_zones_ids
            .into_iter()
            .map(|primary| get_zone_name(&primary, map, zones))
            .join("")
    );

    eprintln!(
        "{}",
        secondary_zone_ids
            .into_iter()
            .map(|secondary| secondary
                .map(|secondary| get_zone_name(&secondary, map, zones))
                .unwrap_or(" ".on_white()))
            .join("")
    );
}

fn add_counts(into: &mut HashMap<usize, usize>, to_add: &HashMap<usize, usize>) {
    for (key, value) in to_add {
        *into.entry(*key).or_insert(0) += value;
    }
}

fn count_edges(step: impl Iterator<Item = (usize, Option<usize>)>) -> HashMap<usize, usize> {
    let mut perimeters = HashMap::new();
    let mut previous = None;

    let counts = step
        .filter_map(|(primary, secondary)| {
            if previous == Some(primary) && secondary != Some(primary) {
                return None;
            }
            if secondary == Some(primary) {
                previous = None;
                return None;
            }
            previous = Some(primary);
            return Some(primary);
        })
        .counts();

    for (zone_id, count) in counts {
        *perimeters.entry(zone_id).or_insert(0) += count;
    }
    perimeters
}

fn print_perimeters(
    perimeters: &HashMap<usize, usize>,
    zones: &Vec<HashSet<(usize, usize)>>,
    map: &Array2<char>,
) {
    #[cfg(not(debug_assertions))]
    return;

    for (zone_id, perimeter) in perimeters {
        let representative = *zones[*zone_id].iter().next().unwrap();
        println!(
            "  {}: {}",
            part_1::format_zone(&map[representative], representative, &zones),
            perimeter
        )
    }
    println!()
}

fn get_zone_name(
    zone_index: &usize,
    map: &Array2<char>,
    zones: &Vec<HashSet<(usize, usize)>>,
) -> ColoredString {
    let representative = *zones[*zone_index].iter().next().unwrap();
    let char = map[representative];
    let representative = *zones[*zone_index].iter().next().unwrap();
    part_1::format_zone(&char, representative, &zones)
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
