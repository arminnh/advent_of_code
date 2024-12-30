use std::collections::HashMap;
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

fn next_node<'a>(
    network: &'a HashMap<String, (String, String)>,
    current_node: &str,
    instruction: &char,
) -> &'a String {
    match network.get(current_node) {
        Some((left, right)) => match instruction {
            'L' => left,
            'R' => right,
            _ => panic!("Invalid instruction: {:?}", instruction),
        },
        None => panic!("Node '{:?}' does not exist in network.", current_node),
    }
}

pub fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut current_node = "AAA";
    let end_node = "ZZZ";
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let mut instructions_iter = instructions.iter().cycle();
    let network: HashMap<String, (String, String)> = parse_network(lines);
    let mut count = 0;

    while current_node != end_node {
        count += 1;
        match instructions_iter.next() {
            Some(instruction) => current_node = next_node(&network, current_node, instruction),
            None => panic!("Ran out of instructions."),
        }
    }

    count
}

#[allow(dead_code)]
pub fn part_2_bruteforce(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let mut instructions_iter = instructions.iter().cycle();
    let network: HashMap<String, (String, String)> = parse_network(lines);
    let mut current_nodes: Vec<&String> = network.keys().filter(|x| x.ends_with('A')).collect();
    let mut count = 0;

    while current_nodes.iter().filter(|x| !x.ends_with('Z')).count() != 0 {
        count += 1;
        match instructions_iter.next() {
            Some(instruction) => {
                current_nodes = current_nodes
                    .iter()
                    .map(|node| next_node(&network, node, instruction))
                    .collect();
            }
            None => panic!("Ran out of instructions."),
        }
    }

    count
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(numbers: Vec<usize>) -> usize {
    numbers.iter().fold(1, |acc, x| acc * x / gcd(acc, *x))
}

pub fn part_2(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    let network: HashMap<String, (String, String)> = parse_network(lines);

    lcm(network
        .keys()
        .filter(|x| x.ends_with('A'))
        .map(|mut node| {
            let mut instructions_iter = instructions.iter().cycle();
            let mut steps = 0;

            while !node.ends_with('Z') {
                steps += 1;
                node = next_node(&network, node, instructions_iter.next().unwrap())
            }

            steps
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

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

    const EXAMPLE_INPUT_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 2);
        assert_eq!(part_1(EXAMPLE_INPUT_2), 6);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2023/day_8")), 18157);
    }

    #[test]
    fn test_part_2_bruteforce_example() {
        assert_eq!(part_2_bruteforce(EXAMPLE_INPUT_3), 6);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_3), 6);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2023/day_8")), 14299763833181);
    }
}
