use crate::util::util::load_input;
use crate::{Solution, SolutionPair};

// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
fn part_1(input: &str) -> u32 {
    let elves: std::str::Split<&str> = input.split("\n\n");
    elves
        .map(|elf: &str| {
            elf.split_ascii_whitespace().fold(0, |acc, calories_str| {
                acc + calories_str.parse::<u32>().unwrap()
            })
        })
        .max()
        .unwrap()
}

// How many Calories are the top 3 Elves carrying in total?
fn part_2(input: &str) -> u32 {
    let elves: std::str::Split<&str> = input.split("\n\n");
    let mut calories_per_elf: Vec<u32> = elves
        .map(|elf: &str| {
            elf.split_ascii_whitespace().fold(0, |acc, calories_str| {
                acc + calories_str.parse::<u32>().unwrap()
            })
        })
        .collect::<Vec<u32>>();
    calories_per_elf.sort_unstable();

    calories_per_elf
        .get(calories_per_elf.len() - 3..)
        .unwrap()
        .iter()
        .sum::<u32>()
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2022/day_1");
    (
        Solution::from(part_1(&input)),
        Solution::from(part_2(&input)),
    )
}
