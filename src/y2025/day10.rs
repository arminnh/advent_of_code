#[derive(Debug)]
struct Machine {
    lights: u8,            // nr of lights, max 10
    target: Vec<bool>,     // target configuration of lights
    buttons: Vec<Vec<u8>>, // each button is a list of lights it toggles
    joltage: Vec<u8>,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        let target: Vec<bool> = parts
            .next()
            .expect("Could not find lights config")
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
        let buttons: Vec<Vec<u8>> = parts
            .take_while(|s| s.starts_with('('))
            .map(|s| {
                s[1..s.len() - 1] // skip brackets
                    .split(',')
                    .map(|c| c.parse::<u8>().expect("Could not parse light nr"))
                    .collect()
            })
            .collect();

        Machine {
            lights: target.len() as u8,
            target: target,
            buttons: buttons,
            joltage: Vec::new(),
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
    while !states.contains(&machine.target) {
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

//
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
    //     assert_eq!(part_2(EXAMPLE_INPUT_1), 0);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(&load_input("inputs/2025/day_10")), 0);
    // }
}
