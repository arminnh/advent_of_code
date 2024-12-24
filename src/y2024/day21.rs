use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;

// The numeric keypad has four rows of buttons: 789, 456, 123, and finally an empty gap followed by 0A.
const NUMERIC_KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];
// The directional keypad has two rows of buttons: a gap / ^ (up) / A (activate) on the first row
// and < (left) / v (down) / > (right) on the second row
const DIRECTIONAL_KEYPAD: [[char; 3]; 4] = [
    ['-', '^', 'A'],
    ['<', 'v', '>'],
    ['*', '*', '*'],
    ['*', '*', '*'],
];

// Return the movements needed between two buttons on the numeric keypad
fn keypad_steps(from: char, to: char, keypad: &[[char; 3]; 4]) -> String {
    let from_row = keypad
        .iter()
        .position(|row| row.contains(&from))
        .expect("Invalid 'from' char");
    let from_col = keypad[from_row]
        .iter()
        .position(|col| col == &from)
        .unwrap();
    let to_row = keypad
        .iter()
        .position(|row| row.contains(&to))
        .expect("Invalid 'to' char");
    let to_col = keypad[to_row].iter().position(|col| col == &to).unwrap();

    let row_steps = if from_row < to_row {
        "v".repeat(to_row - from_row)
    } else {
        "^".repeat(from_row - to_row)
    };

    let col_steps = if from_col < to_col {
        ">".repeat(to_col - from_col)
    } else {
        "<".repeat(from_col - to_col)
    };

    row_steps + &col_steps + "A" // plus A to press the target button
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
fn determine_button_presses(code: &str) -> String {
    let directional_code_robot_2 = steps_for_code(code, &NUMERIC_KEYPAD);
    let directional_code_robot_3 = steps_for_code(&directional_code_robot_2, &DIRECTIONAL_KEYPAD);
    steps_for_code(&directional_code_robot_3, &DIRECTIONAL_KEYPAD)
}

// Determine the buttons to press on the given keypad to enter the target code
fn steps_for_code(code: &str, keypad: &[[char; 3]; 4]) -> String {
    // prepend A as start position
    let full_code: Vec<char> = ("A".to_string() + code).chars().collect();
    full_code
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, ch)| keypad_steps(full_code[i - 1], *ch, keypad))
        .collect::<Vec<String>>()
        .join("")
}

fn complexity(code: &str, button_presses: String) -> usize {
    let numeric_part = code[..code.len() - 1]
        .parse::<usize>()
        .expect("Could not extract numeric part from code");
    numeric_part * button_presses.len()
}

// Find the fewest number of button presses you'll need to perform
// in order to cause the robot in front of the door to type each code.
// What is the sum of the complexities of the five codes on your list?
fn part_1(lines: Lines) -> usize {
    lines
        .map(|code| (code, determine_button_presses(code)))
        .map(|(code, button_presses)| complexity(code, button_presses))
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
        assert_eq!(keypad_steps('A', '0', &NUMERIC_KEYPAD), "<A".to_string());
        assert_eq!(keypad_steps('0', '2', &NUMERIC_KEYPAD), "^A".to_string());
        assert_eq!(keypad_steps('2', '9', &NUMERIC_KEYPAD), "^^>A".to_string());
        assert_eq!(keypad_steps('9', 'A', &NUMERIC_KEYPAD), "vvvA".to_string());
        assert_eq!(keypad_steps('A', '4', &NUMERIC_KEYPAD), "^^<<A".to_string());
        assert_eq!(keypad_steps('4', '5', &NUMERIC_KEYPAD), ">A".to_string());
        assert_eq!(keypad_steps('5', '6', &NUMERIC_KEYPAD), ">A".to_string());
        assert_eq!(keypad_steps('6', 'A', &NUMERIC_KEYPAD), "vvA".to_string());
    }

    #[test]
    fn test_directional_keypad_steps() {
        assert_eq!(
            keypad_steps('A', '<', &DIRECTIONAL_KEYPAD),
            "v<<A".to_string()
        );
        assert_eq!(
            keypad_steps('<', 'A', &DIRECTIONAL_KEYPAD),
            "^>>A".to_string()
        );
        assert_eq!(
            keypad_steps('A', '^', &DIRECTIONAL_KEYPAD),
            "<A".to_string()
        );
        assert_eq!(keypad_steps('^', '^', &DIRECTIONAL_KEYPAD), "A".to_string());
        assert_eq!(keypad_steps('<', '<', &DIRECTIONAL_KEYPAD), "A".to_string());
        assert_eq!(keypad_steps('A', 'A', &DIRECTIONAL_KEYPAD), "A".to_string());
        assert_eq!(
            keypad_steps('^', 'A', &DIRECTIONAL_KEYPAD),
            ">A".to_string()
        );
    }

    #[test]
    fn test_numeric_code_to_directional_code() {
        assert_eq!(
            steps_for_code("029A", &NUMERIC_KEYPAD),
            "<A^A^^>AvvvA".to_string()
        );
        assert_eq!(
            steps_for_code("379A", &NUMERIC_KEYPAD),
            "^A^^<<A>>AvvvA".to_string()
        );
    }

    #[test]
    fn test_directional_code_to_directional_code() {
        assert_eq!(
            steps_for_code("<A^A>^^AvvvA", &DIRECTIONAL_KEYPAD),
            "v<<A^>>A<A>AvA^<AA>Av<AAA^>A".to_string()
        );
        assert_eq!(
            steps_for_code("v<<A>>^A<A>AvA<^AA>A<vAAA>^A", &DIRECTIONAL_KEYPAD),
            "v<A<AA^>>AvAA^<A>Av<<A^>>AvA^Av<A^>Av<<A^>A>AAvA^Av<<A>A^>AAAvA^<A>A".to_string()
        );
        assert_eq!(
            steps_for_code("^A^^<<A>>AvvvA", &DIRECTIONAL_KEYPAD),
            "<A>A<AAv<AA^>>AvAA^Av<AAA^>A".to_string()
        );
        assert_eq!(
            steps_for_code("<A>A<AAv<AA^>>AvAA^Av<AAA^>A", &DIRECTIONAL_KEYPAD),
            "v<<A^>>AvA^Av<<A^>>AAv<A<A^>>AA<Av>AA^Av<A^>AA<A>Av<A<A^>>AAA<Av>A^A".to_string()
        );
    }

    #[test]
    fn test_complexity() {
        assert_eq!(
            complexity(
                "029A",
                "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".to_string()
            ),
            68 * 29
        );
        assert_eq!(
            complexity(
                "980A",
                "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".to_string()
            ),
            60 * 980
        );
        assert_eq!(
            complexity(
                "179A",
                "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".to_string()
            ),
            68 * 179
        );
        assert_eq!(
            complexity(
                "456A",
                "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".to_string()
            ),
            64 * 456
        );
        assert_eq!(
            complexity(
                "379A",
                "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".to_string()
            ),
            64 * 379
        );
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 126384);
    }

    #[test]
    fn test_part_1() {
        // 184058 too high
        assert_eq!(part_1(load_input("inputs/2024/day_21").lines()), 0);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_21").lines()), 0)
    }
}
