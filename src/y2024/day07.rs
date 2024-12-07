use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;
use std::usize;

struct Equation {
    result: usize,
    operands: Vec<usize>,
}

impl Equation {
    // Try all possible operations between the operands, keeping track of intermediate results
    fn result_is_possible(&self) -> bool {
        let mut accumulators = Vec::from([*self.operands.first().unwrap()]);

        for o in &self.operands[1..self.operands.len()] {
            let previous: Vec<usize> = accumulators.drain(..).collect();
            for acc in previous {
                let mul = acc * o;
                let add = acc + o;
                if mul == self.result || add == self.result {
                    return true;
                }
                if mul < self.result {
                    accumulators.push(mul);
                }
                if add < self.result {
                    accumulators.push(add);
                }
            }
        }

        false
    }
}

impl From<&str> for Equation {
    fn from(input: &str) -> Self {
        let mut iter = input.split_whitespace();
        let result = iter
            .next()
            .expect("Input is empty")
            .trim_end_matches(":")
            .parse()
            .expect("Could not parse result of equation");
        let operands = iter
            .map(|o| o.parse().expect("Could not parse operand"))
            .collect();
        Equation { result, operands }
    }
}

// Predict the path of the guard. How many distinct positions will the guard visit before leaving the mapped area?
fn part_1(lines: Lines) -> usize {
    lines
        .map(|line| Equation::from(line))
        .filter_map(|equation| {
            if equation.result_is_possible() {
                Some(equation.result)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_7");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 3749);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_7").lines()), 1260333054159);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), 6);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_7").lines()), 1670)
    }
}
