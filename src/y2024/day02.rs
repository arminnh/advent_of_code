use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;
use std::usize;

fn parse_report(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Could not parse number"))
        .collect()
}

// Report is safe if all increasing or decreasing by at least 1 and most 3
fn is_safe(report: &Vec<i32>) -> bool {
    let (min, max) = report
        .windows(2)
        .fold((i32::MAX, i32::MIN), |(min, max), window| {
            let diff = window[1] - window[0];
            (min.min(diff), max.max(diff))
        });

    (min >= 1 && max <= 3) || (min >= -3 && max <= -1)
}

// How many reports are safe?
fn part_1(lines: Lines) -> usize {
    lines
        .map(|line| parse_report(line))
        .filter_map(|report| if is_safe(&report) { Some(1) } else { None })
        .sum()
}

// Can tolerate one bad entry
fn is_safe_with_tolerance(report: &Vec<i32>) -> bool {
    // bruteforce version
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut r = report.clone();
        r.remove(i);
        if is_safe(&r) {
            return true;
        }
    }
    false

    // let (positives, negatives, extremes) =
    //     report
    //         .windows(2)
    //         .map(|w| w[1] - w[0])
    //         .fold((0, 0, 0), |(pos, neg, ex), diff| {
    //             if diff == 0 || diff.abs() > 3 {
    //                 (pos, neg, ex + 1)
    //             } else if diff > 0 {
    //                 (pos + 1, neg, ex)
    //             } else {
    //                 (pos, neg + 1, ex)
    //             }
    //         });

    // println!("{:?}", (positives, negatives, extremes));
    // // If there are positives, there can only be 1 negative or extreme value
    // if positives > 0 {
    //     (negatives == 0 && extremes <= 2) || (negatives <= 1 && extremes == 0)
    // } else {
    //     (positives == 0 && extremes <= 2) || (positives <= 1 && extremes == 0)
    // }
}

fn part_2(lines: Lines) -> i32 {
    lines
        .map(|line| parse_report(line))
        .filter_map(|report| {
            if is_safe_with_tolerance(&report) {
                Some(1)
            } else {
                None
            }
        })
        .sum()
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

    const EXAMPLE_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 2);
    }

    #[test]
    fn test_is_safe() {
        assert_eq!(is_safe(&Vec::from([1, 1, 1, 1])), false);
        assert_eq!(is_safe(&Vec::from([1, 2, 3, 4, 5])), true);
        assert_eq!(is_safe(&Vec::from([5, 4, 3, 2, 1])), true);
        assert_eq!(is_safe(&Vec::from([1, 4, 7, 10, 13])), true);
        assert_eq!(is_safe(&Vec::from([13, 10, 7, 4, 1])), true);
        assert_eq!(is_safe(&Vec::from([5, 4, 3, 2, 1, 2, 3, 4, 5])), false);
        assert_eq!(is_safe(&Vec::from([10, 13, 7, 5, 3, 1, 4])), false);
        assert_eq!(is_safe(&Vec::from([60, 56, 54, 51, 49, 49, 48, 46])), false);
        assert_eq!(is_safe(&Vec::from([85, 80, 79, 78, 74, 73, 71, 69])), false);
        assert_eq!(is_safe(&Vec::from([41, 39, 37, 35, 33, 31, 32, 32])), false);
        assert_eq!(is_safe(&Vec::from([29, 32, 29, 30, 35])), false);
        assert_eq!(is_safe(&Vec::from([18, 21, 23, 24, 27, 29, 31, 32])), true);
        assert_eq!(is_safe(&Vec::from([2, 4, 5, 8, 11, 13])), true);
        assert_eq!(is_safe(&Vec::from([43, 47, 48, 50, 51, 52])), false);
        assert_eq!(is_safe(&Vec::from([16, 22, 23, 26, 29, 32, 35, 37])), false);
        assert_eq!(is_safe(&Vec::from([91, 92, 92, 89, 86])), false);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_2").lines()), 202);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), 4);
    }

    #[test]
    fn test_is_safe_with_tolerance() {
        assert_eq!(is_safe_with_tolerance(&Vec::from([1, 1, 2])), true);
        assert_eq!(is_safe_with_tolerance(&Vec::from([2, 10, 1])), true);
        assert_eq!(is_safe_with_tolerance(&Vec::from([1, 1, 1, 1])), false);
        assert_eq!(is_safe_with_tolerance(&Vec::from([1, 2, 3, 4, 5])), true);
        assert_eq!(
            is_safe_with_tolerance(&Vec::from([43, 47, 48, 50, 51, 52])),
            true
        );
        assert_eq!(
            is_safe_with_tolerance(&Vec::from([16, 22, 23, 26, 29, 32, 35, 37])),
            true
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_2").lines()), 271);
    }
}
