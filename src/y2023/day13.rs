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

fn nr_of_differences<T>(left: &[T], right: &[T]) -> usize
where
    T: Eq,
{
    left.iter()
        .zip(right.iter())
        .filter(|(l, r)| l != r)
        .count()
}

// Find the reflecting row in the grid while cleaning the given number of smudges.
fn reflecting_row(grid: &Vec<Vec<bool>>, smudges: usize) -> Option<usize> {
    for (i, rows) in grid.windows(2).enumerate() {
        let mut differences = nr_of_differences(&rows[0], &rows[1]);
        if differences <= smudges {
            // row i and i+1 reflect -> check if reflection goes back to one of the edges
            for x in 1..=i {
                if i + 1 + x >= grid.len() {
                    // reached edge of grid
                    break;
                }
                differences += nr_of_differences(&grid[i - x], &grid[i + 1 + x]);
                if differences > smudges {
                    // early termination in case there's already too many differences
                    break;
                }
            }
            // Cleaning the exact amount of smudges, not <=
            if differences == smudges {
                return Some(i);
            }
        }
    }

    None
}

// The reflection score of a grid is either the reflecting row * 100 or the reflecting column.
fn reflection_score(grid: Vec<Vec<bool>>, smudges: usize) -> usize {
    if let Some(r) = reflecting_row(&grid, smudges) {
        return (r + 1) * 100;
    }

    let transposed: Vec<Vec<bool>> = transposed(grid);

    if let Some(r) = reflecting_row(&transposed, smudges) {
        return r + 1;
    }

    0
}

pub fn part_1(input: &str) -> usize {
    let smudges = 0;
    input
        .split("\n\n")
        .map(|pattern| reflection_score(parse_grid(pattern), smudges))
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let smudges = 1;
    input
        .split("\n\n")
        .map(|pattern| reflection_score(parse_grid(pattern), smudges))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::util::load_input;

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

    const GRID_1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    const GRID_2: &str = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    const GRID_3: &str = ".#.##.#.#
.##..##..
.#.##.#..
#......##
#......##
.#.##.#..
.##..##.#";

    const GRID_4: &str = "#..#....#
###..##..
.##.#####
.##.#####
###..##..
#..#....#
#..##...#";

    const GRID_5: &str = "#.##..##.
..#.##.#.
##..#...#
##...#..#
..#.##.#.
..##..##.
#.#.##.#.";

    const GRID_6: &str = "###.##.##
##.####.#
##.#..#.#
####..###
....##...
##.#..#.#
...#..#..
##..###.#
##......#
##......#
..#.##.#.
...#..#..
##.####.#
....##...
...####..
....##...
##.####.#";

    const GRID_7: &str = ".##.##...##...##.
#####..##..##..##
.....##..##..##..
.##.#.#.####.#.#.
.##...#.#..#.#...
....#..........#.
#..#..#......#..#
....###.....####.
.##...#.#..#.#...
.....#..####..#..
#..#...##..##...#
....#...#..#...#.
#..#.##########.#
#..##...####...##
#####.##.##.##.##";

    #[test]
    fn test_reflection_score() {
        assert_eq!(reflection_score(parse_grid(GRID_1), 0), 5);
        assert_eq!(reflection_score(parse_grid(GRID_2), 0), 400);
        assert_eq!(reflection_score(parse_grid(GRID_3), 0), 4);
        assert_eq!(reflection_score(parse_grid(GRID_4), 0), 300);
        assert_eq!(reflection_score(parse_grid(GRID_5), 0), 0);
        assert_eq!(reflection_score(parse_grid(GRID_6), 0), 1);
        assert_eq!(reflection_score(parse_grid(GRID_7), 0), 2);
    }

    #[test]
    fn test_reflection_score_tolerance() {
        assert_eq!(reflection_score(parse_grid(GRID_1), 1), 300);
        assert_eq!(reflection_score(parse_grid(GRID_2), 1), 100);
        assert_eq!(reflection_score(parse_grid(GRID_3), 0), 4);
        assert_eq!(reflection_score(parse_grid(GRID_4), 0), 300);
        assert_eq!(reflection_score(parse_grid(GRID_5), 0), 0);
        assert_eq!(reflection_score(parse_grid(GRID_6), 1), 5);
        assert_eq!(reflection_score(parse_grid(GRID_7), 1), 10);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT), 405);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2023/day_13")), 34100);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT), 400);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2023/day_13")), 33106);
    }
}
