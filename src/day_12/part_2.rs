use crate::day_12::part_1;
use crate::day_12::part_1::get_zone_index;
use iter_tools::Itertools;
use ndarray::{ArrayView1, Axis};
use std::collections::HashMap;
use std::iter::once;

pub fn solve(input: &str) -> usize {
    let (map, dim) = part_1::parse_map(input);
    let zones = part_1::create_zones(&map);

    let mut perimeters = HashMap::<usize, usize>::new();
    let vec = vec!['\0'; dim];
    let null_axis = ArrayView1::from(&vec);

    for ((_, left_axis), (right_x, right_axis)) in
        [Axis(0), Axis(1)].iter().flat_map(|&axis_index| {
            once(null_axis)
                .enumerate()
                .chain(
                    map.axis_iter(axis_index)
                        .enumerate()
                        .chain(once(null_axis).enumerate()),
                )
                .tuple_windows()
        })
    {
        let mut iter = left_axis.iter().zip(right_axis).enumerate();
        let mut previous = None;
        while let Some((y, (left, right))) = iter.next() {
            if left != right && previous == Some(right) {
                continue;
            }
            let is_right_end = right_x == dim;
            let pos = (y, right_x - if is_right_end { 1 } else { 0 });
            *perimeters
                .entry(get_zone_index(&zones, &pos).unwrap())
                .or_insert(Default::default()) += 1;
            previous = Some(if is_right_end {left} else {right});
        }
    }

    perimeters
        .iter()
        .map(|(i, &perim)| perim * zones[*i].len())
        .sum()
}
