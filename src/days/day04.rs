use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::str::Lines;
use std::usize;

fn parse_card_id(input: &str) -> usize {
    input
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

fn parse_numbers(input: &str) -> HashSet<usize> {
    input
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn part_1(lines: Lines) -> usize {
    lines
        .map(
            |line| match line.split([':', '|']).collect::<Vec<&str>>()[..] {
                [_, winners_str, scratched_str] => {
                    let winners = parse_numbers(winners_str);
                    let scratched = parse_numbers(scratched_str);
                    let wins = scratched.iter().filter(|x| winners.contains(x)).count();

                    if wins > 0 {
                        2_usize.pow((wins - 1) as u32)
                    } else {
                        0
                    }
                }
                _ => panic!("Invalid input: '{:?}'", line),
            },
        )
        .sum()
}

fn part_2(lines: Lines) -> usize {
    let mut card_amounts: HashMap<usize, usize> = HashMap::new();

    lines.for_each(
        |line| match line.split([':', '|']).collect::<Vec<&str>>()[..] {
            [card_id_str, winners_str, scratched_str] => {
                let card_id = parse_card_id(card_id_str);
                let current_card_amount = *card_amounts
                    .entry(card_id)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);

                let winners = parse_numbers(winners_str);
                let scratched = parse_numbers(scratched_str);
                let wins = scratched.iter().filter(|x| winners.contains(x)).count();

                for id in card_id + 1..=card_id + wins {
                    *card_amounts.entry(id).or_insert(0) += current_card_amount;
                }
            }
            _ => panic!("Invalid input: '{:?}'", line),
        },
    );

    card_amounts.values().sum()
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_4");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 13);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_4").lines()), 26914);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 30);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_4").lines()), 13080971);
    }
}
