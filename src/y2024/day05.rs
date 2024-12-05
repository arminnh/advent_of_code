use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::str::Lines;
use std::usize;

fn part_1(input: String) -> usize {
    let mut before: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut after: HashMap<usize, Vec<usize>> = HashMap::new();

    let parts: Vec<&str> = input.split("\n\n").collect();

    for rule in parts[0].lines() {
        let rule_parts: Vec<usize> = rule
            .split("|")
            .map(|s| s.parse::<usize>().expect("could not parse number"))
            .collect();
        let first = rule_parts[0];
        let second = rule_parts[1];
        before
            .entry(first)
            .and_modify(|v| v.push(second))
            .or_insert(Vec::from([second]));
        after
            .entry(second)
            .and_modify(|v| v.push(first))
            .or_insert(Vec::from([first]));
    }

    let updates: Vec<Vec<usize>> = parts[1]
        .lines()
        .map(|line| {
            line.split(",")
                .map(|c| c.parse::<usize>().expect("could not parse number"))
                .collect()
        })
        .collect();

    updates
        .iter()
        .filter(|update| {
            for (i, u) in update.iter().enumerate() {
                for j in i..update.len() {
                    if after
                        .get(u)
                        .map(|rules| rules.contains(&update[j]))
                        .unwrap_or(false)
                    {
                        return false;
                    }
                }
            }
            true
        })
        .map(|update| update[update.len() / 2])
        .sum()
}

fn part_2(input: String) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_5");
    (
        Solution::from(part_1(input.clone())),
        Solution::from(part_2(input)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.to_string()), 143);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_5")), 4281);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT_4.lines()), 1);
    //     assert_eq!(part_2(EXAMPLE_INPUT_2.lines()), 9);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(load_input("inputs/2024/day_4").lines()), 1873);
    // }
}
