use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::str::Lines;
use std::usize;

type Position = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
struct Guard {
    position: Position,
    direction: Direction,
}

impl Guard {
    fn in_bounds(&self, grid: &Grid) -> bool {
        self.position.0 >= 0
            && self.position.0 < grid.max_x
            && self.position.1 >= 0
            && self.position.1 < grid.max_y
    }

    fn next_position(&self) -> Position {
        let (offset_x, offset_y) = self.direction.offsets();
        (self.position.0 + offset_x, self.position.1 + offset_y)
    }
}

#[derive(Clone)]
struct Grid {
    obstacles: HashSet<Position>,
    max_x: i32,
    max_y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
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

fn parse_input(lines: Lines) -> (Grid, Guard) {
    let mut obstacles: HashSet<Position> = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut position = (0, 0);
    for (x, line) in lines.enumerate() {
        for (y, ch) in line.chars().enumerate() {
            if ch == '#' {
                obstacles.insert((x as i32, y as i32));
            } else if ch == '^' {
                position = (x as i32, y as i32);
            }
            max_y = max_y.max((y + 1) as i32);
        }
        max_x = max_x.max((x + 1) as i32);
    }

    (
        Grid {
            obstacles,
            max_x,
            max_y,
        },
        Guard {
            position,
            direction: Direction::North,
        },
    )
}

fn walk_path(mut guard: Guard, grid: &Grid) -> HashSet<Position> {
    let mut visited = HashSet::new();
    // if position is not in bounds, guard left the area
    while guard.in_bounds(&grid) {
        visited.insert(guard.position);

        if grid.obstacles.contains(&guard.next_position()) {
            guard.direction = guard.direction.turn_right();
        } else {
            guard.position = guard.next_position();
        }
    }
    visited
}

// Predict the path of the guard. How many distinct positions will the guard visit before leaving the mapped area?
fn part_1(lines: Lines) -> usize {
    let (grid, guard) = parse_input(lines);
    walk_path(guard, &grid).len()
}

// fn print_grid(grid: &Grid, visited: &HashSet<Guard>, guard: Guard) {
//     for x in 0..grid.max_x {
//         for y in 0..grid.max_y {
//             if x == guard.position.0 && y == guard.position.1 {
//                 print!("{}", guard.direction)
//             } else if grid.obstacles.contains(&(x, y)) {
//                 print!("#");
//             } else if visited
//                 .intersection(&HashSet::from([
//                     ((x, y), Direction::North),
//                     ((x, y), Direction::East),
//                     ((x, y), Direction::South),
//                     ((x, y), Direction::West),
//                 ]))
//                 .count()
//                 > 0
//             {
//                 print!("█");
//             } else {
//                 print!(".")
//             }
//         }
//         println!("");
//     }
//     println!("\n");
// }

/// Does the guard loop starting from the given position and direction?
fn guard_loops(grid: &Grid, mut guard: Guard) -> bool {
    // guard loops if it revisits a step in the same direction
    let mut seen: HashSet<Guard> = HashSet::new();
    while seen.insert(guard) {
        let next_position = guard.next_position();

        if grid.obstacles.contains(&next_position) {
            guard.direction = guard.direction.turn_right();
        } else {
            guard.position = guard.next_position();
            if !guard.in_bounds(grid) {
                return false;
            }
        }
    }
    true
}

// In how many positions can you place an obstacle to get the guard stuck in a loop?
fn part_2(lines: Lines) -> usize {
    let (grid, guard) = parse_input(lines);
    let visited: HashSet<Position> = walk_path(guard, &grid);

    // Instead of trying every possible position (16k), try only the path actually walked
    visited
        .into_iter()
        .filter(|position| {
            let mut new_grid = grid.clone();
            new_grid.obstacles.insert(*position);
            guard_loops(&new_grid, guard)
        })
        .count()
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

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), 6);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_6").lines()), 1670)
    }
}
