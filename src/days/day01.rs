use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs;
use std::str::Lines;

fn load_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not open file.")
}

fn part_1(lines: Lines) -> u32 {
    lines
        .map(|line| {
            let first = line
                .chars()
                .skip_while(|c| !c.is_ascii_digit())
                .next()
                .unwrap();
            let last = line
                .chars()
                .rev()
                .skip_while(|c| !c.is_ascii_digit())
                .next()
                .unwrap();

            first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
        })
        .sum()
}

fn parse_line(line: &str) -> usize {
    let letters = HashMap::from([
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ]);

    let indexes: Vec<(usize, &str)> = letters
        .keys()
        .flat_map(|k| line.match_indices(k))
        .collect();
    println!("{:?}", indexes);

    let (_, first) = indexes.iter().min_by_key(|(index, _)| index).unwrap();
    let (_, last) = indexes.iter().max_by_key(|(index, _)| index).unwrap();

    letters.get(first).unwrap() * 10 + letters.get(last).unwrap()
}

fn part_2(lines: Lines) -> usize {
    lines.map(|line| parse_line(line)).sum()
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

    const EXAMPLE_INPUT_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE_INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 142);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_1").lines()), 55208);
    }

    #[test]
    fn test_parse_letters() {
        assert_eq!(parse_line("two1nine"), 29);
        assert_eq!(parse_line("eightwothree"), 83);
        assert_eq!(parse_line("4nineeightseven2"), 42);
        assert_eq!(parse_line("zoneight234"), 14);
        assert_eq!(parse_line("7pqrstsixteen"), 76);
        assert_eq!(parse_line("rxcsh1"), 11);
        assert_eq!(
            parse_line("hlcrfjjkjqrvsevenbtdkvzqvxgrjdcmhggcqrr7fiveeight"),
            78
        );
        assert_eq!(parse_line("z8"), 88);
        assert_eq!(parse_line("18cfour"), 14);
        assert_eq!(parse_line("418cfour"), 44);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_2.lines()), 281);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_1").lines()), 54578);
    }
}
