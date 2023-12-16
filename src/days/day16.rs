use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::str::Lines;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// fn print_grid(grid: &Vec<u8>, width: usize, energized: &HashSet<&usize>, current_beam: usize) {
//     for row in 0..grid.len() / width {
//         for col in 0..width {
//             let index = row * width + col;
//             if index == current_beam {
//                 print!("@");
//             } else if grid[index] == b'.' && energized.contains(&index) {
//                 print!("#");
//             } else {
//                 print!("{}", grid[index] as char);
//             }
//         }
//         println!("");
//     }
// }

// Determine the next directions for a beam on the current cell in the grid. Beams can split in two directions.
fn next_directions(cell: u8, beam_direction: Direction) -> [Option<Direction>; 2] {
    use Direction::*;
    match (cell, beam_direction) {
        (b'|', Left | Right) => [Some(Up), Some(Down)],
        (b'-', Up | Down) => [Some(Left), Some(Right)],
        (b'\\', Up) | (b'/', Down) => [Some(Left), None],
        (b'\\', Down) | (b'/', Up) => [Some(Right), None],
        (b'\\', Left) | (b'/', Right) => [Some(Up), None],
        (b'\\', Right) | (b'/', Left) => [Some(Down), None],
        _ => [Some(beam_direction), None],
    }
}

// Advance the beam in the given direction as long as it stays on the grid.
fn advance_beam(
    i: usize,
    direction: Direction,
    width: usize,
    grid_len: usize,
) -> Option<(usize, Direction)> {
    use Direction::*;
    match direction {
        Up if i >= width => Some((i - width, Up)),
        Down if i < grid_len - width => Some((i + width, Down)),
        Left if i % width > 0 => Some((i - 1, Left)),
        Right if i % width < width - 1 => Some((i + 1, Right)),
        _ => None,
    }
}

fn count_energized_tiles(grid: &Vec<u8>, width: usize, initial_beam: (usize, Direction)) -> usize {
    let mut beams: Vec<(usize, Direction)> = Vec::from([initial_beam]);
    let mut visited: HashSet<(usize, Direction)> = HashSet::new();
    // let mut iters = 0;

    while let Some(beam) = beams.pop() {
        visited.insert(beam);
        // if iters % 20 == 0 {
        //     print!("\x1B[2J\x1b[1;1H");
        //     println!("Beam at {:?} going {:?}", (beam.0 / width, beam.0 % width), beam.1);
        //     print_grid(&grid, width, &visited.iter().map(|(i, _)| i).collect(), beam.0);
        //     thread::sleep(Duration::from_millis(100));
        // }
        // iters += 1;

        next_directions(grid[beam.0], beam.1).iter().for_each(|d| {
            if let Some(new_direction) = d {
                if let Some(next_beam) = advance_beam(beam.0, *new_direction, width, grid.len()) {
                    if !visited.contains(&next_beam) {
                        beams.push(next_beam)
                    }
                }
            }
        })
    }

    let energized: HashSet<&usize> = visited.iter().map(|(i, _)| i).collect();
    energized.len()
}

// Send a beam through the grid and count how many tiles end up being energized.
fn part_1(lines: Lines) -> usize {
    let mut lines = lines.peekable();
    let width = lines.peek().unwrap().len();
    let grid: Vec<u8> = lines.flat_map(|line| line.as_bytes().to_vec()).collect();
    count_energized_tiles(&grid, width, (0, Direction::Right))
}

// Find the beam on the edges of the grid that energizes the largest number of tiles and return that number.
fn part_2(lines: Lines) -> usize {
    let mut lines = lines.peekable();
    let width = lines.peek().unwrap().len();
    let grid: Vec<u8> = lines.flat_map(|line| line.as_bytes().to_vec()).collect();
    let mut initial_beams: Vec<(usize, Direction)> = Vec::new();

    // Left and right edges
    (0..grid.len() / width).into_iter().for_each(|row| {
        initial_beams.push((row * width, Direction::Right));
        initial_beams.push((row * width + width - 1, Direction::Left));
    });
    // Top and bottom edges
    (0..width).into_iter().for_each(|col| {
        initial_beams.push((col, Direction::Down));
        initial_beams.push((grid.len() - 1 - col, Direction::Up));
    });

    initial_beams
        .into_iter()
        .map(|beam| count_energized_tiles(&grid, width, beam))
        .max()
        .unwrap()
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
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 51);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_16").lines()), 7438);
    }
}
