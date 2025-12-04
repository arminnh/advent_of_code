use std::{collections::HashSet, hash::Hash};

type Position = (i32, i32);

// How many rolls of paper can be accessed?
// A roll can be accessed if there are fewer than four rolls of paper in the adjacent positions
pub fn part_1(input: &str) -> usize {
    let paper = parse_input(input);

    paper
        .iter()
        .filter(|&p| neighbors(*p, &paper).iter().count() < 4)
        .count()
}

fn neighbors(pos: Position, paper: &HashSet<Position>) -> Vec<Position> {
    [
        (pos.0 + 1, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0 + 1, pos.1 + 1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0 - 1, pos.1 + 1),
    ]
    .into_iter()
    .filter(|&p| paper.contains(&p))
    .collect()
}

fn parse_input(input: &str) -> HashSet<Position> {
    let mut paper = HashSet::new();

    for (x, line) in input.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            if ch == '@' {
                paper.insert((x as i32, y as i32));
            }
        }
    }

    paper
}

// Once a roll of paper can be accessed, it can be removed.
// How many rolls can be removed in total?
pub fn part_2(input: &str) -> usize {
    let mut paper = parse_input(input);
    let nr_of_rolls = paper.len();

    loop {
        let to_remove: HashSet<Position> = paper
            .iter()
            .filter(|&p| neighbors(*p, &paper).iter().count() < 4)
            .copied()
            .collect();

        if to_remove.is_empty() {
            break;
        } else {
            paper = paper.difference(&to_remove).copied().collect();
        }
    }

    nr_of_rolls - paper.len()
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

    use super::*;

    const EXAMPLE_INPUT_1: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 13);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_4")), 1518);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1), 43);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2025/day_4")), 8665);
    }
}
