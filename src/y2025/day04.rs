use std::collections::HashSet;

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
    let mut neighbors: Vec<Position> = Vec::with_capacity(4);
    for next_pos in [
        (pos.0 + 1, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0 + 1, pos.1 + 1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0 - 1, pos.1 + 1),
    ] {
        if paper.contains(&next_pos) {
            neighbors.push(next_pos);
        }
    }

    neighbors
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

// ...
pub fn part_2(input: &str) -> usize {
    0
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

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT_1), 0);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(&load_input("inputs/2025/day_4")), 0);
    // }
}
