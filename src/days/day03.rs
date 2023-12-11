use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::fs;
use std::str::Lines;

fn load_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not open file.")
}

type Position = (i32, i32);

#[derive(Debug)]
struct Number {
    value: i32,
    position: Position,
    len: i32,
}

impl Number {
    fn touched_by_symbol(&self, symbols: &HashSet<Position>) -> bool {
        for x in self.position.0 - 1..=self.position.0 + 1 {
            for y in self.position.1 - 1..=self.position.1 + self.len {
                if symbols.contains(&(x, y)) {
                    return true;
                }
            }
        }

        false
    }
}

fn parse_input(lines: Lines) -> (Vec<Number>, HashSet<Position>) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: HashSet<Position> = HashSet::new();

    for (row, line) in lines.enumerate() {
        let mut current_value: Option<u32> = None;
        let mut current_start: Option<usize> = None;

        for (col, c) in line.char_indices() {
            if c.is_ascii_digit() {
                let digit = c.to_digit(10).unwrap();
                if let Some(v) = current_value {
                    current_value = Some(v * 10 + digit);
                } else {
                    current_value = Some(digit);
                    current_start = Some(col);
                }
            } else if c != '.' {
                symbols.insert((row as i32, col as i32));
            }

            if !c.is_ascii_digit() || col == line.len() - 1 {
                match (current_value, current_start) {
                    (Some(v), Some(start_y)) => {
                        numbers.push(Number {
                            value: v as i32,
                            position: (row as i32, start_y as i32),
                            len: (col - start_y) as i32,
                        });
                        current_value = None;
                        current_start = None;
                    }
                    _ => (),
                }
            }
        }
    }

    (numbers, symbols)
}

fn part_1(lines: Lines) -> i32 {
    let (numbers, symbols) = parse_input(lines);
    // dbg!(&numbers);
    // dbg!(&symbols);

    numbers
        .iter()
        .filter_map(|num| {
            if num.touched_by_symbol(&symbols) {
                Some(num.value)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(_lines: Lines) -> i32 {
    0
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_1(load_input("inputs/day_3").lines())),
        Solution::from(part_2(load_input("inputs/day_3").lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    const EXAMPLE_INPUT_2: &str = "....11
......
....22
33+...
......
44+.44
......
+55.55
.....+";

    const EXAMPLE_INPUT_3: &str = ".......5......
..7*..*.....4*
...*13*......9
.......15.....
..............
..............
..............
..............
..............
..............
21............
...*9.........";

    const EXAMPLE_INPUT_4: &str = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56";

    #[test]
    fn test_part_1_example_1() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 4361);
    }

    #[test]
    fn test_part_1_example_2() {
        assert_eq!(part_1(EXAMPLE_INPUT_2.lines()), 187);
    }

    #[test]
    fn test_part_1_example_3() {
        assert_eq!(part_1(EXAMPLE_INPUT_3.lines()), 62);
    }

    #[test]
    fn test_part_1_example_4() {
        assert_eq!(part_1(EXAMPLE_INPUT_4.lines()), 925);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_3").lines()), 560670);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_3").lines()), 0);
    }
}
