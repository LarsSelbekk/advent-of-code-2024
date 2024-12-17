use crate::day_17::part_1::Operand::{Literal, Register};
use iter_tools::Itertools;
use std::ops::BitXor;
use Instruction::*;

pub(crate) const A: usize = 0;
pub const B: usize = 1;
pub const C: usize = 2;

pub fn solve(input: &str) -> String {
    let (instructions, mut registers) = parse_input(input);
    let mut output = Vec::new();
    let mut next_instruction_index = 0;

    while let (Some(&instruction), Some(&operand)) = (
        instructions.get(next_instruction_index),
        instructions.get(next_instruction_index + 1),
    ) {
        let instruction: Instruction = instruction.try_into().unwrap();
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
            Out => output.push(operand % 8),
        }
        if instruction != Jnz || registers[A] == 0 {
            next_instruction_index += 2;
        }
    }
    output.into_iter().join(",")
}

pub enum Operand {
    Literal(usize),
    Register(usize),
}

impl Operand {
    pub fn from_instruction_and_operand_code(
        instruction: Instruction,
        operand_code: usize,
    ) -> Self {
        match (operand_code, instruction.is_combo()) {
            (j, false) => Literal(j),
            (j, true) if (0..=3).contains(&j) => Literal(j),
            (j, true) => Register(j - 4),
        }
    }

    pub fn value(&self, registers: &[usize; 3]) -> usize {
        match self {
            Literal(l) => *l,
            Register(reg) => registers[*reg],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    Div(usize),
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
}

impl Instruction {
    fn is_combo(&self) -> bool {
        match self {
            Div(_) | Bst | Out => true,
            Bxl | Jnz | Bxc => false,
        }
    }
}

impl TryFrom<usize> for Instruction {
    type Error = ();

    fn try_from(s: usize) -> Result<Self, Self::Error> {
        match s {
            0 => Ok(Div(0)), // Adiv
            1 => Ok(Bxl),
            2 => Ok(Bst),
            3 => Ok(Jnz),
            4 => Ok(Bxc),
            5 => Ok(Out),
            6 => Ok(Div(1)), // Bdiv
            7 => Ok(Div(2)), // Cdiv
            _ => Err(()),
        }
    }
}

pub fn parse_input(input: &str) -> (Vec<usize>, [usize; 3]) {
    let mut lines = input.lines();
    let register_num_index = "Register A: ".len();
    let instructions_num_index = "Program: ".len();
    let register_a = lines.next().unwrap()[register_num_index..].parse().unwrap();
    let register_b = lines.next().unwrap()[register_num_index..].parse().unwrap();
    let register_c = lines.next().unwrap()[register_num_index..].parse().unwrap();
    let instructions = lines.nth(1).unwrap()[instructions_num_index..]
        .split(",")
        .map(|op| op.parse::<usize>().unwrap())
        .collect_vec();
    (instructions, [register_a, register_b, register_c])
}

#[allow(unused)]
pub fn print_answer() {
    println!("{}", solve(include_str!("input.txt")));
}
