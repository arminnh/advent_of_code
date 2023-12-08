use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs;
use std::str::Lines;
use std::usize;

fn parse_network(lines: Lines) -> HashMap<String, (String, String)> {
    let mut out = HashMap::new();

    lines.for_each(|line| {
        match line
            .replace(&['(', ')', ' '], "")
            .split(['=', ','])
            .collect::<Vec<&str>>()[..]
        {
            [node, left, right] => {
                out.insert(node.to_string(), (left.to_string(), right.to_string()));
            }
            [""] => (),
            _ => panic!("Unsupported network line: {:?}", line),
        };
    });

    out
}

fn part_1(mut lines: Lines) -> usize {
    let mut current_node = "AAA";
    let end_node = "ZZZ";
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let mut instructions_iter = instructions.iter().cycle();
    let network: HashMap<String, (String, String)> = parse_network(lines);
    let mut count = 0;

    while current_node != end_node {
        count += 1;
        match instructions_iter.next() {
            Some(instruction) => match network.get(current_node) {
                Some((left, right)) => match instruction {
                    'L' => current_node = left,
                    'R' => current_node = right,
                    _ => panic!("Invalid instruction: {:?}", instruction),
                },
                None => panic!("Node '{:?}' does not exist in network.", current_node),
            },
            None => panic!("Ran out of instructions."),
        }
    }

    count
}

fn part_2(lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let contents = fs::read_to_string("inputs/day_8").expect("Could not open file.");

    (
        Solution::from(part_1(contents.lines())),
        Solution::from(part_2(contents.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

const EXAMPLE_INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 2);
        assert_eq!(part_1(EXAMPLE_INPUT_2.lines()), 6);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT.lines()), 0);
    // }
}
