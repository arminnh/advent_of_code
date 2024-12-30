use std::usize;

type Program = Vec<u32>;

#[derive(Debug, Clone, Copy)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

// instructions take an given operand and a 'combo' operand which is determined by a function
#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

fn parse_input(input: &str) -> (Registers, Program) {
    let mut lines = input.lines();
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
            // divide A by 2^combo -> store in A --- same as right shift by combo
            Instruction::ADV => registers.a >>= combo_operand(operand, &registers),
            // bitwise XOR B and operand -> store in B
            Instruction::BXL => registers.b ^= operand as u64,
            // combo operand % 8 -> store in B
            Instruction::BST => registers.b = (combo_operand(operand, &registers) % 8) as u64,
            // if A != 0, jump to operand
            Instruction::JNZ => {
                if registers.a != 0 {
                    *instruction_pointer = operand as usize;
                } else {
                    *instruction_pointer += 2;
                }
            }
            // bitwise XOR B and C -> store in B
            Instruction::BXC => registers.b ^= registers.c,
            // output combo % 8
            Instruction::OUT => output.push(combo_operand(operand, &registers) as u32 % 8),
            // adv, but store in B
            Instruction::BDV => registers.b = registers.a >> combo_operand(operand, &registers),
            // adv, but store in C
            Instruction::CDV => registers.c = registers.a >> combo_operand(operand, &registers),
        }

        if *self != Instruction::JNZ {
            *instruction_pointer += 2;
        }
    }
}

fn combo_operand(operand: u32, registers: &Registers) -> u64 {
    match operand {
        0..=3 => operand as u64,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        _ => panic!("Invalid operand > 6 for combo operand."),
    }
}

fn run_program(program: &Program, registers: &mut Registers) -> Vec<u32> {
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

// Run the program. What do you get if you use commas to join the values it outputs into a single string?
pub fn part_1(input: &str) -> String {
    let (mut registers, program) = parse_input(input);
    let output = run_program(&program, &mut registers);

    output
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

// What is the lowest positive initial value for register A that causes the program to output a copy of itself?
pub fn part_2(input: &str) -> u64 {
    let (_, program) = parse_input(input);
    // Example input comes down to following loop:
    //     while a != 0:
    //         a = int(a / 8)
    //         print(a % 8)
    // To make it print a desired outcome, like [0,3,5,4,3,0], can start from the end
    // and build `a` in reverse. Start from the last digit, then add the one
    // before it (for the mod operation) and multiply by 8 (for the division)
    //
    // This works fine for the example, but not on the actual input:
    // program
    //     .iter()
    //     .rev()
    //     .skip(1)
    //     .fold(*program.last().unwrap(), |acc, num| (acc + num) * 8)
    //
    // Actual input is more complex:
    //     BST 4
    //     BXL 1
    //     CDV 5
    //     BXL 5
    //     BXC 0
    //     OUTPUT 5
    //     ADV 3
    // In Python:
    //     while a != 0:
    //         b = a % 8
    //         b = b ^ 1
    //         c = a / 2^b = a >> b
    //         b = b ^ 5
    //         b = b ^ c
    //         print(b % 8)
    //         a = a / 8 = a >> 3
    // Assumptions:
    //     * Last instruction always jumps to 0 -> simple while != 0
    //     * A always gets shifted right by 3 per loop
    //
    // Will still start search from the end and build up the result 3 bits at a time
    // Each search loop only needs to check 8 possible values
    // When a match is found (the output of the program matches the digit we are searching for),
    // shift the result left by 3 and try searching for the next digit.
    // Multiple matches are possible for a target output.
    // If the first match doesn't work out, backtrack and try with a later match.

    fn find(result: u64, program: &Program) -> Option<u64> {
        for attempt in 0..8 {
            let a = result + attempt;
            let mut new_registers = Registers { a, b: 0, c: 0 };
            let output = run_program(&program, &mut new_registers);
            let expected_output = &program[program.len() - output.len()..];
            if output == expected_output {
                if output.len() == program.len() {
                    return Some(a);
                } else {
                    // Try rest of search with this match
                    if let Some(result) = find(a << 3, program) {
                        return Some(result);
                    }
                    // Else hopefully a later match does work out <- backtracking
                }
            }
        }
        None
    }

    find(0, &program).expect("No result!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::util::load_input;

    const EXAMPLE_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const EXAMPLE_INPUT_PART_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_run_program() {
        let mut registers = Registers { a: 0, b: 0, c: 9 };
        let program = vec![2, 6];
        run_program(&program, &mut registers);
        assert_eq!(registers.b, 1);

        let mut registers = Registers { a: 10, b: 0, c: 0 };
        let program = vec![5, 0, 5, 1, 5, 4];
        assert_eq!(run_program(&program, &mut registers), vec![0, 1, 2]);

        let mut registers = Registers {
            a: 2024,
            b: 0,
            c: 0,
        };
        let program = vec![0, 1, 5, 4, 3, 0];
        assert_eq!(
            run_program(&program, &mut registers),
            vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]
        );
        assert_eq!(registers.a, 0);

        let mut registers = Registers { a: 0, b: 29, c: 0 };
        let program = vec![1, 7];
        run_program(&program, &mut registers);
        assert_eq!(registers.b, 26);

        let mut registers = Registers {
            a: 0,
            b: 2024,
            c: 43690,
        };
        let program = vec![4, 0];
        run_program(&program, &mut registers);
        assert_eq!(registers.b, 44354);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT), "4,6,3,5,6,3,5,2,1,0".to_string());
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(&load_input("inputs/2024/day_17")),
            "4,1,7,6,4,1,0,2,7".to_string()
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2024/day_17")), 164279024971453)
    }
}
