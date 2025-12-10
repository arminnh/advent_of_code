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
    let start_state: Vec<bool> = (0..machine.lights).map(|_| false).collect();
    let mut presses = 0;
    // Bruteforce search approach -- just try all combinations until the target is reached
    let mut states: Vec<Vec<bool>> = Vec::from([start_state]);
    // let mut paths: VecDeque<(Vec<bool>)> =
    //     machine.buttons.iter().map(|b| (start_state, *b)).collect();
    while !states.contains(&machine.lights_target) {
        let mut next_states = Vec::new();
        for s in states {
            for b in &machine.buttons {
                next_states.push(next_state(s.clone(), b));
            }
        }
        states = next_states;
        presses += 1;
    }
    presses
}

fn next_state(mut state: Vec<bool>, button: &Vec<u8>) -> Vec<bool> {
    for i in button {
        state[*i as usize] = !state[*i as usize];
    }
    state
}

// What is the fewest button presses required to correctly configure the joltage level counters on all of the machines?
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
