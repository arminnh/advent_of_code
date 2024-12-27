use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::i32;
use std::str::Lines;

type Position = (i32, i32);
type Keypad = HashMap<Position, char>;

// The numeric keypad has four rows of buttons: 789, 456, 123, and finally an empty gap followed by 0A.
const NUMERIC_KEYPAD_ENTRIES: [(Position, char); 11] = [
    ((0, 0), '7'),
    ((0, 1), '8'),
    ((0, 2), '9'),
    ((1, 0), '4'),
    ((1, 1), '5'),
    ((1, 2), '6'),
    ((2, 0), '1'),
    ((2, 1), '2'),
    ((2, 2), '3'),
    ((3, 1), '0'),
    ((3, 2), 'A'),
];

// The directional keypad has two rows of buttons: a gap / ^ (up) / A (activate) on the first row
// and < (left) / v (down) / > (right) on the second row
const DIRECTIONAL_KEYPAD_ENTRIES: [(Position, char); 5] = [
    ((0, 1), '^'),
    ((0, 2), 'A'),
    ((1, 0), '<'),
    ((1, 1), 'v'),
    ((1, 2), '>'),
];

// Return all shortest paths between two buttons on the given keypad
fn keypad_paths(from: char, to: char, keypad: &Keypad) -> Vec<Vec<char>> {
    let start_pos = *keypad
        .iter()
        .find(|(_, &c)| c == from)
        .expect("Target does not exist")
        .0;
    let target_pos = keypad
        .iter()
        .find(|(_, &c)| c == to)
        .expect("Target does not exist")
        .0;
    // Keep track of positions of each possible path and the keypad moves made for that path
    let mut paths: Vec<(Vec<Position>, Vec<char>)> = Vec::new();
    // Store all possible paths currently being explored
    let mut frontier: Vec<(Vec<Position>, Vec<char>)> = vec![(vec![start_pos], vec![])];
    let moves = [((-1, 0), '^'), ((0, 1), '>'), ((1, 0), 'v'), ((0, -1), '<')];
    let mut shortest_path = usize::MAX;

    while let Some((path_positions, path_moves)) = frontier.pop() {
        let last_pos = path_positions.last().unwrap();
        if path_positions.len() > shortest_path {
            continue;
        } else if last_pos == target_pos {
            shortest_path = shortest_path.min(path_positions.len());
            paths.push((
                path_positions,
                // Add 'A' to 'press' the button
                path_moves.into_iter().chain(vec!['A']).collect(),
            ));
        } else {
            for ((dx, dy), direction) in moves {
                let next_pos = (last_pos.0 + dx, last_pos.1 + dy);
                if !path_positions.contains(&next_pos) && keypad.contains_key(&next_pos) {
                    let mut next_positions = path_positions.clone();
                    next_positions.push(next_pos);
                    let mut next_moves = path_moves.clone();
                    next_moves.push(direction);
                    frontier.push((next_positions, next_moves));
                }
            }
        }
    }

    paths.retain(|(p, _)| p.len() == shortest_path);
    paths.into_iter().map(|(_, moves)| moves).collect()
}

// In summary, there are the following keypads:
//   One directional keypad that you are using.
//   Two directional keypads that robots are using.
//   One numeric keypad (on a door) that a robot is using.
// Numeric <- robot_1 <- directional <- robot_2 <- directional <- robot_3 <- directional <- ME
// The given code is what the first robot must press on the numeric keypad.
// (1) figure out the movements robot_1 must make == conversion from numeric code to directional code
// The directional code is what robot_2 must press on the directoinal keypad.
// (2) Convert these movements into another set of movements for robot_3
// (3) Apply (2) again to get the code for ME
fn nr_of_button_presses(code: &str) -> usize {
    let numeric_keypad: Keypad = HashMap::from(NUMERIC_KEYPAD_ENTRIES);
    let directional_keypad: Keypad = HashMap::from(DIRECTIONAL_KEYPAD_ENTRIES);

    let paths_robot_2: Vec<Vec<char>> = paths_for_code(code.chars().collect(), &numeric_keypad);

    let iterate_directional_keypad = |paths: Vec<Vec<char>>|{
        let mut new_paths: Vec<Vec<char>> = paths
            .into_iter()
            .flat_map(|p| paths_for_code(p, &directional_keypad))
            .collect();
        let shortest_path_len: usize = new_paths
            .iter()
            .min_by_key(|p| p.len())
            .expect("No path found!")
            .len();
        new_paths.retain(|p| p.len() == shortest_path_len);
        new_paths
    };

    let paths_robot_3 = iterate_directional_keypad(paths_robot_2);
    let paths_me = iterate_directional_keypad(paths_robot_3);

    paths_me[0].len()
}

