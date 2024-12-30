use std::collections::HashSet;

type Position = (i32, i32);

#[derive(Debug)]
struct Number {
    value: i32,
    position: Position,
    len: i32,
}

impl Number {
    fn adjacent_to_any_symbol(&self, symbols: &HashSet<Position>) -> bool {
        for x in self.position.0 - 1..=self.position.0 + 1 {
            for y in self.position.1 - 1..=self.position.1 + self.len {
                if symbols.contains(&(x, y)) {
                    return true;
                }
            }
        }

        false
    }

    fn adjacent_to_symbol(&self, symbol_position: &Position) -> bool {
        for x in self.position.0 - 1..=self.position.0 + 1 {
            for y in self.position.1 - 1..=self.position.1 + self.len {
                if &(x, y) == symbol_position {
                    return true;
                }
            }
        }

        false
    }
}

fn parse_input(input: &str) -> (Vec<Number>, HashSet<Position>) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: HashSet<Position> = HashSet::new();

    for (row, line) in input.lines().enumerate() {
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

// What is the sum of all of the part numbers in the engine schematic?
pub fn part_1(input: &str) -> i32 {
    let (numbers, symbols) = parse_input(input);

    numbers
        .iter()
        .filter_map(|num| {
            if num.adjacent_to_any_symbol(&symbols) {
                Some(num.value)
            } else {
                None
            }
        })
        .sum()
}

fn gear_ratio(symbol: &Position, numbers: &Vec<Number>) -> Option<i32> {
    let adjacent_nums = numbers
        .iter()
        .filter(|num| num.adjacent_to_symbol(symbol))
        .collect::<Vec<_>>();

    if adjacent_nums.len() == 2 {
        Some(adjacent_nums[0].value * adjacent_nums[1].value)
    } else {
        None
    }
}

// What is the sum of all of the gear ratios in your engine schematic?
pub fn part_2(input: &str) -> i32 {
    let (numbers, symbols) = parse_input(input);

    symbols
        .iter()
        .filter_map(|symbol| gear_ratio(symbol, &numbers))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::util::load_input;

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
        assert_eq!(part_1(EXAMPLE_INPUT_1), 4361);
    }

    #[test]
    fn test_part_1_example_2() {
        assert_eq!(part_1(EXAMPLE_INPUT_2), 187);
    }

    #[test]
    fn test_part_1_example_3() {
        assert_eq!(part_1(EXAMPLE_INPUT_3), 62);
    }

    #[test]
    fn test_part_1_example_4() {
        assert_eq!(part_1(EXAMPLE_INPUT_4), 925);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2023/day_3")), 560670);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1), 467835);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2023/day_3")), 91622824);
    }
}
