use std::collections::HashMap;
use std::usize;

// Is is possible to create the desired design using the available patterns?
// Recursive approach: if the design can be broken down by the given patterns, then it is possible
fn is_possible(design: &str, patterns: &Vec<&str>, cache: &mut HashMap<String, bool>) -> bool {
    // base case, no need to split further
    if let Some(result) = cache.get(design) {
        return *result;
    }

    for pattern in patterns {
        if design.starts_with(pattern) {
            let next_design = &design[pattern.len()..];
            if is_possible(next_design, patterns, cache) {
                cache.insert(next_design.to_string(), true);
                return true;
            }
        }
    }

    cache.insert(design.to_string(), false);
    false
}

// How many designs are possible?
pub fn part_1(input: &str) -> usize {
    let (patterns, designs) = input
        .split_once("\n\n")
        .expect("Could not split input in 2 parts");
    let mut patterns: Vec<&str> = patterns.split(", ").collect();
    patterns.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut cache: HashMap<String, bool> = patterns.iter().map(|p| (p.to_string(), true)).collect();

    designs
        .lines()
        .filter(|design| is_possible(design, &patterns, &mut cache))
        .count()
}

// Instead of checking whether a design is possible, count in how many ways it can be created using the given patterns
fn nr_of_possible_arrangements(
    design: String,
    patterns: &Vec<&str>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    // base case, no need to split further
    if let Some(result) = cache.get(&design) {
        return *result;
    } else if design.len() == 1 {
        if patterns.contains(&&design[..]) {
            cache.insert(design, 1);
            return 1;
        } else {
            cache.insert(design, 0);
            return 0;
        }
    }

    let mut nr_of_arrangements = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            if &design == pattern {
                nr_of_arrangements += 1;
            } else {
                nr_of_arrangements += nr_of_possible_arrangements(
                    design[pattern.len()..].to_string(),
                    patterns,
                    cache,
                );
            }
        }
    }

    cache.insert(design, nr_of_arrangements);
    nr_of_arrangements
}

// What do you get if you add up the number of different ways you could make each design?
pub fn part_2(input: &str) -> usize {
    let (patterns, designs) = input
        .split_once("\n\n")
        .expect("Could not split input in 2 parts");
    let mut patterns: Vec<&str> = patterns.split(", ").collect();
    patterns.sort_by(|a, b| b.len().cmp(&a.len()));
    let mut cache: HashMap<String, usize> = HashMap::new();

    designs
        .lines()
        .map(|design| nr_of_possible_arrangements(design.to_string(), &patterns, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::util::load_input;

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
        assert_eq!(part_1(EXAMPLE_INPUT), 6);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2024/day_19")), 226);
    }

    #[test]
    fn test_part_2_example() {
        let input = "r, wr, b, g, bwu, rb, gb, br\n\n";
        assert_eq!(part_2(&(input.to_string() + "brwrr")), 2);
        assert_eq!(part_2(&(input.to_string() + "bggr")), 1);
        assert_eq!(part_2(&(input.to_string() + "gbbr")), 4);
        assert_eq!(part_2(&(input.to_string() + "rrbgbr")), 6);
        assert_eq!(part_2(&(input.to_string() + "bbrgwb")), 0);

        assert_eq!(part_2(EXAMPLE_INPUT), 16);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2024/day_19")), 601201576113503)
    }
}
