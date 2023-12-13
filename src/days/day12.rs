use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::fmt::{Debug, Formatter, Result};
use std::str::Lines;
use std::usize;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Status {
    Operational,
    Damaged,
    Unknown,
}

impl Status {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Status::Operational,
            '#' => Status::Damaged,
            '?' => Status::Unknown,
            _ => panic!("Invalid character {:?}", c),
        }
    }
}

impl Debug for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Status::Operational => '.',
                Status::Damaged => '#',
                Status::Unknown => '?',
            }
        )
    }
}

fn parse_line(line: &str) -> (Vec<Status>, Vec<usize>) {
    match line.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
        [springs, damaged_spring_groups] => (
            springs.chars().map(|c| Status::from_char(c)).collect(),
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
fn count_possible_arrangements(
    mut springs: Vec<Status>,
    damaged_spring_groups: Vec<usize>,
) -> usize {
    // Wrap the first input in operational to avoid edge case checking
    springs.insert(0, Status::Operational);
    springs.push(Status::Operational);
    // For each candidate, keep track of how far along it has been evaluated
    let mut candidates: Vec<(Vec<Status>, usize)> = Vec::from([(springs, 1)]);

    // Evolve the list of candidates for each group of damaged springs
    for (group_index, group_size) in damaged_spring_groups.iter().enumerate() {
        let mut new_candidates = Vec::new();

        while let Some((mut candidate, processed)) = candidates.pop() {
            for i in processed..candidate.len() - group_size {
                // Step through in windows of group size + 2, since groups must be surrounded by operational springs
                let window = candidate.get(i - 1..i + group_size + 1).unwrap();

                if window.starts_with(&[Status::Damaged]) {
                    // Just moved over a damaged group larger than current group size, so stop here for this group
                    break;
                }

                // If damaged group fits here, then place it and add as new candidate for next group checking
                if !window.ends_with(&[Status::Damaged])
                    && !window[1..1 + group_size].contains(&Status::Operational)
                {
                    let mut new_candidate = candidate.clone();
                    let mut new_group = vec![Status::Damaged].repeat(*group_size);
                    new_group.push(Status::Operational);
                    new_candidate.splice(i..i + group_size + 1, new_group);

                    // When checking last group, the rest of the candidate cannot contain damaged springs
                    if group_index < damaged_spring_groups.len() - 1
                        || !new_candidate
                            .get(i + group_size + 1..)
                            .unwrap()
                            .contains(&Status::Damaged)
                    {
                        new_candidates.push((new_candidate, i + group_size + 1));
                    }
                }

                // Moving on to evaluate later positions in the list, so
                // change unknown at current position to operational for those candidates
                if candidate[i] == Status::Unknown {
                    let _ = std::mem::replace(&mut candidate[i], Status::Operational);
                }
            }
        }

        candidates = new_candidates;
    }

    candidates.len()
}

fn part_1(lines: Lines) -> usize {
    lines
        .map(|line| {
            let (springs, damaged_spring_groups) = parse_line(line);
            count_possible_arrangements(springs, damaged_spring_groups)
        })
        .sum()
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_1(load_input("inputs/day_12").lines())),
        Solution::from(part_2(load_input("inputs/day_12").lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_count_possible_arrangements_1() {
        let (springs, groups) = parse_line("???.### 1,1,3");
        assert_eq!(count_possible_arrangements(springs, groups), 1);
    }

    #[test]
    fn test_count_possible_arrangements_2() {
        let (springs, groups) = parse_line(".??..??...?##. 1,1,3");
        assert_eq!(count_possible_arrangements(springs, groups), 4);
    }

    #[test]
    fn test_count_possible_arrangements_3() {
        let (springs, groups) = parse_line("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(count_possible_arrangements(springs, groups), 1);
    }

    #[test]
    fn test_count_possible_arrangements_4() {
        let (springs, groups) = parse_line("????.#...#... 4,1,1");
        assert_eq!(count_possible_arrangements(springs, groups), 1);
    }

    #[test]
    fn test_count_possible_arrangements_5() {
        let (springs, groups) = parse_line("????.######..#####. 1,6,5");
        assert_eq!(count_possible_arrangements(springs, groups), 4);
    }

    #[test]
    fn test_count_possible_arrangements_6() {
        let (springs, groups) = parse_line("?###???????? 3,2,1");
        assert_eq!(count_possible_arrangements(springs, groups), 10);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 21);
    }

    #[test]
    fn test_count_possible_arrangements_7() {
        let (springs, groups) = parse_line("#????##????#?#??#?? 1,1,13");
        assert_eq!(count_possible_arrangements(springs, groups), 3);
    }

    #[test]
    fn test_count_possible_arrangements_8() {
        let (springs, groups) = parse_line("?.#?#??#?#. 1,6,1");
        assert_eq!(count_possible_arrangements(springs, groups), 1);
    }

    #[test]
    fn test_count_possible_arrangements_9() {
        let (springs, groups) = parse_line(".#?.???????????#..? 2,5,1,1,1,1");
        assert_eq!(count_possible_arrangements(springs, groups), 4);
    }

    #[test]
    fn test_count_possible_arrangements_10() {
        let (springs, groups) = parse_line("??..???.?#????????? 1,3,2,1,1,1");
        assert_eq!(count_possible_arrangements(springs, groups), 60);
    }

    #[test]
    fn test_count_possible_arrangements_11() {
        let (springs, groups) = parse_line("??????#?#? 1,1,3");
        assert_eq!(count_possible_arrangements(springs, groups), 6);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_12").lines()), 7379);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT.lines()), 0);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(load_input("inputs/day_12").lines()), 0);
    // }
}
