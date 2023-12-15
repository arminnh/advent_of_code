use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;
use std::usize;

fn hash(s: &str) -> usize {
    let mut current = 0;

    s.chars().for_each(|c| {
        current += c as usize;
        current *= 17;
        current %= 256;
    });

    current
}

fn part_1(lines: Lines) -> usize {
    lines
        .map(|line| line.split(',').map(|s| hash(s)).sum::<usize>())
        .sum()
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_11");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ot=7"), 231);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 1320);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_15").lines()), 513158);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(load_input("inputs/day_15").lines()), 0);
    // }
}
