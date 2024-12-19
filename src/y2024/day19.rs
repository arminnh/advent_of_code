use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::str::Lines;
use std::usize;

// Is is possible to create the desired design using the available patterns?
// Recursive approach: split string on each avaiable pattern. If one of the splits is possible, then design is possible
fn is_possible(design: String, patterns: &Vec<&str>, cache: &mut HashMap<String, bool>) -> bool {
    // base case, no need to split further
    if let Some(result) = cache.get(&design) {
        return *result;
    }

    for pattern in patterns {
        let split_possible = match design.split_once(pattern) {
            Some((left, "")) => is_possible(left.to_string(), patterns, cache),
            Some(("", right)) => is_possible(right.to_string(), patterns, cache),
            Some((left, right)) => {
                is_possible(left.to_string(), patterns, cache)
                    && is_possible(right.to_string(), patterns, cache)
            }
            None => false,
        };

        if split_possible {
            cache.insert(design, true);
            return true;
        }
    }

    cache.insert(design, false);
    false
}

// How many designs are possible?
fn part_1(input: String) -> usize {
    let (patterns, designs) = input
        .split_once("\n\n")
        .expect("Could not split input in 2 parts");
    let mut patterns: Vec<&str> = patterns.split(", ").collect();
    patterns.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut cache: HashMap<String, bool> = patterns.iter().map(|p| (p.to_string(), true)).collect();

    designs
        .lines()
        .filter(|design| is_possible(design.to_string(), &patterns, &mut cache))
        .count()
}

fn part_2(lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_19");
    (
        Solution::from(part_1(input.clone())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.to_string()), 6);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_19")), 226);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_19").lines()), 0)
    }
}
