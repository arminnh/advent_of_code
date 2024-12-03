use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;
use std::usize;

use regex::Regex;

// How many reports are safe?
fn part_1(lines: Lines) -> usize {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    lines
        .map(|line| {
            re.captures_iter(line)
                .map(|c| c.extract())
                .map(|(_, [left, right])| {
                    left.parse::<usize>().expect("could not parse left number")
                        * right
                            .parse::<usize>()
                            .expect("could not parse right number")
                })
                .sum::<usize>()
        })
        .sum()
}

fn part_2(lines: Lines) -> i32 {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_2");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 161);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_3").lines()), 173517243);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT.lines()), 4);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(load_input("inputs/2024/day_3").lines()), 271);
    // }
}
