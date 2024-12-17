use crate::day_17::part_1;
use crate::day_17::part_1::{Instruction::*, Operand, A, B, C};
use rayon::prelude::*;
use std::ops::BitXor;

pub fn backwards_solve() -> usize {
    backwards_solve_inner(0, 16).expect("Failed to find a solution")
}

/// Premise of the solution: can we build the A register backwards?
///
/// The output has length 16, and the parsed program loops as long as A > 0, with 1 output per
/// iteration. This means that there are 16 iterations.
/// B and C are reset every iteration, derived from register A.
/// A is right-shifted by 3 every iteration, and A is 0 after the last iteration, which means that
/// the last iteration can only use the three left-most bits of A, counting from the left-most
/// 1-bit. Furthermore, the second-to-last iteration can only use the left-most 6, etc with the
/// n-most iteration only having the left-most 3n bits. This means that there are only 8 possible
/// solutions to the last iteration, and it can be solved independently. Then when the last is
/// solved, the same is true for the second to last, etc.
///
///  Find z in 0..2^3 s.t. func(z) = output\[15]
///  Find y in 0..2^3 s.t. func(zy) = output\[14]
///  Find x in 0..2^3 s.t. func(zyx) = output\[13]
///  Find w in 0..2^3 s.t. func(zyxw) = output\[12]
///  Find v in 0..2^3 s.t. func(zyxwv) = output\[11]
///  ...
///  Find k in 0..2^3 s.t. func(z..k) = output\[0]
///
/// At least in my input, I never had to backtrack to I didn't really need to return an Option,
/// but I included it because it feels more robust
fn backwards_solve_inner(
    previous_partial_solution: usize,
    remaining_iterations: usize,
) -> Option<usize> {
    if remaining_iterations == 0 {
        return Some(previous_partial_solution);
    }
    for iteration_solution in 0..8 {
        let partial_solution = (previous_partial_solution << 3) + iteration_solution;
        if output_single_value(partial_solution) == EXPECTED_OUTPUT[remaining_iterations - 1] {
            if let Some(solution) =
                backwards_solve_inner(partial_solution, remaining_iterations - 1)
            {
                return Some(solution);
            }
        }
    }

    None
}

const EXPECTED_OUTPUT: [usize; 16] = [2, 4, 1, 2, 7, 5, 4, 5, 1, 3, 5, 5, 0, 3, 3, 0];

/// The output of a single iteration of the parsed program
/// Can be simplified a lot, but kept simple to reduce complexity
pub fn output_single_value(a: usize) -> usize {
    // 2, 4: B = A % 8
    let mut b = a % 8;
    // 1, 2: B = B xor 2
    b = b.bitxor(2);
    // 7, 5: C = A / 2^B
    let c = a / 2usize.pow(b as _);
    // 4, 5: B = B xor C
    b = b.bitxor(c);
    // 1, 3: B = B xor 3
    b = b.bitxor(3);
    // 5, 5: out B % 8
    b % 8
    // 0, 3: A = A / 2^3
    // 3, 0: jmp ignored

    // Can be simplified to
    // (((a % 8) ^ 2) ^ (a / 2usize.pow(((a % 8) ^ 2) as _)) ^ 3) % 8
}

/// The program I manually parsed from my input ;p
#[allow(unused)]
fn run_parsed_program(initial_a: usize) {
    let mut a = initial_a;
    let mut b = 0;
    let mut c = 0;

    // Program: 2,4,1,2,7,5,4,5,1,3,5,5,0,3,3,0
    while a != 0 {
        // 2, 4: B = A % 8
        b = a % 8;
        // 1, 2: B = B xor 2
        b = b.bitxor(2);
        // 7, 5: C = A / 2^B
        c = a / 2usize.pow(b as _);
        // 4, 5: B = B xor C
        b = b.bitxor(c);
        // 1, 3: B = B xor 3
        b = b.bitxor(3);
        // 5, 5: out B % 8
        print!("{}", b % 8);
        // 0, 3: A = A / 2^3
        a /= 2usize.pow(3u32);
        // 3, 0: jmp to instruction 0 if A != 0
    }
}

/// Too slow to find the solution in a reasonable amount of time, sadly
#[allow(unused)]
pub fn brute_force_solve(input: &str) -> usize {
    let (instructions, registers) = part_1::parse_input(input);
    (0..)
        .find(|&i| is_solution(&instructions, i, [i, registers[B], registers[C]]))
        .unwrap()
}

fn is_solution(instructions: &[usize], initial_a: usize, mut registers: [usize; 3]) -> bool {
    let mut next_instruction_index = 0;
    let mut num_outputted = 0;

    while let (Some(&instruction), Some(&operand)) = (
        instructions.get(next_instruction_index),
        instructions.get(next_instruction_index + 1),
    ) {
        let instruction: part_1::Instruction = instruction.try_into().unwrap();
        let operand =
            Operand::from_instruction_and_operand_code(instruction, operand).value(&registers);

        match instruction {
            Div(output_reg) => registers[output_reg] = registers[A] / 2usize.pow(operand as _),
            Bxl => registers[B] = registers[B].bitxor(operand),
            Bst => registers[B] = operand % 8,
            Jnz => {
                if registers[A] != 0 {
                    next_instruction_index = operand
                }
            }
            Bxc => registers[B] = registers[B].bitxor(registers[C]),
            Out => {
                if operand % 8 != *instructions.get(num_outputted).unwrap_or(&8) {
                    #[cfg(debug_assertions)]
                    if num_outputted >= 8 {
                        println!(
                            "Wrong output after {num_outputted}, next was {},   i was {initial_a}",
                            operand % 8
                        );
                    }
                    return false;
                }
                num_outputted += 1;
                if num_outputted == instructions.len() {
                    return true;
                }
            }
        }
        if instruction != Jnz || registers[A] == 0 {
            next_instruction_index += 2;
        }
    }

    #[cfg(debug_assertions)]
    if num_outputted >= 8 {
        println!("Wrongly terminated after {num_outputted},     i was {initial_a}")
    }
    num_outputted == instructions.len()
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", backwards_solve())
}
