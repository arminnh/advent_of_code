use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;
use std::usize;

// instructions take an given operand and a 'combo' operand which is determined by a function
enum Instruction {
    ADV, // divide A by 2^combo -> store in A
    BXL, // bitwise XOR B and operand -> store in B
    BST, // combo operand % 8 -> store in B
    JNZ, // if A != 0, jump to operand (-2 to account for the increment after).
    BXC, // bitwise XOR B and C -> store in B
    OUT, // output combo % 8
    BDV, // adv, but store in B
    CDV, // adv, but store in C
}

impl Instruction {
    fn from(code: u32) -> Instruction {
        match code {
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
}

fn parse_input(mut lines: Lines) -> (u32, u32, u32, Vec<u32>) {
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

    (a, b, c, program)
}

fn run_program(program: Vec<u32>, a: u32, b: u32, c: u32) -> Vec<usize> {
    let mut instruction_pointer = 0;

    loop {
        let opcode = todo!();
        let operand = todo!();
        let combo_operand = todo!();


        instruction_pointer += 2;
    }
    todo!()
}

fn part_1(lines: Lines) -> String {
    let (a, b, c, program) = parse_input(lines);
    let output = run_program(program, a, b, c);

    let o =  output
        .iter()
        .map(|num| num.to_string()).collect::<Vec<String>>().join(",");

    "".to_string()
}

fn part_2(lines: Lines) -> usize {
    0
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
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 0);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_17").lines()), 0);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_17").lines()), 0)
    }
}
