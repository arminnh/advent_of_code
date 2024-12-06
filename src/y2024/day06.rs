use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::str::Lines;
use std::usize;

type Position = (i32, i32);

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn offsets(&self) -> (i32, i32) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Direction::North => "^",
            Direction::East => ">",
            Direction::South => "V",
            Direction::West => "<",
        };
        write!(f, "{}", out)
    }
}

fn move_pos(pos: Position, direction: &Direction) -> Position {
    let (offset_x, offset_y) = direction.offsets();
    (pos.0 + offset_x, pos.1 + offset_y)
}

fn simulate_steps(
    pos: Position,
    direction: Direction,
    obstacles: &HashSet<Position>,
    visited: &mut HashSet<Position>,
    max_x: i32,
    max_y: i32,
) -> Option<Position> {
    visited.insert(pos);
    print_grid(obstacles, visited, pos, &direction, max_x, max_y);
    let next_pos = move_pos(pos, &direction);

    // if next position is not in bounds, guard leaves the area
    let in_bounds = pos.0 >= 0 && pos.0 < max_x && pos.1 >= 0 && pos.1 < max_y;
    if !in_bounds {
        None
    } else if obstacles.contains(&next_pos) {
        let next_dir = direction.turn_right();
        simulate_steps(pos, next_dir, obstacles, visited, max_x, max_y)
    } else {
        simulate_steps(next_pos, direction, obstacles, visited, max_x, max_y)
    }
}

fn print_grid(
    obstacles: &HashSet<Position>,
    visited: &HashSet<Position>,
    pos: Position,
    dir: &Direction,
    max_x: i32,
    max_y: i32,
) {
    for x in 0..max_x {
        for y in 0..max_y {
            if obstacles.contains(&(x, y)) {
                print!("#");
            } else if visited.contains(&(x, y)) {
                print!("█");
            } else if x == pos.0 && y == pos.1 {
                print!("{}", dir)
            } else {
                print!(".")
            }
        }
        println!("");
    }
    println!("\n");
}

// Predict the path of the guard. How many distinct positions will the guard visit before leaving the mapped area?
fn part_1(lines: Lines) -> usize {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut obstacles: HashSet<Position> = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut start_pos = (0, 0);
    for (x, line) in lines.enumerate() {
        for (y, ch) in line.chars().enumerate() {
            if ch == '#' {
                obstacles.insert((x as i32, y as i32));
            } else if ch == '^' {
                start_pos = (x as i32, y as i32);
            }
            max_y = max_y.max((y + 1) as i32);
        }
        max_x = max_x.max((x + 1) as i32);
    }

    simulate_steps(
        start_pos,
        Direction::North,
        &obstacles,
        &mut visited,
        max_x,
        max_y,
    );
    visited.len() - 1
}

fn part_2(lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_6");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 41);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_6").lines()), 4758);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT.to_string()), 123);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(load_input("inputs/2024/day_6").lines()), 5466);
    // }
}
