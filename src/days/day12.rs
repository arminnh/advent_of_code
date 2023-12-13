use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::fmt::{Debug, Formatter, Result};
use std::str::Lines;
use std::usize;

#[derive(PartialEq, Eq, Clone)]
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

// impl Display for Candidate {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         let result = self.0.iter().fold(Ok(()), |result, status| {
//             result.and_then(|_| write!(f, "{}", status))
//         });

//         result.and_then(|_| write!(f, " | "));

//         self.1.iter().fold(result, |result, status| {
//             result.and_then(|_| write!(f, "{}", status))
//         })
//     }
// }

// impl Display for Candidates {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         self.0.iter().fold(Ok(()), |result, candidate| {
//             result.and_then(|_| write!(f, "{}", candidate))
//         })
//     }
// }

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

// count how many possible arrangements of operational and damaged springs fit the given criteria in each row.
// contiguous groups of damaged springs are always separated by at least one operational spring
fn count_possible_arrangements(
    mut springs: Vec<Status>,
    damaged_spring_groups: Vec<usize>,
) -> usize {
    // wrap the first input in Operational to avoid edge case checking
    springs.insert(0, Status::Operational);
    springs.push(Status::Operational);
    // for each candidate, keep track of how far along is has been evaluated
    let mut candidates: Vec<(Vec<Status>, usize)> = Vec::from([(springs, 1)]);

    for group_size in damaged_spring_groups.iter() {
        let mut new_candidates = Vec::new();
        // println!("\nGroup {:?}\nCandidates:", group_size);
        // candidates.iter().for_each(|c| println!("  {:?}", c));

        while let Some((mut candidate, processed)) = candidates.pop() {
            // iterate in window of group size + 1, because groups must be surrounded by operational springs ('.')
            // println!(
            //     "Candidate: {:?} | {:?}",
            //     &candidate[0..processed],
            //     &candidate[processed..]
            // );

            for i in processed..candidate.len() - group_size {
                let window = candidate.get(i - 1..i + group_size + 1).unwrap();
                // println!("    window {:?}", window);

                // if we can place damaged group here, then place it and add as new candidate for next iteration
                if window.starts_with(&[Status::Operational])
                    && !window.ends_with(&[Status::Damaged])
                    && !window[1..1 + group_size].contains(&Status::Operational)
                {
                    let mut new_candidate = candidate.clone();
                    let mut new_group = Vec::new();
                    for _ in 0..*group_size {
                        new_group.push(Status::Damaged)
                    }
                    new_group.push(Status::Operational);
                    new_candidate.splice(i..i + group_size + 1, new_group);
                    // println!("     => Pushing new candidate {:?}", new_candidate);
                    new_candidates.push((new_candidate, i + group_size + 1));
                }

                if candidate[i] == Status::Unknown {
                    let _ = std::mem::replace(&mut candidate[i], Status::Operational);
                }
            }

            // evaluating
            //     .clone()
            //     .windows(group_size + 2)
            //     .for_each(|window| {
            //         println!("    window {:?}", window);

            //         // if we can place damaged group here, then place it and add as new candidate for next iteration
            //         if !window.starts_with(&[Status::Damaged])
            //             && !window.ends_with(&[Status::Damaged])
            //             && !window[1..window.len() - 1].contains(&Status::Operational)
            //         {
            //             let mut new_done: Vec<Status> = done.clone();
            //             new_done.push(Status::Operational);
            //             for _ in 0..*group_size {
            //                 new_done.push(Status::Damaged);
            //             }
            //             new_done.push(Status::Operational);

            //             let new_evaluating = evaluating
            //                 .get(group_size + 2..evaluating.len())
            //                 .unwrap()
            //                 .to_vec();
            //             println!(
            //                 "     => Pushing new candidate {:?}, {:?}",
            //                 new_done, new_evaluating
            //             );
            //             new_candidates.push((new_done, new_evaluating));
            //         }

            //         done.push(match evaluating.first().unwrap() {
            //             Status::Operational | Status::Unknown => Status::Operational,
            //             Status::Damaged => Status::Damaged,
            //         });
            //         evaluating.remove(0);
            //     });

            // for i in start_index..=candidate.len() - group_size - 2 {
            //     // iterate in window of group size + 2, because groups must be surrounded by operational springs ('.')
            //     let window = &candidate[i..i + group_size + 2];
            //     println!("Checking window {:?} in candidate {:?}", window, candidate);

            //     // if we can place damaged group here, then place it and add as new candidate for next iteration
            //     if !window.starts_with(&[Status::Damaged])
            //         && !window.ends_with(&[Status::Damaged])
            //         && !window[1..window.len() - 1].contains(&Status::Operational)
            //     {
            //         let new_candidate = candidate[0..i + 1].to_string()
            //             + &window[1..window.len() - 1].replace("?", "#")
            //             + &window[window.len() - 1..window.len() - 1].replace("?", ".")
            //             + &candidate[i + group_size + 2 - 1..];
            //         println!("==> Pushing new candidate {:?}", new_candidate);
            //         new_candidates.push((new_candidate, i + 1));
            //     }

            //     // if
            //     if candidate.get(i + 1..i + 2).unwrap() == "?" {
            //         candidate.replace_range(i + 1..i + 2, ".");
            //     }
            // }
        }

        candidates = new_candidates;
    }

    // println!("\n\nFinal candidates :");
    // candidates.iter().for_each(|c| println!("  {:?}", c));
    candidates
        .iter()
        .filter(|(c, _)| valid_arrangement(&c, &damaged_spring_groups))
        .count()
}

fn valid_arrangement(candidate: &Vec<Status>, damaged_spring_groups: &Vec<usize>) -> bool {
    candidate
        .split(|x| *x == Status::Operational || *x == Status::Unknown)
        .filter_map(|s| if s.len() > 0 { Some(s.len()) } else { None })
        .collect::<Vec<usize>>()
        .eq(damaged_spring_groups)
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
    fn test_count_possible_arrangements_7() {
        let (springs, groups) = parse_line("#????##????#?#??#?? 1,1,13");
        assert_eq!(count_possible_arrangements(springs, groups), 3);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 21);
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
