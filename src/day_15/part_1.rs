use std::collections::HashSet;

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
                        obstacle_positions.insert((y, x));
                    }
                    '@' => robot_position = (y, x),
                    '#' => {
                        wall_positions.insert((y, x));
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
        println!("{:?}", mov);
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
    pos: (usize, usize),
    mov: (isize, isize),
    dim: usize,
    walls: &HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
    let next_position = (pos.0 as isize + mov.0, pos.1 as isize + mov.1);
    if next_position.0 < 0
        || next_position.1 < 0
        || next_position.0 >= dim as _
        || next_position.1 >= dim as _
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
    robot_pos: &mut (usize, usize),
    obstacle_positions: &mut HashSet<(usize, usize)>,
    mov: (isize, isize),
    walls: &HashSet<(usize, usize)>,
    dim: usize,
) {
    let mut last_obstacle_position = *robot_pos;
    while let Some(next_obstacle_position) = add_position(last_obstacle_position, mov, dim, walls) {
        last_obstacle_position = next_obstacle_position;
        if !obstacle_positions.contains(&next_obstacle_position) {
            *robot_pos = add_position(*robot_pos, mov, dim, walls).unwrap();
            obstacle_positions.remove(robot_pos);
            if last_obstacle_position != *robot_pos {
                obstacle_positions.insert(last_obstacle_position);
            }
            return;
        }
    }
}

fn print_map(
    robot_position: (usize, usize),
    wall_positions: &HashSet<(usize, usize)>,
    obstacle_positions: &HashSet<(usize, usize)>,
    dim: usize,
) {
    #[cfg(not(debug_assertions))]
    return;

    let mut s = String::with_capacity(dim * dim + dim);
    for y in 0..dim {
        for x in 0..dim {
            s.push(if robot_position == (y, x) {
                '@'
            } else if obstacle_positions.contains(&(y, x)) {
                'O'
            } else if wall_positions.contains(&(y, x)) {
                '#'
            } else {
                '.'
            });
        }
        s.push('\n');
    }

    println!("{}", s);
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
