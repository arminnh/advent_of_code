use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::usize;

enum Operation {
    AND,
    OR,
    XOR,
}

impl Operation {
    fn from(s: &str) -> Self {
        match s {
            "AND" => Operation::AND,
            "OR" => Operation::OR,
            "XOR" => Operation::XOR,
            _ => panic!("Invalid operation"),
        }
    }
}

struct Gate {
    a: String,
    b: String,
    op: Operation,
}

impl Gate {
    fn from(s: &str) -> Self {
        match s.split(" ").collect::<Vec<_>>()[..] {
            [a, op, b] => Gate {
                a: a.to_string(),
                b: b.to_string(),
                op: Operation::from(op),
            },
            _ => panic!("Could not parse gate."),
        }
    }
}

fn parse_input(input: &str) -> (HashMap<String, bool>, Vec<(String, Gate)>) {
    let (wires, gates) = input
        .split_once("\n\n")
        .expect("Could not split input in two");

    let wires: HashMap<String, bool> = wires
        .lines()
        .map(|line| {
            let (wire, value) = line.split_once(": ").expect("Could not parse wire");
            (wire.to_string(), value == "1")
        })
        .collect();

    let gates: Vec<(String, Gate)> = gates
        .lines()
        .map(|line| {
            let (gate, target_wire) = line.split_once(" -> ").unwrap();
            (target_wire.to_string(), Gate::from(gate))
        })
        .collect();

    (wires, gates)
}

fn resolve(
    target_wire: &str,
    wires: &mut HashMap<String, bool>,
    gates: &mut Vec<(String, Gate)>,
) -> bool {
    println!("Resolving '{}'", target_wire);
    // Each wire is connected to at most one gate output, but can be connected to many gate inputs.
    let i = gates
        .iter()
        .position(|(output_wire, _)| output_wire == target_wire)
        .expect("Could not find a gate that results in the target wire");
    let (_, gate) = gates.remove(i);

    let a = match wires.get(&gate.a) {
        Some(value) => *value,
        None => resolve(&gate.a, wires, gates),
    };
    let b = match wires.get(&gate.b) {
        Some(value) => *value,
        None => resolve(&gate.b, wires, gates),
    };
    let result = match gate.op {
        Operation::AND => a && b,
        Operation::OR => a || b,
        Operation::XOR => a ^ b,
    };
    wires.insert(target_wire.to_string(), result);
    result
}

// Simulate the system of gates and wires. What decimal number does it output on the wires starting with z?
fn part_1(input: &str) -> usize {
    let (mut wires, mut gates) = parse_input(input);

    while let Some((wire, gate)) = gates.pop() {
        let a = match wires.get(&gate.a) {
            Some(value) => *value,
            None => resolve(&gate.a, &mut wires, &mut gates),
        };
        let b = match wires.get(&gate.b) {
            Some(value) => *value,
            None => resolve(&gate.b, &mut wires, &mut gates),
        };
        let result = match gate.op {
            Operation::AND => a && b,
            Operation::OR => a || b,
            Operation::XOR => a ^ b,
        };
        wires.insert(wire, result);
    }

    let mut z_wires: Vec<_> = wires.keys().filter(|key| key.starts_with("z")).collect();
    z_wires.sort();
    println!("{:?}", z_wires);
    // Convert to decimal. Wires are in order of least significant to most significant bit
    z_wires.into_iter().enumerate().fold(0, |acc, (i, wire)| {
        acc + (*wires.get(wire).unwrap() as usize) * 2_usize.pow(i as u32)
    })
}

fn part_2(input: &str) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_24");
    (
        Solution::from(part_1(&input)),
        Solution::from(part_2(&input)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const EXAMPLE_INPUT_2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 4);
        assert_eq!(part_1(EXAMPLE_INPUT_2), 2024);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2024/day_24")), 0);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2024/day_24")), 0)
    }
}
