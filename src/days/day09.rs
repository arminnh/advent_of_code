use crate::{Solution, SolutionPair};
use std::fs;
use std::str::Lines;

fn load_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not open file.")
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn extrapolate(nums: Vec<i32>) -> i32 {
    if nums.iter().sum::<i32>() == 0 {
        return 0;
    }

    let last = nums.last().unwrap();
    let diff: Vec<i32> = nums.windows(2).map(|w| w[1] - w[0]).collect();
    // println!("diff: {:?}, last: {:?}", diff, last);

    last + extrapolate(diff)
}

fn part_1(lines: Lines) -> i32 {
    lines.map(|line| extrapolate(parse_line(line))).sum()
}

fn part_2(_lines: Lines) -> i32 {
    0
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_1(load_input("inputs/day_9").lines())),
        Solution::from(part_2(load_input("inputs/day_9").lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("0 3 6 9 12 15"), vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(parse_line("1 3 6 10 15 21"), vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(
            parse_line("10 13 16 21 30 45"),
            vec![10, 13, 16, 21, 30, 45]
        );
    }

    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate(vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrapolate(vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(extrapolate(vec![10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 114);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_9").lines()), 1972648895);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_9").lines()), 0);
    }
}
