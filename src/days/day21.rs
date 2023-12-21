use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::str::Lines;
use std::usize;

type Position = (i32, i32);

#[allow(dead_code)]
fn print_grid(
    max_x: i32,
    max_y: i32,
    rocks: &HashSet<Position>,
    possible_positions: &HashSet<Position>,
) {
    for x in 0..max_x {
        for y in 0..max_y {
            if rocks.contains(&(x as i32, y as i32)) {
                print!("#");
            } else if possible_positions.contains(&(x as i32, y as i32)) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
    println!("");
}

fn rock_positions(grid: &Vec<Vec<u8>>) -> HashSet<Position> {
    grid.iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter().enumerate().filter_map(move |(y, &c)| {
                if c == b'#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn get_start(grid: &Vec<Vec<u8>>) -> Position {
    let x = grid.iter().position(|bytes| bytes.contains(&b'S')).unwrap();
    let y = grid[x].iter().position(|&b| b == b'S').unwrap();
    (x as i32, y as i32)
}

fn neighbors(p: &Position) -> [Position; 4] {
    [
        (p.0 + 1, p.1),
        (p.0 - 1, p.1),
        (p.0, p.1 + 1),
        (p.0, p.1 - 1),
    ]
}

fn normalize_coord(coord: i32, max: i32) -> i32 {
    if coord >= 0 {
        coord % max
    } else {
        (max - (-coord % max)) % max
    }
}

fn normalize_position(p: Position, max_x: i32, max_y: i32) -> Position {
    (normalize_coord(p.0, max_x), normalize_coord(p.1, max_y))
}

// Determine how many positions (avoiding rocks) can be reached in exactly N steps
fn nr_of_possible_positions(
    rocks: HashSet<(i32, i32)>,
    max_x: i32,
    max_y: i32,
    start: (i32, i32),
    iterations: usize,
) -> usize {
    let mut possible_positions_even: HashSet<Position> = HashSet::from([start]);
    let mut possible_positions_uneven: HashSet<Position> = HashSet::from([]);
    let mut last_iteration_positions: HashSet<Position> = HashSet::from([start]);

    for i in 0..iterations {
        for p in last_iteration_positions.drain().collect::<Vec<Position>>() {
            for n in neighbors(&p)
                .into_iter()
                // normalize the position to check for rocks on the infinitely repeating grid
                .filter(|&n| !rocks.contains(&normalize_position(n, max_x, max_y)))
            {
                if i % 2 == 0 {
                    if possible_positions_uneven.insert(n) {
                        last_iteration_positions.insert(n);
                    }
                } else {
                    if possible_positions_even.insert(n) {
                        last_iteration_positions.insert(n);
                    }
                }
            }
        }
        if i % 1000 == 0 {
            println!("Iteration {}", i);
        }
        // if i % 2 == 0 {
        //     print_grid(max_x, max_y, &rocks, &possible_positions_uneven);
        // } else {
        //     print_grid(max_x, max_y, &rocks, &possible_positions_even);
        // }
    }

    if iterations % 2 == 0 {
        possible_positions_even.len()
    } else {
        possible_positions_uneven.len()
    }
}

fn part_1(lines: Lines, iterations: usize) -> usize {
    let grid: Vec<Vec<u8>> = lines.map(|line| line.as_bytes().to_vec()).collect();
    let max_x = grid.len() as i32;
    let max_y = grid[0].len() as i32;
    let start = get_start(&grid);
    let rocks = rock_positions(&grid);

    nr_of_possible_positions(rocks, max_x, max_y, start, iterations)
}

fn part_2(lines: Lines, iterations: usize) -> usize {
    part_1(lines, iterations)
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_21");
    (
        Solution::from(part_1(input.lines(), 64)),
        Solution::from(0),
        // Solution::from(part_2(input.lines(), 26501365)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines(), 6), 16);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_21").lines(), 64), 3617);
    }

    #[test]
    fn test_part_2_example_1() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines(), 10), 50);
    }

    #[test]
    fn test_part_2_example_2() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines(), 50), 1594);
    }

    #[test]
    fn test_part_2_example_3() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines(), 100), 6536);
    }

    #[test]
    fn test_part_2_example_4() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines(), 500), 167004);
    }

    #[test]
    fn test_part_2_example_5() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines(), 1000), 668697);
    }

    #[test]
    fn test_part_2_example_6() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines(), 5000), 16733044);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_21").lines(), 26501365), 0);
    }
}
