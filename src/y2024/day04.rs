use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;
use std::usize;

fn directions() -> Vec<(i32, i32)> {
    vec![
        (1, 0),
        (1, 1),
        (1, -1),
        (-1, 0),
        (-1, 1),
        (-1, -1),
        (0, 1),
        (0, -1),
    ]
}

fn neighbors(x: i32, y: i32, max_x: i32, max_y: i32) -> Vec<(i32, i32)> {
    vec![
        (x + 1, y),
        (x + 1, y + 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x - 1, y - 1),
        (x, y + 1),
        (x, y - 1),
    ]
    .into_iter()
    .filter(|&(x, y)| x > 0 && x < max_x && y > 0 && y < max_y)
    .collect()
}

fn search_in_direction(
    grid: &Vec<Vec<char>>,
    x: i32,
    y: i32,
    dir_x: i32,
    dir_y: i32,
    max_x: i32,
    max_y: i32,
    rest_of_word: &str,
) -> usize {
    if rest_of_word.is_empty() {
        return 1;
    }
    let next_char = rest_of_word.chars().next().unwrap();
    let next_x = x + dir_x;
    let next_y = y + dir_y;
    if next_x < 0 || next_x >= max_x || next_y < 0 || next_y >= max_y {
        return 0;
    }
    if grid[next_x as usize][next_y as usize] == next_char {
        return search_in_direction(
            grid,
            next_x,
            next_y,
            dir_x,
            dir_y,
            max_x,
            max_y,
            &rest_of_word[1..],
        );
    }
    0
}

fn count_occurences(
    grid: &Vec<Vec<char>>,
    x: i32,
    y: i32,
    max_x: i32,
    max_y: i32,
    rest_of_word: &str,
) -> usize {
    println!(
        "{:?}, {}: {:?}",
        (x, y),
        grid[x as usize][y as usize],
        rest_of_word
    );
    if rest_of_word.is_empty() {
        return 1;
    }
    directions()
        .into_iter()
        .map(|(dir_x, dir_y)| {
            search_in_direction(grid, x, y, dir_x, dir_y, max_x, max_y, rest_of_word)
        })
        .sum()
}

// Add up all the results of the mult operations
fn part_1(lines: Lines) -> usize {
    let grid: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
    let max_x = grid.len();
    let max_y = grid[0].len();

    let mut result = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            if grid[x][y] == 'X' {
                let cnt =
                    count_occurences(&grid, x as i32, y as i32, max_x as i32, max_y as i32, "MAS");
                println!("FOUND FROM {:?}: {}", (x, y), cnt);
                result += cnt;
            }
        }
    }
    result
}

// The most recent do() or don't() instruction determines whether to apply the mul()
fn part_2(lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_4");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";

    const EXAMPLE_INPUT_2: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    const EXAMPLE_INPUT_3: &str = "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 4);
        assert_eq!(part_1(EXAMPLE_INPUT_2.lines()), 18);
        assert_eq!(part_1(EXAMPLE_INPUT_3.lines()), 18);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_4").lines()), 2524);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT_2.lines()), 48);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(load_input("inputs/2024/day_4")), 100450138);
    // }
}
