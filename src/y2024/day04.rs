use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;
use std::usize;

type Position = (i32, i32);
type Direction = (i32, i32);

fn directions() -> Vec<Direction> {
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

fn in_bounds(pos: Position, max_x: i32, max_y: i32) -> bool {
    pos.0 >= 0 && pos.0 < max_x && pos.1 >= 0 && pos.1 < max_y
}

fn search_in_direction(
    grid: &Vec<Vec<char>>,
    mut pos: Position,
    dir: Direction,
    max_x: i32,
    max_y: i32,
    word: &str,
) -> usize {
    for c in word.chars() {
        pos = (pos.0 + dir.0, pos.1 + dir.1);
        if !in_bounds(pos, max_x, max_y) || grid[pos.0 as usize][pos.1 as usize] != c {
            return 0;
        }
    }
    1
}

// How many times does XMAS appear?
fn part_1(lines: Lines) -> usize {
    let grid: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
    let max_x = grid.len();
    let max_y = grid[0].len();

    let mut result = 0;
    for x in 0..max_x {
        for y in 0..max_y {
            if grid[x][y] == 'X' {
                result += directions()
                    .into_iter()
                    .map(|dir| {
                        search_in_direction(
                            &grid,
                            (x as i32, y as i32),
                            dir,
                            max_x as i32,
                            max_y as i32,
                            "MAS",
                        )
                    })
                    .sum::<usize>();
            }
        }
    }
    result
}

// How many times does MAS appear in the shape of an X?
fn part_2(lines: Lines) -> usize {
    // let grid: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
    // let max_x = grid.len();
    // let max_y = grid[0].len();

    // let mut result = 0;
    // for x in 0..max_x {
    //     for y in 0..max_y {
    //         if grid[x][y] == 'X' {
    //             let cnt =
    //                 count_occurences(&grid, x as i32, y as i32, max_x as i32, max_y as i32, "MAS");
    //             println!("FOUND FROM {:?}: {}", (x, y), cnt);
    //             result += cnt;
    //         }
    //     }
    // }
    // result
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

    const EXAMPLE_INPUT_4: &str = "M.S
.A.
M.S";

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

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_4.lines()), 1);
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(load_input("inputs/2024/day_4")), 100450138);
    // }
}
