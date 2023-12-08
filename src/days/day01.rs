use crate::{Solution, SolutionPair};
use std::fs;
use std::str::Lines;
use std::usize;

fn load_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not open file.")
}

fn part_1(lines: Lines) -> u32 {
    lines
        .map(|line| {
            let first = line
                .chars()
                .skip_while(|c| !c.is_ascii_digit())
                .next()
                .unwrap();
            let last = line
                .chars()
                .rev()
                .skip_while(|c| !c.is_ascii_digit())
                .next()
                .unwrap();

            first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
        })
        .sum()
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_1(load_input("inputs/day_1").lines())),
        Solution::from(part_2(load_input("inputs/day_1").lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 142);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_1").lines()), 55208);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_1").lines()), 0);
    }
}
