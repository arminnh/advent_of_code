use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::str::Lines;

type Position = (i32, i32);
type Path = Vec<char>;
type Keypad = HashMap<Position, char>;
// Possible paths between two buttons
type PathsCache = HashMap<(char, char), Vec<Path>>;
// Amounts of total button presses between two buttons on layers of keypads
type CountsCache = HashMap<(char, char, usize), usize>;

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
fn keypad_paths(from: char, to: char, keypad: &Keypad) -> Vec<Path> {
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
    let mut paths: Vec<(Vec<Position>, Path)> = Vec::new();
    // Store all possible paths currently being explored
    let mut frontier: Vec<(Vec<Position>, Path)> = vec![(vec![start_pos], vec![])];
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

fn get_keypad_paths_cached(
    from: char,
    to: char,
    cache: &mut PathsCache,
    keypad: &Keypad,
) -> Vec<Path> {
    if let Some(paths) = cache.get(&(from, to)) {
        paths.clone()
    } else {
        let paths = keypad_paths(from, to, keypad);
        cache.insert((from, to), paths.clone());
        paths
    }
}

// For each code, return all possible shortest paths of buttons to press on the given keypad
// --- Slow because of allocations for the many possible paths -> see nr_of_button_presses_with_cache
fn generate_next_paths_for_codes(codes: Vec<Path>, keypad: &Keypad) -> Vec<Path> {
    let mut next_paths: Vec<Path> = Vec::new();

    for code in codes {
        // A is the start position
        let mut paths: Vec<Path> = keypad_paths('A', code[0], keypad);

        for w in code.windows(2) {
            // Find all possible paths between each pair of characters
            let new_paths = keypad_paths(w[0], w[1], keypad);

            // Combine each possible path so far with each new path found
            paths = paths
                .into_iter()
                .flat_map(|prev_path| {
                    new_paths.clone().into_iter().map(move |new_path| {
                        prev_path
                            .clone()
                            .into_iter()
                            .chain(new_path.into_iter())
                            .collect()
                    })
                })
                .collect();
        }
        next_paths.append(&mut paths);
    }

    next_paths
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
// --- Slow because of allocations for the many possible paths -> see nr_of_button_presses_with_cache
fn nr_of_button_presses(code: &str, nr_of_directional_keypads: usize) -> usize {
    let numeric_keypad: Keypad = HashMap::from(NUMERIC_KEYPAD_ENTRIES);
    let directional_keypad: Keypad = HashMap::from(DIRECTIONAL_KEYPAD_ENTRIES);

    // paths for robot 1 on the numeric keypad
    let mut paths = generate_next_paths_for_codes(vec![code.chars().collect()], &numeric_keypad);
    // paths for rest of the chain of keypads
    for _ in 0..nr_of_directional_keypads {
        paths = generate_next_paths_for_codes(paths, &directional_keypad);
        let shortest_path_len: usize = paths
            .iter()
            .min_by_key(|p| p.len())
            .expect("No path found!")
            .len();
        paths.retain(|p| p.len() == shortest_path_len);
    }
    paths[0].len()
}

// Recursively reduce the given code/path into the number of key presses on the final directional keypads
fn recurse_directional(
    from: char,
    to: char,
    layer: usize,
    paths_cache: &mut PathsCache,
    counts_cache: &mut CountsCache,
    keypad: &Keypad,
) -> usize {
    // println!("layer {}: {} to {}", layer, from, to);

    // <vA <A A >>^A vA A <^A
    //   v  < <    A  > >  ^A
    //             <        A
    //                      0

    // if let Some(cached) = counts_cache.get(&(from, to, layer)) {
        // println!("Cached {:?}: {:?}", (from, to, layer), cached);
    //     cached.clone()
    // } else
    if layer == 1 {
        let result = get_keypad_paths_cached(from, to, paths_cache, keypad)[0].len();
        // println!("layer {}: {} to {}. Result: {}", layer, from, to, result);
        result
    } else {
        // all possible paths between first two buttons in the code
        let paths: Vec<Path> = get_keypad_paths_cached(from, to, paths_cache, keypad);
        // println!("Computing {} to {} on layer {}. Paths: {:?}", from, to, layer, paths);
        // find the shortest possible path between the first two buttons on this layer
        let result = paths
            .into_iter()
            .map(|mut p| {
                p.insert(0, 'A');
                p.windows(2)
                    .map(|w| {
                        recurse_directional(
                            w[0],
                            w[1],
                            layer - 1,
                            paths_cache,
                            counts_cache,
                            keypad,
                        )
                    })
                    .sum()
            })
            .min()
            .unwrap();
        // println!("layer {}: {} to {}. Result: {}", layer, from, to, result);
        result
    }
}

// Instead of generating all possible paths for all layers, keep track of the number of key presses for each layer
// This time, do all calculations on each pair of characters in the given code, instead of iterating the full code
fn nr_of_button_presses_with_cache(
    code: &str,
    directional_keypad_layers: usize,
    paths_cache: &mut PathsCache,
    counts_cache: &mut CountsCache,
) -> usize {
    let numeric_keypad: Keypad = HashMap::from(NUMERIC_KEYPAD_ENTRIES);
    let directional_keypad: Keypad = HashMap::from(DIRECTIONAL_KEYPAD_ENTRIES);

    // First layer is on the numeric keypad -> generate initial paths
    let code: Vec<char> = code.chars().collect();
    let paths = generate_next_paths_for_codes(vec![code], &numeric_keypad);
    // println!("First paths: {:?}", paths);

    paths
        .into_iter()
        .map(|mut p| {
            p.insert(0, 'A');
            p.windows(2)
                .map(|w| {
                    recurse_directional(
                        w[0],
                        w[1],
                        directional_keypad_layers,
                        paths_cache,
                        counts_cache,
                        &directional_keypad,
                    )
                })
                .sum()
        })
        .min()
        .unwrap()
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
    let mut paths_cache: PathsCache = HashMap::new();
    let mut counts_cache: CountsCache = HashMap::new();

    lines
        .map(|code| {
            let nr = nr_of_button_presses_with_cache(code, 2, &mut paths_cache, &mut counts_cache);
            complexity(code, nr)
        })
        .sum()
}

// Part 1 but with 25 layers of directional keypad robots
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
            generate_next_paths_for_codes(vec!["029A".chars().collect()], &numeric_keypad),
            vec![
                vec!['<', 'A', '^', 'A', '>', '^', '^', 'A', 'v', 'v', 'v', 'A'],
                vec!['<', 'A', '^', 'A', '^', '>', '^', 'A', 'v', 'v', 'v', 'A'],
                vec!['<', 'A', '^', 'A', '^', '^', '>', 'A', 'v', 'v', 'v', 'A']
            ]
        );
        assert_eq!(
            generate_next_paths_for_codes(vec!["379A".chars().collect()], &numeric_keypad),
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
            generate_next_paths_for_codes(
                vec!["<A^A>^^AvvvA".chars().collect()],
                &directional_keypad,
            ),
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
            generate_next_paths_for_codes(
                vec!["v<<A>>^A<A>AvA<^AA>A<vAAA>^A".chars().collect()],
                &directional_keypad,
            )
            .len(),
            2048 // that's a lot of possible paths
        );
        assert_eq!(
            generate_next_paths_for_codes(
                vec!["^A^^<<A>>AvvvA".chars().collect()],
                &directional_keypad,
            ),
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
        assert_eq!(complexity("456A", 64), 64 * 456);
        assert_eq!(complexity("379A", 64), 64 * 379);
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
