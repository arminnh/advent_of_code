use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::str::Lines;
use std::usize;

type Position = (usize, usize);

fn print_grid(
    max_x: usize,
    max_y: usize,
    rocks: &HashSet<(usize, usize)>,
    possible_positions: &HashSet<(usize, usize)>,
) {
    for x in 0..max_x {
        for y in 0..max_y {
            if rocks.contains(&(x, y)) {
                print!("#");
            } else if possible_positions.contains(&(x, y)) {
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

fn rock_positions(grid: &Vec<Vec<u8>>) -> HashSet<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(x, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(y, &c)| if c == b'#' { Some((x, y)) } else { None })
        })
        .collect()
}

fn get_start(grid: &Vec<Vec<u8>>) -> (usize, usize) {
    let x = grid.iter().position(|bytes| bytes.contains(&b'S')).unwrap();
    let y = grid[x].iter().position(|&b| b == b'S').unwrap();
    (x, y)
}

fn neighbors(p: &Position, max_x: usize, max_y: usize) -> Vec<Position> {
    let mut out = Vec::new();
    if p.0 < max_x - 1 {
        out.push((p.0 + 1, p.1));
    }
    if p.0 > 0 {
        out.push((p.0 - 1, p.1));
    }
    if p.1 < max_y - 1 {
        out.push((p.0, p.1 + 1));
    }
    if p.1 > 0 {
        out.push((p.0, p.1 - 1));
    }
    out
}

fn part_1(lines: Lines, iterations: usize) -> usize {
    let grid: Vec<Vec<u8>> = lines.map(|line| line.as_bytes().to_vec()).collect();
    let max_x = grid.len();
    let max_y = grid[0].len();
    let rocks: HashSet<(usize, usize)> = rock_positions(&grid);
    let mut possible_positions: HashSet<Position> = HashSet::from([get_start(&grid)]);

    for i in 0..iterations {
        for position in possible_positions.drain().collect::<Vec<Position>>() {
            for neighbor in neighbors(&position, max_x, max_y) {
                if !rocks.contains(&neighbor) {
                    possible_positions.insert(neighbor);
                }
            }
        }
        println!("Iteration {}:", i);
        print_grid(max_x, max_y, &rocks, &possible_positions);
    }

    possible_positions.len()
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_21");
    (
        Solution::from(part_1(input.lines(), 64)),
        Solution::from(part_2(input.lines())),
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
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_21").lines()), 0);
    }
}
