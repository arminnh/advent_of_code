use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;
use std::usize;

type Stones = Vec<u64>;

fn parse_stones(line: &str) -> Stones {
    line.split_whitespace()
        .map(|s| s.parse().expect("Could not parse stone number"))
        .collect()
}

fn blink(stones: Stones) -> Stones {
    let mut new_stones = Vec::with_capacity(stones.len() * 2);

    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else {
            let nr_of_digits = stone.ilog10() + 1;
            if nr_of_digits % 2 == 0 {
                let divisor = 10_u64.pow(nr_of_digits / 2);
                let (left, right) = (stone / divisor, stone % divisor);
                new_stones.push(left);
                new_stones.push(right);
            } else {
                new_stones.push(stone * 2024)
            }
        }
    }

    new_stones
}

// Consider the arrangement of stones in front of you. How many stones will you have after blinking 25 times?
fn part_1(lines: Lines) -> usize {
    lines
        .map(parse_stones)
        .map(|mut stones| {
            for _ in 0..25 {
                stones = blink(stones);
            }
            stones.len()
        })
        .sum()
}

fn part_2(lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_11");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "125 17";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 55312);
    }

    #[test]
    fn test_blink() {
        assert_eq!(blink(parse_stones("125 17")), vec![253000, 1, 7]);
        assert_eq!(blink(parse_stones("253000 1 7")), vec![253, 0, 2024, 14168]);
        assert_eq!(
            blink(parse_stones("253 0 2024 14168")),
            vec![512072, 1, 20, 24, 28676032]
        );
        assert_eq!(
            blink(parse_stones("512072 1 20 24 28676032")),
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032]
        );
        assert_eq!(
            blink(parse_stones("512 72 2024 2 0 2 4 2867 6032")),
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
        );
        assert_eq!(
            blink(parse_stones(
                "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32"
            )),
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2
            ]
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_11").lines()), 190865);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_11").lines()), 0)
    }
}
