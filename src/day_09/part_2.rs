use iter_tools::Itertools;

pub fn solve(input: &str) -> usize {
    let (files, mut spaces, mut result) = parse_input(input);

    let mut file_moved_to = vec![None; files.len()];

    for file in files.iter().rev() {
        if let Some((space_vec_index, space)) =
            spaces.iter().find_position(|&space| space.len >= file.len)
        {
            if space.index < file.index {
                file_moved_to[file.id] = Some(space.index);
                let remaining_space = space.len - file.len;
                if remaining_space > 0 {
                    spaces[space_vec_index] = Space::new(space.index + file.len, remaining_space);
                } else {
                    spaces.remove(space_vec_index);
                }
                insert_space_for_file(&mut spaces, &file);
            }
        }
    }

    for file in files {
        let file_index = if let Some(space_index) = file_moved_to[file.id] {
            space_index
        } else {
            file.index
        };
        result[file_index..file_index + file.len].clone_from_slice(&vec![file.id; file.len]);
    }

    result
        .iter()
        .enumerate()
        .map(|(i, &file_id)| i * file_id)
        .sum()
}

fn insert_space_for_file(spaces: &mut Vec<Space>, file: &&File) {
    let space_insertion_index = spaces
        .binary_search_by_key(&file.index, |space| space.index)
        .err()
        .unwrap();
    match spaces.get_mut(space_insertion_index.overflowing_sub(1).0) {
        Some(space) if space.index + space.len == file.index => {
            space.len += file.len;
        }
        _ => spaces.insert(space_insertion_index, Space::new(file.index, file.len)),
    }
}

fn parse_input(input: &str) -> (Vec<File>, Vec<Space>, Vec<usize>) {
    let mut sum_width = 0;
    let (files, spaces) = input
        .chars()
        .enumerate()
        .map(|(i, len)| {
            let len = len.to_digit(10).unwrap() as usize;
            let pos = sum_width;
            sum_width += len;
            (i, pos, len)
        })
        .partition::<Vec<_>, _>(|(i, _, _)| i % 2 == 0);

    let files = files
        .iter()
        .map(|&(i, index, len)| File::new(i / 2, index, len))
        .collect_vec();
    let spaces = spaces
        .iter()
        .filter(|&&(_, _, len)| len > 0)
        .map(|&(_, index, len)| Space::new(index, len))
        .collect_vec();

    (files, spaces, vec![0; sum_width])
}

struct File {
    id: usize,
    index: usize,
    len: usize,
}

impl File {
    pub fn new(id: usize, index: usize, len: usize) -> Self {
        Self { id, index, len }
    }
}

struct Space {
    index: usize,
    len: usize,
}

impl Space {
    pub fn new(index: usize, len: usize) -> Self {
        Self { index, len }
    }
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")))
}
