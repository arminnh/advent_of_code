use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};
use std::str::Lines;
use std::usize;

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(c: char) -> Self {
        match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            _ => Spring::Unknown,
        }
    }
}

impl Debug for Spring {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Spring::Operational => '.',
                Spring::Damaged => '#',
                Spring::Unknown => '?',
            }
        )
    }
}

fn parse_line(line: &str) -> (Vec<Spring>, Vec<usize>) {
    match line.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
        [springs, damaged_spring_groups] => (
            springs.chars().map(|c| Spring::from(c)).collect(),
            damaged_spring_groups
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
        ),
        _ => panic!("Invalid input line {:?}", line),
    }
}

// Count how many possible arrangements of operational and damaged springs fit the given criteria in each row.
// Contiguous groups of damaged springs are always separated by at least one operational spring.
fn possible_arrangements(
    springs: Vec<Spring>,
    damaged_spring_groups: Vec<usize>,
    cache: &mut HashMap<(Vec<Spring>, Vec<usize>), usize>,
) -> usize {
    // println!("{springs:?}, {damaged_spring_groups:?}");
    if let Some(result) = cache.get(&(springs.clone(), damaged_spring_groups.clone())) {
        // println!("    Hit {result:?}");
        return *result;
    }

    if let Some(&group_size) = damaged_spring_groups.first() {
        // If no space left for remaining groups (+ buffers for operational springs) -> no match
        if springs.len()
            < damaged_spring_groups.iter().sum::<usize>() + damaged_spring_groups.len() - 1
        {
            // println!("    no room left");
            cache.insert((springs, damaged_spring_groups), 0);
            return 0;
        }

        // If first spring is operational, nothing to do here -> skip all operational springs
        if springs[0] == Spring::Operational {
            // println!("    skip Operational");
            let result =
                possible_arrangements(springs[1..].to_vec(), damaged_spring_groups.clone(), cache);
            cache.insert((springs, damaged_spring_groups), result);
            return result;
        }

        // Starting from the first spring (+ 1 for the operational spring after the group),
        // if a damaged group fits, then "place" it and check the result for the rest of the springs
        let mut result = 0;
        if !springs[..group_size].contains(&Spring::Operational)
            && (springs.len() == group_size || springs[group_size] != Spring::Damaged)
        {
            // println!("    place group");
            result = possible_arrangements(
                springs[std::cmp::min(group_size + 1, springs.len())..].to_vec(),
                damaged_spring_groups[1..].to_vec(),
                cache,
            );
        }

        // If first spring is unknown, add the number of results if we were to skip it
        // instead of placing a potential damaged group
        if springs[0] == Spring::Unknown {
            // println!("    skip group");
            result +=
                possible_arrangements(springs[1..].to_vec(), damaged_spring_groups.clone(), cache);
        }

        cache.insert((springs, damaged_spring_groups), result);
        result
    } else {
        // No groups left, but still have a damaged spring left -> no match
        let result = match springs.contains(&Spring::Damaged) {
            true => 0,
            false => 1,
        };
        // println!("    no groups left -> {result}");
        cache.insert((springs, damaged_spring_groups), result);
        result
    }
}

pub fn part_1(input: &str) -> usize {
    let mut cache = HashMap::new();
    input
        .lines()
        .map(|line| {
            let (springs, damaged_spring_groups) = parse_line(line);
            possible_arrangements(springs, damaged_spring_groups, &mut cache)
        })
        .sum()
}

fn unfold_input(
    springs: Vec<Spring>,
    damaged_spring_groups: Vec<usize>,
) -> (Vec<Spring>, Vec<usize>) {
    let mut new_springs = springs.clone();
    for _ in 0..4 {
        new_springs.push(Spring::Unknown);
        new_springs.append(&mut springs.clone());
    }
    (new_springs, damaged_spring_groups.repeat(5))
}

pub fn part_2(input: &str) -> usize {
    let mut cache = HashMap::new();
    input
        .lines()
        .map(|line| {
            let (springs, damaged_spring_groups) = parse_line(line);
            let (springs, damaged_spring_groups) = unfold_input(springs, damaged_spring_groups);
            possible_arrangements(springs, damaged_spring_groups, &mut cache)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::util::load_input;

    const EXAMPLE_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_count_possible_arrangements_1() {
        let (springs, groups) = parse_line("???.### 1,1,3");
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 1);
    }

    #[test]
    fn test_count_possible_arrangements_2() {
        let (springs, groups) = parse_line(".??..??...?##. 1,1,3");
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 4);
    }

    #[test]
    fn test_count_possible_arrangements_3() {
        let (springs, groups) = parse_line("?#?#?#?#?#?#?#? 1,3,1,6");
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 1);
    }

    #[test]
    fn test_count_possible_arrangements_4() {
        let (springs, groups) = parse_line("????.#...#... 4,1,1");
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 1);
    }

    #[test]
    fn test_count_possible_arrangements_5() {
        let (springs, groups) = parse_line("????.######..#####. 1,6,5");
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 4);
    }

    #[test]
    fn test_count_possible_arrangements_6() {
        let (springs, groups) = parse_line("?###???????? 3,2,1");
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 10);
    }

    #[test]
    fn test_count_possible_arrangements_7() {
        let (springs, groups) = parse_line("#????##????#?#??#?? 1,1,13");
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 3);
    }

    #[test]
    fn test_count_possible_arrangements_8() {
        let (springs, groups) = parse_line("?.#?#??#?#. 1,6,1");
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 1);
    }

    #[test]
    fn test_count_possible_arrangements_9() {
        let (springs, groups) = parse_line(".#?.???????????#..? 2,5,1,1,1,1");
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 4);
    }

    #[test]
    fn test_count_possible_arrangements_10() {
        let (springs, groups) = parse_line("??..???.?#????????? 1,3,2,1,1,1");
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 60);
    }

    #[test]
    fn test_count_possible_arrangements_11() {
        let (springs, groups) = parse_line("??????#?#? 1,1,3");
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 6);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT), 21);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2023/day_12")), 7379);
    }

    #[test]
    fn test_part_2_example_line_1() {
        let (springs, groups) = parse_line("???.### 1,1,3");
        let (springs, groups) = unfold_input(springs, groups);
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 1);
    }

    #[test]
    fn test_part_2_example_line_2() {
        let (springs, groups) = parse_line(".??..??...?##. 1,1,3");
        let (springs, groups) = unfold_input(springs, groups);
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 16384);
    }

    #[test]
    fn test_part_2_example_line_3() {
        let (springs, groups) = parse_line("?#?#?#?#?#?#?#? 1,3,1,6");
        let (springs, groups) = unfold_input(springs, groups);
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 1);
    }

    #[test]
    fn test_part_2_example_line_4() {
        let (springs, groups) = parse_line("????.#...#... 4,1,1");
        let (springs, groups) = unfold_input(springs, groups);
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 16);
    }

    #[test]
    fn test_part_2_example_line_5() {
        let (springs, groups) = parse_line("????.######..#####. 1,6,5");
        let (springs, groups) = unfold_input(springs, groups);
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 2500);
    }

    #[test]
    fn test_part_2_example_line_6() {
        let (springs, groups) = parse_line("?###???????? 3,2,1");
        let (springs, groups) = unfold_input(springs, groups);
        let mut cache = HashMap::new();
        assert_eq!(possible_arrangements(springs, groups, &mut cache), 506250);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT), 525152);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2023/day_12")), 7732028747925);
    }
}
