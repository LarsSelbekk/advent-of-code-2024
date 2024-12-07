pub fn solve(input: &str) -> usize {
    input.lines().filter_map(|line| {
        let (result, args) = line.split_once(": ").unwrap();
        let result = result.parse::<usize>().unwrap();
        let args = args.split(' ').map(|arg| arg.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let is_solvable = (0..2_u32.pow((args.len() - 1) as u32)).any(|i| {
            let mut iter = args.iter();
            let mut computed = *iter.next().unwrap();
            for (op_index, arg) in iter.enumerate() {
                if i & (1 << op_index) == 0 {
                    computed += arg;
                } else {
                    computed *= arg;
                }
            }
            result == computed
        });
        if is_solvable {
            Some(result)
        } else {
            None
        }
    }).sum()
}

pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}