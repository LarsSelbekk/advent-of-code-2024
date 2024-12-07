use iter_tools::Itertools;

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let (desired_result, operands) = line.split_once(": ").unwrap();
            let desired_result = desired_result.parse::<usize>().unwrap();
            let operands = operands
                .split(' ')
                .map(|arg| arg.parse::<usize>().unwrap())
                .collect_vec();

            let is_solvable = (0..3_u32.pow((operands.len() - 1) as u32)).any(|tritmap| {
                let mut operands_iter = operands.iter();
                let mut computed = *operands_iter.next().unwrap();
                for (operand_index, arg) in operands_iter.enumerate() {
                    match get_trit(tritmap, operand_index) {
                        0 => computed += arg,
                        1 => computed *= arg,
                        2 => computed = concat(computed, *arg),
                        _ => unreachable!(),
                    }
                }
                computed == desired_result
            });
            if is_solvable {
                Some(desired_result)
            } else {
                None
            }
        })
        .sum()
}

fn concat(left: usize, right: usize) -> usize {
    // 123 || 24 = 12324
    // = 12300 + 24
    // = 123 * 10^2 + 24
    // = 123 * 10^(ilog10(24)+1)
    left * 10_u32.pow(right.ilog10() + 1) as usize + right
}

pub fn get_trit(i: u32, trit_index: usize) -> u32 {
    // Get a specific trit of a trinary number
    //  9 = 100 = 1*3^2 + 0*3^1 + 0*3^0
    //            ^       ^       ^
    // 23 = 212 = 2*3^2 + 1*3^1 + 2*3^0
    //            ^       ^       ^

    // In binary, we have
    // 01011       i
    //   ^         i & 00100
    //    ^^       i & 00011 <=>     (i % 00100)
    // ^^          i & 11000 <=> i - (i % 01000)

    // In trinary, we don't have the bitwise and (&) operator, so we have to use the equivalent
    // modulo formulation. For trinary number i and trit index o:
    // 21202  i
    //    ^^  i % 3^o
    // ^^     i - (i % 3^(o+1))
    //   ^    i - (i % 3^o) - (i - (i % 3^(o+1)))
    //        = - (i % 3^o) + (i % 3^(o+1))
    // In addition, since we're only interested in the trit itself (and not the value of that
    // trit raised to the power of 3), we have to divide it by 3^o

    ((i % 3_u32.pow((trit_index + 1) as _)) - (i % 3_u32.pow(trit_index as _)))
        / 3_u32.pow(trit_index as _)
}

pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
