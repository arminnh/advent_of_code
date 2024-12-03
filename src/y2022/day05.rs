use std::str::Lines;

use crate::util::util::load_input;
use crate::{Solution, SolutionPair};

// Print all stacks from left to right.
fn print_stacks(stacks: &Vec<Vec<char>>) {
    println!("Stacks:");
    for i in 0..stacks.len() {
        println!("\t{}:\t{:?}", i + 1, stacks[i]);
    }
    println!();
}

// Handle a "setup" line which is supposed to set up the stacks. 0 can be ignored - represents no cargo.
// Example:
//     [N] [0] [Q] [0] [0] [N] [0] [0] [0]
// The n'th element between square brackets should be inserted at the bottom of the n'th stack.
fn handle_setup_line(stacks: &mut Vec<Vec<char>>, line: &str) {
    line.split(" ").enumerate().for_each(|(i, cargo)| {
        let c: char = cargo.chars().nth(1).unwrap();
        if c != '0' {
            if let Some(stack) = stacks.get_mut(i) {
                // get_mut to get a mutable reference !!!
                stack.insert(0, c);
            } else {
                panic!("Stack doesn't exist");
            }
        }
    });
}

// Handle a "move" line which defines a number of moves from one stack to another one.
fn handle_move_line_part_1(stacks: &mut Vec<Vec<char>>, line: &str) {
    println!("handle_move_line: {}", line);
    let split: Vec<&str> = line.split(' ').collect();
    let i: usize = split[1].parse::<usize>().unwrap();
    let from: usize = split[3].parse::<usize>().unwrap() - 1;
    let to: usize = split[5].parse::<usize>().unwrap() - 1;

    (0..i).for_each(|_| match stacks.get_mut(from).unwrap().pop() {
        Some(cargo) => stacks.get_mut(to).unwrap().push(cargo),
        None => println!("Out of cargo!!"),
    });

    print_stacks(stacks);
}

// Handle a "move" line which defines a number of moves from one stack to another one.
// In this version, moving multiple items should move them as a single block instead of LIFO.
fn handle_move_line_part_2(stacks: &mut Vec<Vec<char>>, line: &str) {
    println!("handle_move_line: {}", line);
    let split: Vec<&str> = line.split(' ').collect();
    let i: usize = split[1].parse::<usize>().unwrap();
    let from: usize = split[3].parse::<usize>().unwrap() - 1;
    let to: usize = split[5].parse::<usize>().unwrap() - 1;

    let from_stack: &mut Vec<char> = stacks.get_mut(from).unwrap();
    let mut cargo: Vec<char> = from_stack.split_off(from_stack.len() - i);
    stacks.get_mut(to).unwrap().append(&mut cargo);

    print_stacks(stacks);
}

// After the rearrangement procedure completes, what crate ends up on top of each stack?
fn process_moves(lines: Lines, part_1: bool) -> String {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    (0..9).for_each(|_| stacks.push(Vec::new()));
    print_stacks(&stacks);

    lines.for_each(|line| {
        if line.len() > 0 {
            match line.chars().nth(0).unwrap() {
                '[' => handle_setup_line(&mut stacks, line),
                'm' if part_1 => handle_move_line_part_1(&mut stacks, line),
                'm' => handle_move_line_part_2(&mut stacks, line),
                _ => print_stacks(&stacks),
            }
        }
    });

    stacks.iter().map(|s| s.last().unwrap()).collect()
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2022/day_5");
    (
        Solution::from(process_moves(input.lines(), true)),
        Solution::from(process_moves(input.lines(), false)),
    )
}
