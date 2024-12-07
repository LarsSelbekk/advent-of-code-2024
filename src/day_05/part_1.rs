use iter_tools::Itertools;
use std::collections::HashSet;

pub fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let requirements = lines
        .take_while_ref(|line| !line.is_empty())
        .map(|line| line.split_once("|").unwrap())
        .map(|(requirement, requirer)| {
            (
                requirer.parse::<usize>().unwrap(),
                requirement.parse::<usize>().unwrap(),
            )
        })
        .into_grouping_map()
        .collect::<HashSet<_>>();

    lines
        .skip(1)
        .map(|line| {
            line.split(",")
                .map(|element| element.parse::<usize>().unwrap())
                .collect_vec()
        })
        .map(|update| {
            let elements_in_update = HashSet::<usize>::from_iter(update.iter().copied());
            let mut verified_elements = HashSet::new();

            for element in update.iter() {
                if requirements
                    .get(element)
                    .unwrap_or(&HashSet::new())
                    .iter()
                    .any(|requirement| {
                        elements_in_update.contains(requirement)
                            && !verified_elements.contains(requirement)
                    })
                {
                    return 0;
                }
                verified_elements.insert(*element);
            }

            return update[update.len() / 2];
        })
        .sum()
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")))
}
