use std::collections::HashMap;
use std::mem::swap;
use std::usize;

#[derive(PartialEq, Eq, Debug)]
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

#[derive(Debug)]
struct Gate {
    a: String,
    b: String,
    op: Operation,
}

impl Gate {
    fn from(a: &str, b: &str, op: Operation) -> Self {
        Gate {
            a: a.to_string(),
            b: b.to_string(),
            op,
        }
    }

    fn from_str(s: &str) -> Self {
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

impl PartialEq for Gate {
    fn eq(&self, other: &Self) -> bool {
        self.op == other.op
            && ((self.a == other.a && self.b == other.b)
                || (self.a == other.b && self.b == other.a))
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
            (target_wire.to_string(), Gate::from_str(gate))
        })
        .collect();

    (wires, gates)
}

fn resolve(
    target_wire: &str,
    wires: &mut HashMap<String, bool>,
    gates: &mut Vec<(String, Gate)>,
) -> bool {
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

// Convert to decimal. Bits are in order of least significant to most significant bit
fn binary_to_decimal(bits: &[bool]) -> usize {
    bits.iter().enumerate().fold(0, |acc, (i, bit)| {
        acc + (*bit as usize) * 2_usize.pow(i as u32)
    })
}

fn wires_to_decimal(wires: &HashMap<String, bool>, first_letter: &str) -> usize {
    let mut wires_filtered: Vec<&String> = wires
        .keys()
        .filter(|key| key.starts_with(first_letter))
        .collect();
    wires_filtered.sort();
    let bits = &wires_filtered
        .iter()
        .map(|&w| *wires.get(w).unwrap())
        .collect::<Vec<bool>>();

    binary_to_decimal(bits)
}

// Simulate the system of gates and wires. What decimal number does it output on the wires starting with z?
pub fn part_1(input: &str) -> usize {
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

    wires_to_decimal(&wires, "z")
}

fn gate_index(gates: &Vec<(String, Gate)>, gate: &Gate) -> Option<usize> {
    gates.iter().position(|(_, g)| g == gate)
}

fn resulting_wire(gates: &Vec<(String, Gate)>, gate: &Gate) -> String {
    gates[gate_index(gates, gate).expect("Could not find gate")]
        .0
        .clone()
}

fn swap_wires(
    gates: &mut Vec<(String, Gate)>,
    swapped: &mut Vec<String>,
    gate_index: usize,
    expected: &str,
    actual: &str,
) {
    let other_index = gates.iter().position(|(w, _)| w == &expected).unwrap();
    // println!(
    //     "swap: expected: {} at index {}, actual: {} at index {}",
    //     expected, actual, gate_index, other_index
    // );
    swapped.push(expected.to_string());
    swapped.push(actual.to_string());

    gates[gate_index].0 = expected.to_string();
    gates[other_index].0 = actual.to_string();
}

// Your system of gates and wires has four pairs of gates which need their output wires swapped - eight wires in total.
// Determine which four pairs of gates need their outputs swapped so that your system correctly performs addition;
// what do you get if you sort the names of the eight wires involved in a swap and then join those names with commas?
pub fn part_2(input: &str) -> String {
    // The system seems to perform addition using a collection of full adders
    // This means we should be able to find the following operations in the input:
    // X_i XOR Y_i -> tmp_1
    // tmp_1 XOR C_in_i -> Z_i
    // tmp_1 AND C_in_i -> tmp_2
    // X_i AND Y_i -> tmp_3
    // tmp_2 OR tmp_3 -> C_in_i+1
    // Special case X_0 and Y_0:
    //     * X_0 XOR Y_0 -> Z_0
    //     * X_0 AND Y_0 -> C_in_1
    let (_, mut gates) = parse_input(input);
    let mut swapped = Vec::<String>::new();

    let mut c_in = "mqs".to_string();
    // Iterate through each bit and ensure that gates lead to the correct wires
    for i in 1..45 {
        let x_i = format!("x{:0>2}", i);
        let y_i = format!("y{:0>2}", i);
        let z_i = format!("z{:0>2}", i);
        // println!("{} -> {}, {}, {}, c_in: {}", i, x_i, y_i, z_i, c_in);
        // X_i XOR Y_i -> tmp_1
        let x_xor_y = Gate::from(&x_i, &y_i, Operation::XOR);
        let mut tmp_1 = resulting_wire(&gates, &x_xor_y);

        // X_i AND Y_i -> tmp_3
        let x_and_y = Gate::from(&x_i, &y_i, Operation::AND);
        let mut tmp_3 = resulting_wire(&gates, &x_and_y);

        // Each iteration, we know which combination of wires leads to z_i. If the next wire
        // does not result in z_i, then swap it with z_i so rest of the checks can finish successfully
        // tmp_1 XOR C_in_i -> Z_i
        let tmp1_xor_cin = Gate::from(&tmp_1, &c_in, Operation::XOR);
        let i = gate_index(&gates, &tmp1_xor_cin).unwrap_or_else(|| {
            // Gate using tmp1 could not be found. -> Must have been swapped with tmp3
            // println!("Swapping tmp1 and tmp3");
            let tmp_1_i = gate_index(&gates, &x_xor_y).expect("x_xor_y");
            swap_wires(&mut gates, &mut swapped, tmp_1_i, &tmp_1, &tmp_3);
            swap(&mut tmp_1, &mut tmp_3);
            // Search again after swapping
            let tmp1_xor_cin = Gate::from(&tmp_1, &c_in, Operation::XOR);
            gate_index(&gates, &tmp1_xor_cin)
                .expect("Could not find tmp1_xor_cin even after swapping")
        });

        let actual_wire = gates[i].0.clone();
        if actual_wire != z_i {
            // Expected z_i but found something else -> swap
            swap_wires(&mut gates, &mut swapped, i, &z_i, &actual_wire);
            if tmp_3 == z_i {
                tmp_3 = actual_wire;
            }
        }

        // tmp_1 AND C_in_i -> tmp_2
        let tmp1_and_c = Gate::from(&tmp_1, &c_in, Operation::AND);
        let tmp_2 = resulting_wire(&gates, &tmp1_and_c);

        // tmp_2 OR tmp_3 -> C_in_i+1
        let tmp2_or_tmp3 = Gate::from(&tmp_2, &tmp_3, Operation::OR);
        let next_c_in = resulting_wire(&gates, &tmp2_or_tmp3);

        c_in = next_c_in.clone();
    }

    swapped.sort();
    swapped.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::util::load_input;

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
        assert_eq!(part_1(&load_input("inputs/2024/day_24")), 42410633905894);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(&load_input("inputs/2024/day_24")),
            "cqm,mps,vcv,vjv,vwp,z13,z19,z25".to_string()
        )
    }
}
