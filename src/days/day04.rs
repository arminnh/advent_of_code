use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::fs;
use std::str::Lines;
use std::usize;

fn load_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not open file.")
}

fn parse_numbers(input: &str) -> HashSet<usize> {
    input
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn part_1(lines: Lines) -> usize {
    lines
        .map(
            |line| match line.split([':', '|']).collect::<Vec<&str>>()[..] {
                [_, winners_str, scratched_str] => {
                    let winners = parse_numbers(winners_str);
                    let scratched = parse_numbers(scratched_str);
                    let wins = scratched.iter().filter(|x| winners.contains(x)).count();

                    if wins > 0 {
                        2_usize.pow((wins - 1) as u32)
                    } else {
                        0
                    }
                }
                _ => panic!("Invalid input: '{:?}'", line),
            },
        )
        .sum()
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_1(load_input("inputs/day_4").lines())),
        Solution::from(part_2(load_input("inputs/day_4").lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 13);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_4").lines()), 26914);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_4").lines()), 0);
    }
}