// Determine the buttons to press on the given keypad to enter the target code
fn paths_for_code(mut code: Vec<char>, keypad: &Keypad) -> Vec<Vec<char>> {
    // prepend A as start position
    code.insert(0, 'A');
    let mut paths: Vec<Vec<char>> = Vec::new();

    for w in code.windows(2) {
        let new_paths: Vec<Vec<char>> = keypad_paths(w[0], w[1], keypad);
        if paths.is_empty() {
            paths = new_paths;
        } else {
            let mut combined_path: Vec<Vec<char>> = Vec::new();
            for prev_p in paths {
                for new_p in new_paths.clone() {
                    let combined = prev_p
                        .clone()
                        .into_iter()
                        .chain(new_p.into_iter())
                        .collect();
                    combined_path.push(combined);
                }
            }
            paths = combined_path;
        }
    }
    paths
}

fn complexity(code: &str, nr_button_presses: usize) -> usize {
    let numeric_part = code[..code.len() - 1]
        .parse::<usize>()
        .expect("Could not extract numeric part from code");
    numeric_part * nr_button_presses
}

// Find the fewest number of button presses you'll need to perform
// in order to cause the robot in front of the door to type each code.
// What is the sum of the complexities of the five codes on your list?
fn part_1(lines: Lines) -> usize {
    lines
        .map(|code| complexity(code, nr_of_button_presses(code)))
        .sum()
}

