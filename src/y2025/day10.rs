use std::collections::{BinaryHeap, HashMap, VecDeque};

#[derive(Debug)]
struct Machine {
    lights_target: Vec<bool>, // target configuration of lights
    buttons: Vec<Vec<u8>>,    // each button is a list of lights indices it toggles
    joltage_target: Vec<u16>, // part 2: buttons increment joltage counters instead of toggling lights
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split_whitespace().collect();
        let lights: Vec<bool> = parts[0]
            .chars()
            .filter_map(|c| {
                if c == '#' {
                    Some(true)
                } else if c == '.' {
                    Some(false)
                } else {
                    None // Ignore brackets
                }
            })
            .collect();
        let buttons: Vec<Vec<u8>> = parts[1..parts.len() - 1]
            .iter()
            .map(|s| {
                s[1..s.len() - 1] // skip brackets
                    .split(',')
                    .map(|c| c.parse::<u8>().expect("Could not parse light nr"))
                    .collect()
            })
            .collect();
        let joltage_str = parts[parts.len() - 1];
        let joltage: Vec<u16> = joltage_str[1..joltage_str.len() - 1]
            .split(',')
            .map(|c| c.parse::<u16>().expect("Could not parse joltage nr"))
            .collect();
        if lights.len() != joltage.len() {
            panic!("Nr of lights does not equal nr of joltage counters");
        }

        Machine {
            lights_target: lights,
            buttons: buttons,
            joltage_target: joltage,
        }
    }
}

// What is the lowest nr of button presses required to configure the lights on all of the machines
pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| Machine::from(line))
        .map(|machine| fewest_button_presses_lights(machine))
        .sum()
}

// The least nr of button presses to set the machine to the target lights configuration
fn fewest_button_presses_lights(machine: Machine) -> usize {
    // Each press of a button toggles the lights
    // So pressing it once is the same as pressing any uneven number of times
    // In other words: look for the combination of buttons that results in the target
    // Will check each possible combination, starting with each button by itself
    let mut combinations: VecDeque<Vec<usize>> =
        (0..machine.buttons.len()).map(|i| vec![i]).collect();
    // Convert list of booleans into one number for easy XOR
    let target_state: u16 =
        machine.lights_target.iter().rev().fold(
            0,
            |acc, b| {
                if *b {
                    (acc << 1) + 1
                } else {
                    acc << 1
                }
            },
        );
    // Convert buttons from nested vecs of indices to vec of u16 for XORing with
    let buttons: Vec<u16> = machine
        .buttons
        .iter()
        .map(|button| button.iter().fold(0, |acc, i| acc + (1 << i)))
        .collect();

    while let Some(combination) = combinations.pop_front() {
        let mut state: u16 = 0;
        for button_index in &combination {
            state ^= buttons[*button_index];
        }
        if state == target_state {
            return combination.len();
        }
        // Form next combinations by adding buttons after the last one used in the current combination
        let last_button_index = combination[combination.len() - 1];
        for j in last_button_index + 1..machine.buttons.len() {
            combinations.push_back(combination.iter().copied().chain([j].into_iter()).collect());
        }
    }
    0
}

// What is the lowest nr of button presses required to configure the joltage counters on all of the machines?
pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| Machine::from(line))
        .map(|machine| fewest_button_presses_joltage(machine))
        .sum()
}

/*
The least nr of button presses to set the machine to the target joltage counters

Search space has exploded, since now targeting a list of numbers instead of a list of booleans. XOR trick won't work.
First row in input has 13 buttons and highest counter of 66. So search space of 13^66 for only first of 200 machines.
Too many states for BFS/DFS. Could not make it work with an A* heuristic.

Other perspective: system of linear equations: M * x = T
Where:
    M is matrix of buttons. M_ij = 1 when button j increments counter i
    x = unknown vector. x_i is number of times button i must be pressed (integer value >= 0)
    T = vector of target counters
We want the minimum sum of all x_i that satisfies M*x=T with all x_i >= 0

There can be more buttons (nr of unknowns) than equations (nr of counters), so there can be free variables.
Could use an Integer Linear Program (ILP) solver to find the solution
From checking a few inputs manually, it seems like the number of free variables is relatively small

Will try solving by:
1. Gaussian elimination / reduced row echelon form
2. Detect free variables (see pivot columns)
3. Search valid combinations of free variables (or nullspace parametrization?)
4. Select combination resulting in min x
*/
fn fewest_button_presses_joltage(machine: Machine) -> usize {
    println!("{:?}", machine);
    // state = joltage counts sum, joltage counts & nr of button presses
    let start_state = (0, vec![0; machine.joltage_target.len()], 0);
    let mut frontier: BinaryHeap<(isize, Vec<u16>, usize)> =
        BinaryHeap::from([start_state.clone()]);
    // keep track of minimum number of presses needed to reach each state
    let mut min_presses: HashMap<Vec<u16>, usize> = HashMap::from([(start_state.1, 0)]);

    // Heuristic function for A* scoring.
    let h = |counts: &[u16]| -> usize {
        counts
            .iter()
            .zip(machine.joltage_target.iter())
            .map(|(c, t)| *t - *c)
            .max()
            .unwrap_or(0) as usize
    };

    while let Some((_, counts, presses)) = frontier.pop() {
        if counts == machine.joltage_target {
            println!("---> {}", presses);
            return presses;
        }

        for button in &machine.buttons {
            let mut next_counts = counts.clone();
            let mut exceeded = false;
            for b in button {
                let joltage_i = *b as usize;
                next_counts[joltage_i] += 1;
                if next_counts[joltage_i] > machine.joltage_target[joltage_i] {
                    exceeded = true;
                    break;
                }
            }
            if exceeded {
                continue;
            }

            let next_presses = presses + 1;
            if next_presses < *min_presses.get(&next_counts).unwrap_or(&usize::MAX) {
                min_presses.insert(next_counts.clone(), next_presses);
                let next_cost = (next_presses + h(&next_counts)) as isize;
                frontier.push((-next_cost, next_counts, next_presses));
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

    use super::*;

    const EXAMPLE_INPUT_1: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 7);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_10")), 507);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1), 33);
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(&load_input("inputs/2025/day_10")), 0);
    // }
}
