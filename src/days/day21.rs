use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::str::Lines;
use std::usize;

type Position = (i32, i32);

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

fn part_1(lines: Lines, iterations: usize) -> usize {
    let grid: Vec<Vec<u8>> = lines.map(|line| line.as_bytes().to_vec()).collect();
    let rocks: HashSet<Position> = rock_positions(&grid);
    let start = get_start(&grid);
    let mut possible_positions_even: HashSet<Position> = HashSet::from([start]);
    let mut possible_positions_uneven: HashSet<Position> = HashSet::from([]);
    let mut last_iteration_positions: HashSet<Position> = HashSet::from([start]);

    for i in 0..iterations {
        for p in last_iteration_positions.drain().collect::<Vec<Position>>() {
            for n in neighbors(&p).into_iter().filter(|n| !rocks.contains(n)) {
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
    }

    if iterations % 2 == 0 {
        possible_positions_even.len()
    } else {
        possible_positions_uneven.len()
    }
}

fn part_2(_lines: Lines, _iterations: usize) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_21");
    (
        Solution::from(part_1(input.lines(), 64)),
        Solution::from(part_2(input.lines(), 64)),
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
