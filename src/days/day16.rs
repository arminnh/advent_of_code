use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::Lines;
use std::time::Duration;
use std::{thread, usize};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn print_grid(grid: &Vec<u8>, width: usize, energized: &HashSet<&usize>, current_beam: usize) {
    for row in 0..grid.len() / width {
        for col in 0..width {
            let index = row * width + col;
            if index == current_beam {
                print!("@");
            } else if grid[index] == b'.' && energized.contains(&index) {
                print!("#");
            } else {
                print!("{}", grid[index] as char);
            }
        }
        println!("");
    }
}

// Determine the next directions for a beam on the current cell in the grid. Beams can split in two directions.
fn next_directions(cell: u8, beam_direction: Direction) -> (Direction, Option<Direction>) {
    match (cell, beam_direction) {
        (b'|', Direction::Left | Direction::Right) => (Direction::Up, Some(Direction::Down)),
        (b'-', Direction::Up | Direction::Down) => (Direction::Left, Some(Direction::Right)),
        (b'\\', Direction::Up) | (b'/', Direction::Down) => (Direction::Left, None),
        (b'\\', Direction::Down) | (b'/', Direction::Up) => (Direction::Right, None),
        (b'\\', Direction::Left) | (b'/', Direction::Right) => (Direction::Up, None),
        (b'\\', Direction::Right) | (b'/', Direction::Left) => (Direction::Down, None),
        _ => (beam_direction, None),
    }
}

// Advance the beam in the given direction as long as it stays on the grid.
fn advance_beam(
    i: usize,
    direction: Direction,
    width: usize,
    grid_len: usize,
) -> Option<(usize, Direction)> {
    match direction {
        Direction::Up if i >= width => Some((i - width, Direction::Up)),
        Direction::Down if i < grid_len - width => Some((i + width, Direction::Down)),
        Direction::Left if i % width > 0 => Some((i - 1, Direction::Left)),
        Direction::Right if i % width < width - 1 => Some((i + 1, Direction::Right)),
        _ => None,
    }
}

// Send a beam through the grid and count how many tiles end up being energized.
fn part_1(lines: Lines) -> usize {
    let mut lines = lines.peekable();
    let width = lines.peek().unwrap().len();
    let grid: Vec<u8> = lines.flat_map(|line| line.as_bytes().to_vec()).collect();
    let mut beams: VecDeque<(usize, Direction)> = VecDeque::from([(0, Direction::Right)]);
    let mut visited: HashMap<usize, HashSet<Direction>> = HashMap::new();

    while let Some((i, direction)) = beams.pop_front() {
        // print!("\x1B[2J\x1b[1;1H");
        // println!("Beam at {:?} going {:?}", (i / width, i % width), direction);
        let visited_directions = visited.entry(i).or_default();
        visited_directions.insert(direction);
        // print_grid(&grid, width, &visited.keys().collect(), i);
        // thread::sleep(Duration::from_millis(33));

        let (dir_1, dir_2) = next_directions(grid[i], direction);

        if let Some(next_pos) = advance_beam(i, dir_1, width, grid.len()) {
            if !visited.contains_key(&next_pos.0)
                || !visited.get(&next_pos.0).unwrap().contains(&next_pos.1)
            {
                beams.push_back(next_pos)
            }
        }
        if let Some(d) = dir_2 {
            if let Some(next_pos) = advance_beam(i, d, width, grid.len()) {
                if !visited.contains_key(&next_pos.0)
                    || !visited.get(&next_pos.0).unwrap().contains(&next_pos.1)
                {
                    beams.push_back(next_pos)
                }
            }
        }
    }

    visited.len()
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_16");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 46);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_16").lines()), 7199);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_16").lines()), 0);
    }
}
