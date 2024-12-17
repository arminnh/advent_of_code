use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;
use std::usize;

type Program = Vec<u32>;

#[derive(Debug)]
struct Registers {
    a: u32,
    b: u32,
    c: u32,
}

// instructions take an given operand and a 'combo' operand which is determined by a function
#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    ADV, // divide A by 2^combo -> store in A
    BXL, // bitwise XOR B and operand -> store in B
    BST, // combo operand % 8 -> store in B
    JNZ, // if A != 0, jump to operand
    BXC, // bitwise XOR B and C -> store in B
    OUT, // output combo % 8
    BDV, // adv, but store in B
    CDV, // adv, but store in C
}

fn parse_input(mut lines: Lines) -> (Registers, Program) {
    let mut next_register = || {
        lines
            .next()
            .expect("Could not read next line for register")
            .split_once(": ")
            .expect("Invalid register line")
            .1
            .parse()
            .expect("Could not parse register")
    };
    let a = next_register();
    let b = next_register();
    let c = next_register();

    let program = lines
        .skip(1)
        .next()
        .expect("Could not read line for program")
        .split_once(": ")
        .expect("Invalid line for program")
        .1
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    (Registers { a, b, c }, program)
}

impl Instruction {
    fn from(opcode: u32) -> Instruction {
        match opcode {
            0 => Instruction::ADV,
            1 => Instruction::BXL,
            2 => Instruction::BST,
            3 => Instruction::JNZ,
            4 => Instruction::BXC,
            5 => Instruction::OUT,
            6 => Instruction::BDV,
            7 => Instruction::CDV,
            _ => panic!("Invalid instruction"),
        }
    }

    fn execute(
        &self,
        registers: &mut Registers,
        operand: u32,
        instruction_pointer: &mut usize,
        output: &mut Vec<u32>,
    ) {
        match self {
            Instruction::ADV => registers.a /= 2u32.pow(combo_operand(operand, &registers)),
            Instruction::BXL => registers.b ^= operand,
            Instruction::BST => registers.b = combo_operand(operand, &registers) % 8,
            Instruction::JNZ => {
                if registers.a != 0 {
                    *instruction_pointer = operand as usize;
                } else {
                    *instruction_pointer += 2;
                }
            }
            Instruction::BXC => registers.b ^= registers.c,
            Instruction::OUT => output.push(combo_operand(operand, &registers) % 8),
            Instruction::BDV => {
                registers.b = registers.a / 2u32.pow(combo_operand(operand, &registers))
            }
            Instruction::CDV => {
                registers.c = registers.a / 2u32.pow(combo_operand(operand, &registers))
            }
        }

        if *self != Instruction::JNZ {
            *instruction_pointer += 2;
        }
    }
}

fn combo_operand(operand: u32, registers: &Registers) -> u32 {
    match operand {
        0..=3 => operand,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        _ => panic!("Invalid operand > 6 for combo operand."),
    }
}

fn run_program(program: Vec<u32>, registers: &mut Registers) -> Vec<u32> {
    let mut instruction_pointer = 0;
    let mut output = Vec::new();

    while let Some(&opcode) = program.get(instruction_pointer) {
        let operand = *program
            .get(instruction_pointer + 1)
            .expect("Operand out of bounds");

        Instruction::from(opcode).execute(
            registers,
            operand,
            &mut instruction_pointer,
            &mut output,
        );
    }
    output
}

fn part_1(lines: Lines) -> String {
    let (mut registers, program) = parse_input(lines);
    let output = run_program(program, &mut registers);

    output
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn part_2(lines: Lines) -> String {
    "".to_string()
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_17");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_run_program() {
        let mut registers = Registers { a: 0, b: 0, c: 9 };
        let program = vec![2, 6];
        run_program(program, &mut registers);
        assert_eq!(registers.b, 1);

        let mut registers = Registers { a: 10, b: 0, c: 0 };
        let program = vec![5, 0, 5, 1, 5, 4];
        assert_eq!(run_program(program, &mut registers), vec![0, 1, 2]);

        let mut registers = Registers {
            a: 2024,
            b: 0,
            c: 0,
        };
        let program = vec![0, 1, 5, 4, 3, 0];
        assert_eq!(
            run_program(program, &mut registers),
            vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]
        );
        assert_eq!(registers.a, 0);

        let mut registers = Registers { a: 0, b: 29, c: 0 };
        let program = vec![1, 7];
        run_program(program, &mut registers);
        assert_eq!(registers.b, 26);

        let mut registers = Registers {
            a: 0,
            b: 2024,
            c: 43690,
        };
        let program = vec![4, 0];
        run_program(program, &mut registers);
        assert_eq!(registers.b, 44354);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            part_1(EXAMPLE_INPUT.lines()),
            "4,6,3,5,6,3,5,2,1,0".to_string()
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(load_input("inputs/2024/day_17").lines()),
            "".to_string()
        );
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), "".to_string());
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(load_input("inputs/2024/day_17").lines()),
            "".to_string()
        )
    }
}
