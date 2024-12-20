use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;

enum Direction {
    Backward,
    Forward,
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_ascii_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn extrapolate(nums: Vec<i32>, direction: &Direction) -> i32 {
    if nums.iter().sum::<i32>() == 0 {
        return 0;
    }

    let diff: Vec<i32> = nums.windows(2).map(|w| w[1] - w[0]).collect();
    let extrapolated = extrapolate(diff, direction);

    match direction {
        Direction::Backward => nums.first().unwrap() - extrapolated,
        Direction::Forward => extrapolated + nums.last().unwrap(),
    }
}

fn part_1(lines: Lines) -> i32 {
    lines
        .map(|line| extrapolate(parse_line(line), &Direction::Forward))
        .sum()
}

fn part_2(lines: Lines) -> i32 {
    lines
        .map(|line| extrapolate(parse_line(line), &Direction::Backward))
        .sum()
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2023/day_9");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
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
        assert_eq!(
            extrapolate(vec![0, 3, 6, 9, 12, 15], &Direction::Forward),
            18
        );
        assert_eq!(
            extrapolate(vec![1, 3, 6, 10, 15, 21], &Direction::Forward),
            28
        );
        assert_eq!(
            extrapolate(vec![10, 13, 16, 21, 30, 45], &Direction::Forward),
            68
        );
    }

    #[test]
    fn test_extrapolate_backwards() {
        assert_eq!(
            extrapolate(vec![0, 3, 6, 9, 12, 15], &Direction::Backward),
            -3
        );
        assert_eq!(
            extrapolate(vec![1, 3, 6, 10, 15, 21], &Direction::Backward),
            0
        );
        assert_eq!(
            extrapolate(vec![10, 13, 16, 21, 30, 45], &Direction::Backward),
            5
        );
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 114);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2023/day_9").lines()), 1972648895);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2023/day_9").lines()), 919);
    }
}
