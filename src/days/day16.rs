use crate::{Solution, SolutionPair};
use std::fs;
use std::str::Lines;
use std::usize;

fn load_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not open file.")
}

fn part_1(_lines: Lines) -> usize {
    0
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_1(load_input("inputs/day_16").lines())),
        Solution::from(part_2(load_input("inputs/day_16").lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_16").lines()), 0);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_16").lines()), 0);
    }
}
