use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::usize;

fn parse_grid(pattern: &str) -> Vec<Vec<bool>> {
    pattern
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '#' => true,
                    _ => false,
                })
                .collect()
        })
        .collect()
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<bool>>) {
    grid.iter().for_each(|row| {
        row.iter()
            .for_each(|&c| print!("{}", if c { '#' } else { ' ' }));
        println!("");
    });
}

fn transposed(grid: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut transposed = vec![vec![false; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = grid[i][j];
        }
    }

    transposed
}

fn reflecting_row(grid: &Vec<Vec<bool>>) -> Option<usize> {
    for (i, rows) in grid.windows(2).enumerate() {
        if rows[0] == rows[1] {
            // row i and i+1 reflect -> check if reflection goes back to one of the edges
            if (1..=i).all(|x| i + 1 + x >= grid.len() || grid[i - x] == grid[i + 1 + x]) {
                return Some(i);
            }
        }
    }

    None
}

fn reflection_score(mut grid: Vec<Vec<bool>>) -> usize {
    if let Some(r) = reflecting_row(&grid) {
        return (r + 1) * 100;
    }

    let transposed: Vec<Vec<bool>> = transposed(grid);

    if let Some(r) = reflecting_row(&transposed) {
        return r + 1;
    }

    panic!("No solution!");
}

fn part_1(input: String) -> usize {
    input
        .split("\n\n")
        .map(|pattern| {
            let grid: Vec<Vec<bool>> = parse_grid(pattern);
            reflection_score(grid)
        })
        .sum()
}

fn part_2(lines: String) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_1(load_input("inputs/day_13"))),
        Solution::from(part_2(load_input("inputs/day_13"))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_reflection_score_vertical() {
        let grid = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        assert_eq!(reflection_score(parse_grid(grid)), 5);
    }

    #[test]
    fn test_reflection_score_horizontal() {
        let grid = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(reflection_score(parse_grid(grid)), 400);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.to_string()), 405);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_13")), 34100);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT), 0);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(load_input("inputs/day_13")), 0);
    // }
}
