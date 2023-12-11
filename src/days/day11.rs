use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::str::Lines;
use std::usize;
use std::{cmp, fs};

fn load_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not open file.")
}

type Position = (usize, usize);

fn parse_galaxies(lines: Lines) -> HashSet<Position> {
    lines
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices().filter_map(
                move |(col, c)| {
                    if c != '.' {
                        Some((row, col))
                    } else {
                        None
                    }
                },
            )
        })
        .collect()
}

// expand every row and column that does not contain a galaxy
fn expand_universe(galaxy: HashSet<Position>, expansion_factor: usize) -> HashSet<Position> {
    let rows: HashSet<usize> = galaxy.iter().map(|(x, _)| *x).collect();
    let cols: HashSet<usize> = galaxy.iter().map(|(_, y)| *y).collect();
    let max_x: usize = *rows.iter().max().unwrap();
    let max_y: usize = *cols.iter().max().unwrap();
    // collect rows and cols that do not contain a galaxy
    let empty_x: Vec<usize> = (0..max_x).filter(|x| !rows.contains(x)).collect();
    let empty_y: Vec<usize> = (0..max_y).filter(|y| !cols.contains(y)).collect();

    galaxy
        .iter()
        .map(|(x, y)| {
            let empty_rows_before = empty_x.iter().filter(|empty| *empty < x).count();
            let empty_cols_before = empty_y.iter().filter(|empty| *empty < y).count();
            (
                x + empty_rows_before * (expansion_factor - 1),
                y + empty_cols_before * (expansion_factor - 1),
            )
        })
        .collect()
}

fn manhattan_distance(a: &Position, b: &Position) -> usize {
    let x_diff = cmp::max(a.0, b.0) - cmp::min(a.0, b.0);
    let y_diff = cmp::max(a.1, b.1) - cmp::min(a.1, b.1);
    x_diff + y_diff
}

fn sum_of_shortest_paths_between_galaxies(galaxies: HashSet<(usize, usize)>) -> usize {
    let mut done: HashSet<Position> = HashSet::new();
    galaxies
        .iter()
        .map(|galaxy| {
            done.insert(*galaxy);
            galaxies
                .difference(&done)
                .map(|g| manhattan_distance(galaxy, g))
                .sum::<usize>()
        })
        .sum()
}

fn part_1(lines: Lines) -> usize {
    let expansion_factor = 2;
    let galaxies = expand_universe(parse_galaxies(lines), expansion_factor);
    sum_of_shortest_paths_between_galaxies(galaxies)
}

fn part_2(lines: Lines) -> usize {
    let expansion_factor = 1_000_000;
    let galaxies = expand_universe(parse_galaxies(lines), expansion_factor);
    sum_of_shortest_paths_between_galaxies(galaxies)
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_1(load_input("inputs/day_11").lines())),
        Solution::from(part_2(load_input("inputs/day_11").lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 374);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_11").lines()), 9609130);
    }

    #[test]
    fn test_part_2_example_expand_by_10() {
        let expansion_factor = 10;
        let galaxies = expand_universe(parse_galaxies(EXAMPLE_INPUT_1.lines()), expansion_factor);
        println!("{:?}", galaxies);
        assert_eq!(sum_of_shortest_paths_between_galaxies(galaxies), 1030);
    }

    #[test]
    fn test_part_2_example_expand_by_100() {
        let expansion_factor = 100;
        let galaxies = expand_universe(parse_galaxies(EXAMPLE_INPUT_1.lines()), expansion_factor);
        println!("{:?}", galaxies);
        assert_eq!(sum_of_shortest_paths_between_galaxies(galaxies), 8410);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_11").lines()), 0);
    }
}
