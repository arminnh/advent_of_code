use std::collections::HashSet;
use std::usize;

type Position = (usize, usize);

#[derive(Clone)]
struct Grid {
    // Separate lists of obstacles in horizontal/vertical directions to move in jumps instead of step by step
    obstacles_horizontal: Vec<Vec<usize>>,
    obstacles_vertical: Vec<Vec<usize>>,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    fn from(obstacles: Vec<Position>, max_x: usize, max_y: usize) -> Grid {
        let mut obstacles_horizontal: Vec<Vec<usize>> =
            std::iter::repeat(Vec::with_capacity(max_y))
                .take(max_x)
                .collect();
        let mut obstacles_vertical: Vec<Vec<usize>> = std::iter::repeat(Vec::with_capacity(max_x))
            .take(max_y)
            .collect();
        for &(x, y) in obstacles.iter() {
            obstacles_horizontal.get_mut(x).unwrap().push(y);
            obstacles_vertical.get_mut(y).unwrap().push(x);
        }
        Grid {
            obstacles_horizontal,
            obstacles_vertical,
            max_x,
            max_y,
        }
    }

    // Instead of moving step by step, jump to the next obstacle that will eventually be reached
    fn next_position(&self, (x, y): Position, direction: Direction) -> Result<Position, Position> {
        match direction {
            Direction::North => self
                .obstacles_vertical
                .get(y)
                .unwrap()
                .iter()
                .rev()
                .find(|&&obs_x| obs_x < x)
                .map(|new_x| Ok((new_x + 1, y)))
                .unwrap_or(Err((0, y))),
            Direction::East => self
                .obstacles_horizontal
                .get(x)
                .unwrap()
                .iter()
                .find(|&&obs_y| obs_y > y)
                .map(|new_y| Ok((x, new_y - 1)))
                .unwrap_or(Err((x, self.max_y - 1))),
            Direction::South => self
                .obstacles_vertical
                .get(y)
                .unwrap()
                .iter()
                .find(|&&obs_x| obs_x > x)
                .map(|new_x| Ok((new_x - 1, y)))
                .unwrap_or(Err((self.max_x - 1, y))),
            Direction::West => self
                .obstacles_horizontal
                .get(x)
                .unwrap()
                .iter()
                .rev()
                .find(|&&obs_y| obs_y < y)
                .map(|new_y| Ok((x, new_y + 1)))
                .unwrap_or(Err((x, 0))),
        }
    }

    fn insert_obstacle(&mut self, (x, y): Position) {
        let horizontal = self.obstacles_horizontal.get_mut(x).unwrap();
        if let Err(idx) = horizontal.binary_search_by(|&v| v.cmp(&y)) {
            horizontal.insert(idx, y);
        }

        let vertical = self.obstacles_vertical.get_mut(y).unwrap();
        if let Err(idx) = vertical.binary_search_by(|&v| v.cmp(&x)) {
            vertical.insert(idx, x);
        }
    }
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

fn parse_input(input: &str) -> (Grid, Position) {
    let mut obstacles: Vec<Position> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut position = (0, 0);
    for (x, line) in input.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            if ch == '#' {
                obstacles.push((x, y));
            } else if ch == '^' {
                position = (x, y);
            }
            max_y = max_y.max(y + 1);
        }
        max_x = max_x.max(x + 1);
    }
    (Grid::from(obstacles, max_x, max_y), position)
}

// fn print_grid(grid: &Grid, visited: &HashSet<Position>, position: Position, direction: Direction) {
//     for x in 0..grid.max_x {
//         for y in 0..grid.max_y {
//             if x == position.0 && y == position.1 {
//                 print!("{}", direction)
//             } else if grid.obstacles.contains(&(x, y)) {
//                 print!("#");
//             } else if visited.contains(&(x, y)) {
//                 print!("█");
//             } else {
//                 print!(".")
//             }
//         }
//         println!("");
//     }
//     println!("\n");
// }

fn walk_path(mut position: Position, mut direction: Direction, grid: &Grid) -> HashSet<Position> {
    let mut visited = HashSet::new();
    loop {
        let next_position_result = grid.next_position(position, direction);
        let next_position = match next_position_result {
            Ok(p) => p,
            Err(p) => p,
        };
        // Insert all positions between current and next
        mark_positions_as_visited(position, next_position, direction, &mut visited);

        if next_position_result.is_err() {
            // Reached edge of map, break out of loop
            break;
        }
        position = next_position;
        direction = direction.turn_right();
    }
    visited
}

fn mark_positions_as_visited(
    from: (usize, usize),
    to: (usize, usize),
    direction: Direction,
    visited: &mut HashSet<(usize, usize)>,
) {
    match direction {
        Direction::North | Direction::South => {
            for x in from.0.min(to.0)..=from.0.max(to.0) {
                visited.insert((x, from.1));
            }
        }
        Direction::West | Direction::East => {
            for y in from.1.min(to.1)..=from.1.max(to.1) {
                visited.insert((from.0, y));
            }
        }
    }
    // print_grid(grid, &visited, next_position, direction);
}

// Predict the path of the guard. How many distinct positions will the guard visit before leaving the mapped area?
pub fn part_1(input: &str) -> usize {
    let (grid, start_position) = parse_input(input);
    walk_path(start_position, Direction::North, &grid).len()
}

// Check if guard loops starting from the given position and direction
fn guard_loops(grid: &Grid, mut position: Position, mut direction: Direction) -> bool {
    // guard loops if it revisits a step in the same direction
    let mut seen: HashSet<(Position, Direction)> = HashSet::new();
    while seen.insert((position, direction)) {
        let next_position = grid.next_position(position, direction);
        if next_position.is_err() {
            // Reached edge of map, break out of loop
            return false;
        }
        position = next_position.unwrap();
        direction = direction.turn_right();
    }
    true
}

// In how many positions can you place an obstacle to get the guard stuck in a loop?
pub fn part_2(input: &str) -> usize {
    let (grid, start_position) = parse_input(input);
    let visited: HashSet<Position> = walk_path(start_position, Direction::North, &grid);

    // Instead of trying every possible position (16k), try only the path actually walked
    visited
        .into_iter()
        .filter(|position| {
            let mut new_grid = grid.clone();
            new_grid.insert_obstacle(*position);
            guard_loops(&new_grid, start_position, Direction::North)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::util::load_input;

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
        assert_eq!(part_1(EXAMPLE_INPUT), 41);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2024/day_6")), 4758);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT), 6);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2024/day_6")), 1670)
    }
}
