use std::collections::VecDeque;

pub fn solve(input: &str) -> usize {
    let mut res = vec![];
    let mut map = input
        .chars()
        .map(|c| c as u8 - 48)
        .enumerate()
        .collect::<VecDeque<_>>();
    'outer: while let Some((index, value)) = map.pop_front() {
        if index % 2 == 0 {
            res.append(&mut vec![index / 2; value as _]);
        } else {
            let mut remaining_spaces = value;
            while remaining_spaces > 0 {
                if let Some((i, length_of_file)) = map.pop_back() {
                    if i % 2 == 0 {
                        let file_id = i / 2;
                        let to_take = remaining_spaces.min(length_of_file);
                        res.append(&mut vec![file_id; to_take as _]);
                        if to_take < length_of_file {
                            map.push_back((file_id * 2, length_of_file - to_take));
                        }
                        remaining_spaces -= to_take;
                    }
                } else {
                    break 'outer;
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    eprintln!("\n{:?}", res);

    res.iter()
        .enumerate()
        .map(|(i, file_id)| i * (*file_id))
        .sum()
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")))
}
