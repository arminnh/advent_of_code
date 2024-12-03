use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;

fn parse_grid(lines: Lines) -> Vec<Vec<u32>> {
    lines
        .map(|l| {
            let row = l
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            println!("{:?}", row);
            row

        })
        .collect::<Vec<Vec<u32>>>()
}

fn print_visbility(visibility: &Vec<Vec<bool>>) {
    println!();
    for row in visibility {
        println!("{:?}", row.iter().map(|&c| c as u8).collect::<Vec<u8>>());
    }
}

// A tree is visible if all of the other trees between it and an edge of
// the grid are shorter than it. Only consider trees in the same row or column;
// that is, only look up, down, left, or right from any given tree.
fn part_1(lines: Lines) -> u32 {
    let grid: Vec<Vec<u32>> = parse_grid(lines);
    let size: usize = grid.len();
    let mut visibility: Vec<Vec<bool>> = vec![vec![false; size]; size];

    // Edges start off visible
    (0..size).for_each(|i| {
        visibility[i][0] = true;
        visibility[i][size - 1] = true;
        visibility[0][i] = true;
        visibility[size - 1][i] = true;
    });
    print_visbility(&visibility);

    for i in 1..size - 1 {
        let mut max_left_to_right = grid[i][0];
        let mut max_right_to_left = grid[i][size - 1];
        let mut max_top_to_bottom = grid[0][i];
        let mut max_bottom_to_top = grid[size - 1][i];
        for j in 1..size - 1 {
            if grid[i][j] > max_left_to_right {
                visibility[i][j] = true;
                max_left_to_right = grid[i][j]
            }
            if grid[i][size - 1 - j] > max_right_to_left {
                visibility[i][size - 1 - j] = true;
                max_right_to_left = grid[i][size - 1 - j]
            }
            if grid[j][i] > max_top_to_bottom {
                visibility[j][i] = true;
                max_top_to_bottom = grid[j][i]
            }
            if grid[size - 1 - j][i] > max_bottom_to_top {
                visibility[size - 1 - j][i] = true;
                max_bottom_to_top = grid[size - 1 - j][i]
            }
        }
    }
    print_visbility(&visibility);

    visibility.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |acc, &c| acc + c as u32)
    })
}

// A tree's scenic score is found by multiplying together its viewing distance in each of the four directions.
fn scenic_score(grid: &Vec<Vec<u32>>, i: usize, j: usize, size: usize) -> u32 {
    let height: u32 = grid[i][j];
    let mut view_dist: [u32; 4] = [0, 0, 0, 0];

    // up
    for k in 1..i + 1 {
        view_dist[0] += 1;
        if height <= grid[i - k][j] {
            break;
        }
    }
    // down
    for k in i + 1..size {
        view_dist[1] += 1;
        if height <= grid[k][j] {
            break;
        }
    }
    // left
    for k in 1..j + 1 {
        view_dist[2] += 1;
        if height <= grid[i][j - k] {
            break;
        }
    }
    // right
    for k in j + 1..size {
        view_dist[3] += 1;
        if height <= grid[i][k] {
            break;
        }
    }

    view_dist[0] * view_dist[1] * view_dist[2] * view_dist[3]
}

// Consider each tree on your map. What is the highest scenic score possible for any tree?
fn part_2(lines: Lines) -> u32 {
    let grid: Vec<Vec<u32>> = parse_grid(lines);
    let mut max: u32 = 0;
    let size: usize = grid.len();

    for i in 1..size - 1 {
        for j in 1..size - 1 {
            let score: u32 = scenic_score(&grid, i, j, size);
            max = std::cmp::max(max, score);
        }
    }

    max
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2022/day_8");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT.lines()), 21)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT.lines()), 8)
    }
}
