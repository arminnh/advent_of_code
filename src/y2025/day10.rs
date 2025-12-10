use std::collections::VecDeque;

#[derive(Debug)]
struct Machine {
    lights: u8,               // nr of lights, max 10
    lights_target: Vec<bool>, // target configuration of lights
    buttons: Vec<Vec<u8>>,    // each button is a list of lights it toggles
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
            lights: lights.len() as u8,
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
        .map(|machine| fewest_button_presses(machine))
        .sum()
}

// The least nr of button presses to set the machine to the target configuration
fn fewest_button_presses(machine: Machine) -> usize {
    // Each press of a button toggles the lights
    // So pressing it once is the same as pressing any uneven number of times
    // In other words: look for the combination of buttons that results in the target
    let start_state: Vec<bool> = (0..machine.lights).map(|_| false).collect();
    // Will check each possible combination, starting with each button by itself
    let mut combinations: VecDeque<Vec<usize>> =
        (0..machine.buttons.len()).map(|i| vec![i]).collect();

    while let Some(combination) = combinations.pop_front() {
        let mut state = start_state.clone();
        let nr_of_buttons = combination.len();
        for button_index in &combination {
            for i in machine.buttons[*button_index].clone() {
                state[i as usize] = !state[i as usize];
            }
        }
        if state == machine.lights_target {
            return nr_of_buttons;
        }
        // Form next combinations by adding buttons after the last one used in the current combination
        let last_button_index = combination[nr_of_buttons - 1];
        for j in last_button_index + 1..machine.buttons.len() {
            combinations.push_back(combination.iter().copied().chain([j].into_iter()).collect());
        }
    }
    0
}

// What is the lowest nr of button presses required to configure the joltage counters on all of the machines?
pub fn part_2(input: &str) -> usize {
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

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT_1), 33);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(&load_input("inputs/2025/day_10")), 0);
    // }
}
