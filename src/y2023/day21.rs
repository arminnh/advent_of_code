use crate::util::util::load_input;
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
    rocks: &HashSet<(i32, i32)>,
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

    nr_of_possible_positions(&rocks, max_x, max_y, start, iterations)
}

fn second_order_lagrange_polynomial(x_i: Vec<f64>, y_i: Vec<f64>) -> impl Fn(f64) -> f64 {
    move |new_x| -> f64 {
        let term_1 = y_i[0]
            * ((new_x - x_i[1]) / (x_i[0] - x_i[1]))
            * ((new_x - x_i[2]) / (x_i[0] - x_i[2]));
        let term_2 = y_i[1]
            * ((new_x - x_i[0]) / (x_i[1] - x_i[0]))
            * ((new_x - x_i[2]) / (x_i[1] - x_i[2]));
        let term_3 = y_i[2]
            * ((new_x - x_i[0]) / (x_i[2] - x_i[0]))
            * ((new_x - x_i[1]) / (x_i[2] - x_i[1]));
        term_1 + term_2 + term_3
    }
}

// This elf is very into ultra marathon running
fn part_2(lines: Lines, iterations: usize) -> i64 {
    let grid: Vec<Vec<u8>> = lines.map(|line| line.as_bytes().to_vec()).collect();
    let max_x = grid.len() as i32;
    let max_y = grid[0].len() as i32;
    let start = get_start(&grid);
    let rocks = rock_positions(&grid);

    // The pattern of growth repeats after max_x steps (size of grid). The start is in the center.
    // The number of positions that can be reached is a quadratic function
    // -> Lagrange interpolation to map nr of iterations to nr of positions
    // -> Can be used to extrapolate to other iterations where the pattern repeats, which 26501365 luckily is.
    let x_i: Vec<f64> = vec![
        start.0 as f64,
        start.0 as f64 + max_x as f64,
        start.0 as f64 + 2.0 * max_x as f64,
    ];
    let y_i: Vec<f64> = x_i
        .iter()
        .map(|&iters| nr_of_possible_positions(&rocks, max_x, max_y, start, iters as usize) as f64)
        .collect();

    let polynomial_fn = second_order_lagrange_polynomial(x_i, y_i);
    polynomial_fn(iterations as f64) as i64
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2023/day_21");
    (
        Solution::from(part_1(input.lines(), 64)),
        Solution::from(part_2(input.lines(), 26501365)),
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
        assert_eq!(part_1(load_input("inputs/2023/day_21").lines(), 64), 3617);
    }

    #[test]
    fn test_lagrange_fn() {
        let polynomial = second_order_lagrange_polynomial(vec![0.0, 1.0, 2.0], vec![0.0, 1.0, 4.0]);
        assert_eq!(polynomial(0.0), 0.0);
        assert_eq!(polynomial(1.0), 1.0);
        assert_eq!(polynomial(2.0), 4.0);
        assert_eq!(polynomial(3.0), 9.0);
        assert_eq!(polynomial(4.0), 16.0);
        assert_eq!(polynomial(10.0), 100.0);
        assert_eq!(polynomial(20.0), 400.0);
    }

    #[test]
    fn test_lagrange_fn_2() {
        let polynomial = second_order_lagrange_polynomial(
            vec![65.0, 196.0, 327.0],
            vec![3703.0, 32957.0, 91379.0],
        );
        assert_eq!(polynomial(26501365.0), 596857397104703.0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(load_input("inputs/2023/day_21").lines(), 26501365),
            596857397104703
        );
    }
}
