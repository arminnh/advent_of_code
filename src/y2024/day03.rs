

use regex::Regex;
use std::usize;

fn parse_num(left: &str) -> usize {
    left.parse::<usize>().expect("could not parse number")
}

// Add up all the results of the mult operations
pub fn part_1(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    re.captures_iter(&input)
        .map(|c| c.extract().1)
        .map(|[left, right]| parse_num(left) * parse_num(right))
        .sum::<usize>()
}

// The most recent do() or don't() instruction determines whether to apply the mul()
pub fn part_2(input: &str) -> usize {
    let re = Regex::new(r"(?m)(do|don't)(\(\))|mul\(([0-9]+),([0-9]+)\)").unwrap();

    re.captures_iter(&input)
        .map(|c| c.extract().1)
        .fold(("do", 0), |(last, result), [left, right]| match left {
            "do" => (left, result),
            "don't" => (left, result),
            _ => match last {
                "do" => (last, result + parse_num(left) * parse_num(right)),
                _ => (last, result),
            },
        })
        .1
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;
    use super::*;

    const EXAMPLE_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(&EXAMPLE_INPUT), 161);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2024/day_3")), 173517243);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_2), 48);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2024/day_3")), 100450138);
    }
}
