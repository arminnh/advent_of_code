use std::collections::HashSet;
use std::fs;
use std::str::Lines;

fn move_head(head: (i32, i32), direction: &str) -> (i32, i32) {
    match direction {
        "U" => (head.0 + 1, head.1),
        "D" => (head.0 - 1, head.1),
        "L" => (head.0, head.1 - 1),
        "R" => (head.0, head.1 + 1),
        _ => panic!("Unsupported direction str."),
    }
}

/// If the head is ever two steps directly up, down, left, or right from the tail,
/// the tail must also move one step in that direction so it remains close enough.
/// Otherwise, if the head and tail aren't touching and aren't in the same row or column,
/// the tail always moves one step diagonally to keep up:
fn move_tail(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32) {
    if head == tail
        || head.0 == tail.0 && (head.1 - tail.1).abs() < 2
        || head.1 == tail.1 && (head.0 - tail.0).abs() < 2
        || (head.0 - tail.0).abs() == 1 && (head.1 - tail.1).abs() == 1
    {
        *tail
    } else if head.0 == tail.0 {
        let offset = if head.1 > tail.1 { 1 } else { -1 };
        (tail.0, tail.1 + offset)
    } else if head.1 == tail.1 {
        let offset = if head.0 > tail.0 { 1 } else { -1 };
        (tail.0 + offset, tail.1)
    } else {
        (
            tail.0 + if head.0 > tail.0 { 1 } else { -1 },
            tail.1 + if head.1 > tail.1 { 1 } else { -1 },
        )
    }
}

/// Simulate your complete hypothetical series of motions.
/// How many positions does the tail of the rope visit at least once?
fn part_1(lines: Lines) -> usize {
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    lines.for_each(
        |line| match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [direction, amount] => {
                for _ in 0..amount.parse().unwrap() {
                    head = move_head(head, direction);
                    tail = move_tail(&head, &tail);
                    visited.insert(tail);
                    // println!("{:?}, {:?}", head, tail);
                }
            }
            _ => panic!("Unsupported input: {:?}", line),
        },
    );

    visited.len()
}

/// Simulate your complete series of motions on a larger rope with ten knots.
/// How many positions does the tail of the rope visit at least once?
fn part_2(lines: Lines) -> usize {
    let mut snake: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    lines.for_each(
        |line| match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [direction, amount] => {
                for _ in 0..amount.parse().unwrap() {
                    snake[0] = move_head(snake[0], direction);
                    for i in 1..snake.len() {
                        snake[i] = move_tail(&snake[i - 1], &snake[i]);
                    }
                    visited.insert(snake[snake.len() - 1]);
                }
            }
            _ => panic!("Unsupported input: {:?}", line),
        },
    );

    visited.len()
}

/// Day 9: Rope Bridge
fn main() {
    if let Ok(contents) = fs::read_to_string("inputs/day_9") {
        println!("{}", part_1(contents.lines()));
        println!("{}", part_2(contents.lines()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT.lines()), 13)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT.lines()), 1)
    }

    #[test]
    fn test_part_2_larger() {
        let input = "R 5
    U 8
    L 8
    D 3
    R 17
    D 10
    L 25
    U 20";
        assert_eq!(part_2(input.lines()), 36)
    }
}