fn part_2(lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_21");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_numeric_keypad_steps() {
        let numeric_keypad: Keypad = HashMap::from(NUMERIC_KEYPAD_ENTRIES);

        assert_eq!(
            keypad_paths('A', '0', &numeric_keypad),
            vec![vec!['<', 'A']]
        );
        assert_eq!(
            keypad_paths('0', '2', &numeric_keypad),
            vec![vec!['^', 'A']]
        );
        assert_eq!(
            keypad_paths('2', '9', &numeric_keypad),
            vec![
                vec!['>', '^', '^', 'A'],
                vec!['^', '>', '^', 'A'],
                vec!['^', '^', '>', 'A']
            ]
        );
        assert_eq!(
            keypad_paths('9', 'A', &numeric_keypad),
            vec![vec!['v', 'v', 'v', 'A']]
        );
        assert_eq!(
            keypad_paths('A', '4', &numeric_keypad),
            vec![
                vec!['<', '^', '<', '^', 'A'],
                vec!['<', '^', '^', '<', 'A'],
                vec!['^', '<', '<', '^', 'A'],
                vec!['^', '<', '^', '<', 'A'],
                vec!['^', '^', '<', '<', 'A']
            ]
        );
        assert_eq!(
            keypad_paths('4', '5', &numeric_keypad),
            vec![vec!['>', 'A']]
        );
        assert_eq!(
            keypad_paths('5', '6', &numeric_keypad),
            vec![vec!['>', 'A']]
        );
        assert_eq!(
            keypad_paths('6', 'A', &numeric_keypad),
            vec![vec!['v', 'v', 'A']]
        );
    }

    #[test]
    fn test_directional_keypad_steps() {
        let directional_keypad: Keypad = HashMap::from(DIRECTIONAL_KEYPAD_ENTRIES);

        assert_eq!(
            keypad_paths('A', '<', &directional_keypad),
            vec![vec!['<', 'v', '<', 'A'], vec!['v', '<', '<', 'A']]
        );
        assert_eq!(
            keypad_paths('<', 'A', &directional_keypad),
            vec![vec!['>', '>', '^', 'A'], vec!['>', '^', '>', 'A']]
        );
        assert_eq!(
            keypad_paths('A', '^', &directional_keypad),
            vec![vec!['<', 'A']]
        );
        assert_eq!(keypad_paths('^', '^', &directional_keypad), vec![vec!['A']]);
        assert_eq!(keypad_paths('<', '<', &directional_keypad), vec![vec!['A']]);
        assert_eq!(keypad_paths('A', 'A', &directional_keypad), vec![vec!['A']]);
        assert_eq!(
            keypad_paths('^', 'A', &directional_keypad),
            vec![vec!['>', 'A']]
        );
    }

    #[test]
    fn test_paths_for_numeric_code() {
        let numeric_keypad: Keypad = HashMap::from(NUMERIC_KEYPAD_ENTRIES);

        assert_eq!(
            paths_for_code("029A".chars().collect(), &numeric_keypad),
            vec![
                vec!['<', 'A', '^', 'A', '>', '^', '^', 'A', 'v', 'v', 'v', 'A'],
                vec!['<', 'A', '^', 'A', '^', '>', '^', 'A', 'v', 'v', 'v', 'A'],
                vec!['<', 'A', '^', 'A', '^', '^', '>', 'A', 'v', 'v', 'v', 'A']
            ]
        );
        assert_eq!(
            paths_for_code("379A".chars().collect(), &numeric_keypad),
            vec![
                vec!['^', 'A', '<', '<', '^', '^', 'A', '>', '>', 'A', 'v', 'v', 'v', 'A'],
                vec!['^', 'A', '<', '^', '<', '^', 'A', '>', '>', 'A', 'v', 'v', 'v', 'A'],
                vec!['^', 'A', '<', '^', '^', '<', 'A', '>', '>', 'A', 'v', 'v', 'v', 'A'],
                vec!['^', 'A', '^', '<', '<', '^', 'A', '>', '>', 'A', 'v', 'v', 'v', 'A'],
                vec!['^', 'A', '^', '<', '^', '<', 'A', '>', '>', 'A', 'v', 'v', 'v', 'A'],
                vec!['^', 'A', '^', '^', '<', '<', 'A', '>', '>', 'A', 'v', 'v', 'v', 'A']
            ]
        );
    }

    #[test]
    fn test_paths_for_directional_code() {
        let directional_keypad: Keypad = HashMap::from(DIRECTIONAL_KEYPAD_ENTRIES);

        assert_eq!(
            paths_for_code("<A^A>^^AvvvA".chars().collect(), &directional_keypad),
            vec![
                vec![
                    '<', 'v', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    '<', 'v', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    '<', 'v', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    '<', 'v', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    '<', 'v', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    '<', 'v', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    '<', 'v', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    '<', 'v', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    '<', 'v', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    '<', 'v', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    '<', 'v', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    '<', 'v', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    '<', 'v', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    '<', 'v', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    '<', 'v', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    '<', 'v', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '>', '^', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '<', '^',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', '<', 'v', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    'v', '<', '<', 'A', '>', '^', '>', 'A', '<', 'A', '>', 'A', 'v', 'A', '^', '<',
                    'A', 'A', '>', 'A', 'v', '<', 'A', 'A', 'A', '^', '>', 'A'
                ]
            ]
        );
        assert_eq!(
            paths_for_code(
                "v<<A>>^A<A>AvA<^AA>A<vAAA>^A".chars().collect(),
                &directional_keypad
            )
            .len(),
            2048 // that's a lot of possible paths
        );
        assert_eq!(
            paths_for_code("^A^^<<A>>AvvvA".chars().collect(), &directional_keypad),
            vec![
                vec![
                    '<', 'A', '>', 'A', '<', 'A', 'A', 'v', '<', 'A', 'A', '>', '>', '^', 'A', 'v',
                    'A', 'A', '^', 'A', '<', 'v', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    '<', 'A', '>', 'A', '<', 'A', 'A', 'v', '<', 'A', 'A', '>', '>', '^', 'A', 'v',
                    'A', 'A', '^', 'A', '<', 'v', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    '<', 'A', '>', 'A', '<', 'A', 'A', 'v', '<', 'A', 'A', '>', '>', '^', 'A', 'v',
                    'A', 'A', '^', 'A', 'v', '<', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    '<', 'A', '>', 'A', '<', 'A', 'A', 'v', '<', 'A', 'A', '>', '>', '^', 'A', 'v',
                    'A', 'A', '^', 'A', 'v', '<', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    '<', 'A', '>', 'A', '<', 'A', 'A', 'v', '<', 'A', 'A', '>', '^', '>', 'A', 'v',
                    'A', 'A', '^', 'A', '<', 'v', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    '<', 'A', '>', 'A', '<', 'A', 'A', 'v', '<', 'A', 'A', '>', '^', '>', 'A', 'v',
                    'A', 'A', '^', 'A', '<', 'v', 'A', 'A', 'A', '^', '>', 'A'
                ],
                vec![
                    '<', 'A', '>', 'A', '<', 'A', 'A', 'v', '<', 'A', 'A', '>', '^', '>', 'A', 'v',
                    'A', 'A', '^', 'A', 'v', '<', 'A', 'A', 'A', '>', '^', 'A'
                ],
                vec![
                    '<', 'A', '>', 'A', '<', 'A', 'A', 'v', '<', 'A', 'A', '>', '^', '>', 'A', 'v',
                    'A', 'A', '^', 'A', 'v', '<', 'A', 'A', 'A', '^', '>', 'A'
                ]
            ]
        );
    }

    #[test]
    fn test_complexity() {
        assert_eq!(complexity("029A", 68), 68 * 29);
        assert_eq!(complexity("980A", 60), 60 * 980);
        assert_eq!(complexity("179A", 68), 68 * 179);
        assert_eq!(complexity("456A", 82), 64 * 456);
        assert_eq!(complexity("379A", 82), 64 * 379);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 126384);
    }

        #[test]
        fn test_part_1() {
            assert_eq!(part_1(load_input("inputs/2024/day_21").lines()), 177814);
        }

    //     #[test]
    //     fn test_part_2_example() {
    //         assert_eq!(part_2(EXAMPLE_INPUT.lines()), 0);
    //     }

    //     #[test]
    //     fn test_part_2() {
    //         assert_eq!(part_2(load_input("inputs/2024/day_21").lines()), 0)
    //     }
}
