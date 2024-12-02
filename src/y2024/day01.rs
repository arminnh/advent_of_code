use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{BinaryHeap, HashMap};
use std::str::Lines;

// Distance between lists
fn part_1(lines: Lines) -> usize {
    let mut left: BinaryHeap<usize> = BinaryHeap::new();
    let mut right: BinaryHeap<usize> = BinaryHeap::new();

    lines.for_each(|line| {
        let mut nums = line
            .split("   ")
            .map(|s| s.parse::<usize>().expect("could not parse number"));
        left.push(nums.next().unwrap());
        right.push(nums.next().unwrap());
    });

    std::iter::from_fn(|| left.pop().zip(right.pop()))
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

// Similarity score
fn part_2(lines: Lines) -> usize {
    let mut left: HashMap<usize, usize> = HashMap::new();
    let mut right: HashMap<usize, usize> = HashMap::new();

    lines.for_each(|line| {
        let mut nums = line
            .split("   ")
            .map(|s| s.parse::<usize>().expect("could not parse number"));

        left.entry(nums.next().unwrap())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        right
            .entry(nums.next().unwrap())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    left.keys()
        .map(|key| key * left.get(key).unwrap() * right.get(key).unwrap_or(&0))
        .sum()
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_1");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 11);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_1").lines()), 1879048);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), 31);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_1").lines()), 21024792);
    }
}
