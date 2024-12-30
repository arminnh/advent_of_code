use std::usize;

struct Equation {
    result: usize,
    operands: Vec<usize>,
}

impl Equation {
    // Try all possible operations between the operands, keeping track of intermediate results
    fn result_is_possible(&self, use_concatenation: bool) -> bool {
        let mut accumulators = Vec::from([self.operands[0]]);

        for o in &self.operands[1..] {
            let previous: Vec<usize> = accumulators.drain(..).collect();
            for acc in previous {
                let mul = acc * o;
                let add = acc + o;
                if mul <= self.result {
                    accumulators.push(mul);
                }
                if add <= self.result {
                    accumulators.push(add);
                }

                if use_concatenation {
                    let con = concatenate(acc, *o);
                    if con <= self.result {
                        accumulators.push(con);
                    }
                }
            }
        }

        accumulators.contains(&self.result)
    }
}

impl From<&str> for Equation {
    fn from(input: &str) -> Self {
        let (result, operands) = input.split_once(": ").unwrap();

        Equation {
            result: result.parse().expect("Could not parse result of equation"),
            operands: operands
                .split(" ")
                .map(|o| o.parse().expect("Could not parse operand"))
                .collect(),
        }
    }
}

// What is the total calibration result of the equations that could possibly be true?
pub fn part_1(input: &str) -> usize {
    let use_concatenation = false;
    input
        .lines()
        .map(|line| Equation::from(line))
        .filter(|equation| equation.result_is_possible(use_concatenation))
        .map(|equation| equation.result)
        .sum()
}

fn concatenate(a: usize, b: usize) -> usize {
    a * 10_usize.pow(b.ilog10() + 1) + b
    // (a.to_string() + &b.to_string()).parse().unwrap()
}

// Part 1 + concatenation operator
pub fn part_2(input: &str) -> usize {
    let use_concatenation = true;
    input
        .lines()
        .map(|line| Equation::from(line))
        .filter(|equation| equation.result_is_possible(use_concatenation))
        .map(|equation| equation.result)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::util::load_input;

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
        assert_eq!(part_1(EXAMPLE_INPUT), 3749);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2024/day_7")), 1260333054159);
    }

    #[test]
    fn test_concatenate() {
        assert_eq!(concatenate(1, 1), 11);
        assert_eq!(concatenate(22, 2), 222);
        assert_eq!(concatenate(333, 333), 333333);
        assert_eq!(concatenate(1, 61011), 161011);
        assert_eq!(concatenate(16, 1011), 161011);
        assert_eq!(concatenate(161, 011), 16111);
        assert_eq!(concatenate(1610, 11), 161011);
        assert_eq!(concatenate(16101, 1), 161011);
        assert_eq!(concatenate(10, 10), 1010);
        assert_eq!(concatenate(1, 30000), 130000);
        assert_eq!(concatenate(50000, 1), 500001);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT), 11387);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2024/day_7")), 162042343638683)
    }
}
