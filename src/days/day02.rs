use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs;
use std::str::{FromStr, Lines};
use std::usize;

fn load_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not open file.")
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct InvalidColorError;
impl FromStr for Color {
    type Err = InvalidColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(InvalidColorError),
        }
    }
}

fn parse_game_id(game: &str) -> usize {
    game.split_at(5).1.parse().unwrap()
}

fn parse_sets(sets: &str) -> Vec<HashMap<Color, i32>> {
    sets.split(";")
        .map(|set| {
            set.split(",")
                .map(
                    |cube| match cube.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
                        [amount, color] => (
                            Color::from_str(color).unwrap(),
                            amount.parse::<i32>().unwrap(),
                        ),
                        _ => panic!("Invalid cube '{:?}' in set '{:?}'", cube, set),
                    },
                )
                .collect()
        })
        .collect()
}

fn is_possible(sets: Vec<HashMap<Color, i32>>, max_counts: &HashMap<Color, i32>) -> bool {
    for set in sets {
        for (k, v) in set {
            if v > *max_counts.get(&k).unwrap() {
                return false;
            }
        }
    }

    true
}

fn part_1(lines: Lines) -> usize {
    let max_counts = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

    lines
        .map(|line| match line.split(":").collect::<Vec<&str>>()[..] {
            [game, sets] => (parse_game_id(game), parse_sets(sets)),
            _ => panic!("Unsupported input: {:?}", line),
        })
        .filter_map(|(game_id, sets)| {
            if is_possible(sets, &max_counts) {
                Some(game_id)
            } else {
                None
            }
        })
        .sum()
}
fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_1(load_input("inputs/day_2").lines())),
        Solution::from(part_2(load_input("inputs/day_2").lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_parse_game_id() {
        assert_eq!(parse_game_id("Game 0"), 0);
        assert_eq!(parse_game_id("Game 1"), 1);
        assert_eq!(parse_game_id("Game 99"), 99);
        assert_eq!(parse_game_id("Game 100"), 100);
    }

    #[test]
    fn test_parse_sets() {
        assert_eq!(
            parse_sets("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            vec![
                HashMap::from([(Color::Red, 4), (Color::Blue, 3)]),
                HashMap::from([(Color::Red, 1), (Color::Green, 2), (Color::Blue, 6)]),
                HashMap::from([(Color::Green, 2)])
            ]
        );
        assert_eq!(
            parse_sets("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"),
            vec![
                HashMap::from([(Color::Red, 3), (Color::Green, 1), (Color::Blue, 6)]),
                HashMap::from([(Color::Red, 6), (Color::Green, 3)]),
                HashMap::from([(Color::Red, 14), (Color::Green, 3), (Color::Blue, 15)])
            ]
        );
        assert_eq!(
            parse_sets("18 red, 11 green, 3 blue; 2 blue, 19 red, 7 green; 4 green, 1 blue, 6 red; 4 green, 2 red, 4 blue; 10 green, 5 red, 2 blue; 13 red, 12 green, 4 blue"),
            vec![
                HashMap::from([(Color::Red, 18), (Color::Green, 11), (Color::Blue, 3)]),
                HashMap::from([(Color::Red, 19), (Color::Green, 7), (Color::Blue, 2)]),
                HashMap::from([(Color::Red, 6), (Color::Green, 4), (Color::Blue, 1)]),
                HashMap::from([(Color::Red, 2), (Color::Green, 4), (Color::Blue, 4)]),
                HashMap::from([(Color::Red, 5), (Color::Green, 10), (Color::Blue, 2)]),
                HashMap::from([(Color::Red, 13), (Color::Green, 12), (Color::Blue, 4)])
            ]);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 8);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_2").lines()), 2600);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_2").lines()), 0);
    }
}
