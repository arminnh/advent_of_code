use std::collections::{HashMap, HashSet};
use std::usize;

type Rules = HashMap<usize, HashSet<usize>>;

/// Map each number in the input to set of numbers it should be before
fn parse_rules(input: &str) -> Rules {
    let mut rules: Rules = HashMap::new();
    for rule in input.lines() {
        let rule_parts: Vec<usize> = rule
            .split("|")
            .map(|s| s.parse::<usize>().expect("could not parse number in rule"))
            .collect();

        rules
            .entry(rule_parts[0])
            .or_default()
            .insert(rule_parts[1]);
    }
    rules
}

/// Page updates are given as lines of comma separated lists
fn parse_updates(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|c| {
                    c.parse::<usize>()
                        .expect("could not parse number in update")
                })
                .collect()
        })
        .collect()
}

/// An update is correctly ordered if all page numbers are in the correct order as per the rules
fn is_update_correctly_ordered(update: &Vec<usize>, rules: &Rules) -> bool {
    for (i, page_before) in update.iter().enumerate() {
        for page_after in &update[i..update.len()] {
            if rules
                .get(page_after)
                .map(|rules| rules.contains(page_before))
                .unwrap_or(false)
            {
                return false;
            }
        }
    }
    true
}

// What do you get if you add up the middle page number from the correctly-ordered updates?
pub fn part_1(input: &str) -> usize {
    let parts: Vec<&str> = input.split("\n\n").collect();
    // Map of each number to set of numbers it should be before
    let rules: Rules = parse_rules(parts[0]);
    let updates = parse_updates(parts[1]);

    updates
        .iter()
        .filter(|update| is_update_correctly_ordered(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum()
}

/// Updates should be ordered in accordance with the rules
fn update_ordering(a: &usize, b: &usize, rules: &Rules) -> std::cmp::Ordering {
    if rules.get(b).map(|rules| rules.contains(a)).unwrap_or(false) {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}

// For each of the incorrectly-ordered updates, use the page ordering rules to put the page numbers in the right order.
// What do you get if you add up the middle page numbers after correctly ordering just those updates?
pub fn part_2(input: &str) -> usize {
    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules: Rules = parse_rules(parts[0]);
    let updates = parse_updates(parts[1]);

    updates
        .into_iter()
        .filter(|update| !is_update_correctly_ordered(update, &rules))
        .map(|mut update| {
            update.sort_by(|a, b| update_ordering(a, b, &rules));
            update[update.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::util::load_input;

    const EXAMPLE_INPUT: &str = "47|53
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
        assert_eq!(part_1(EXAMPLE_INPUT), 143);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2024/day_5")), 4281);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT), 123);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2024/day_5")), 5466);
    }
}
