use iter_tools::Itertools;
use std::collections::HashSet;
use std::iter::{empty, once};

type Coords = (usize, usize);

pub fn solve(input: &str) -> usize {
    let dim = input.find('\n').unwrap();
    let mut robot_position = (0, 0);
    let mut obstacle_positions = HashSet::new();
    let mut wall_positions = HashSet::new();
    let mut iter = input.lines();

    iter.by_ref()
        .take_while(|line| !line.is_empty())
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .take(dim)
                .enumerate()
                .for_each(|(x, char)| match char {
                    '.' => {}
                    'O' => {
                        obstacle_positions.insert((y, x * 2));
                    }
                    '@' => robot_position = (y, x * 2),
                    '#' => {
                        wall_positions.insert((y, x * 2));
                        wall_positions.insert((y, x * 2 + 1));
                    }
                    _ => unreachable!(),
                })
        });

    iter.flat_map(|line| {
        line.chars().map(|char| match char {
            '<' => (0, -1),
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            _ => unreachable!(),
        })
    })
    .for_each(|mov| {
        print_map(robot_position, &wall_positions, &obstacle_positions, dim);
        #[cfg(debug_assertions)]
        println!("{}\n", format_direction(mov));
        move_robot_and_obstacles(
            &mut robot_position,
            &mut obstacle_positions,
            mov,
            &wall_positions,
            dim,
        );
    });

    print_map(robot_position, &wall_positions, &obstacle_positions, dim);

    obstacle_positions.iter().map(|(y, x)| 100 * y + x).sum()
}

fn add_position(
    pos: Coords,
    mov: (isize, isize),
    dim: usize,
    walls: &HashSet<Coords>,
) -> Option<Coords> {
    let next_position = (pos.0 as isize + mov.0, pos.1 as isize + mov.1);
    if next_position.0 < 0
        || next_position.1 < 0
        || next_position.0 >= dim as _
        || next_position.1 >= dim as isize * 2
    {
        return None;
    }

    let next_position = (next_position.0 as _, next_position.1 as _);
    if !walls.contains(&next_position) {
        Some(next_position)
    } else {
        None
    }
}

fn move_robot_and_obstacles(
    robot_pos: &mut Coords,
    obstacle_positions: &mut HashSet<Coords>,
    mov: (isize, isize),
    walls: &HashSet<Coords>,
    dim: usize,
) {
    if let Some(next_position) = add_position(*robot_pos, mov, dim, walls) {
        if let Some(obstacle_moves) =
            get_pushable_obstacles(next_position, mov, walls, obstacle_positions, dim)
        {
            *robot_pos = next_position;
            for (old, new) in obstacle_moves.unique_by(|a| a.0) {
                if obstacle_positions.remove(&old) {
                    obstacle_positions.insert(new);
                }
                // print_map(*robot_pos, walls, obstacle_positions, dim);
            }
        }
    }
}

fn get_pushable_obstacles(
    pos: Coords,
    mov: (isize, isize),
    walls: &HashSet<Coords>,
    obstacles: &HashSet<Coords>,
    dim: usize,
) -> Option<Box<dyn Iterator<Item = (Coords, Coords)>>> {
    let left = (pos.0, pos.1.wrapping_add_signed(-1));
    if !obstacles.contains(&left) && !obstacles.contains(&pos) {
        return Some(Box::new(empty()));
    }
    let (root, other) = if obstacles.contains(&pos) {
        (pos, add_position(pos, (0, 1), dim, walls).unwrap())
    } else {
        (left, pos)
    };
    if mov.0 == 0 {
        // horizontally
        let next_pos = add_position(pos, mov, dim, walls)?;
        let next_next_pos = add_position(next_pos, mov, dim, walls)?;
        Some(Box::new(
            get_pushable_obstacles(next_next_pos, mov, walls, obstacles, dim)?
                .chain(once((root, add_position(root, mov, dim, walls).unwrap()))),
        ))
    } else {
        // vertically
        let above_left = add_position(root, mov, dim, walls)?;
        let above_right = add_position(other, mov, dim, walls)?;
        let left_res = get_pushable_obstacles(above_left, mov, walls, obstacles, dim)?;
        let right_res = get_pushable_obstacles(above_right, mov, walls, obstacles, dim)?;
        Some(Box::new(left_res.chain(right_res).chain(once((
            root,
            add_position(root, mov, dim, walls).unwrap(),
        )))))
    }
}

fn print_map(
    robot_position: Coords,
    wall_positions: &HashSet<Coords>,
    obstacle_positions: &HashSet<Coords>,
    dim: usize,
) {
    #[cfg(not(debug_assertions))]
    return;

    let mut s = String::with_capacity(2 * (dim * dim + dim));
    for y in 0..dim {
        for x in 0..(dim * 2) {
            s.push(if robot_position == (y, x) {
                '@'
            } else if obstacle_positions.contains(&(y, x)) {
                '['
            } else if obstacle_positions.contains(&(y, x.wrapping_sub(1))) {
                ']'
            } else if wall_positions.contains(&(y, x)) {
                '#'
            } else {
                '.'
            });
        }
        s.push('\n');
    }

    print!("{}", s);
}

fn format_direction(mov: (isize, isize)) -> char {
    match mov {
        (-1, 0) => '\u{2191}',
        (1, 0) => '\u{2193}',
        (0, -1) => '\u{2190}',
        (0, 1) => '\u{2192}',
        _ => unreachable!(),
    }
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
