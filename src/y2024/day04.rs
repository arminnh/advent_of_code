use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;
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
        let in_bounds = pos.0 >= 0 && pos.0 < max_x && pos.1 >= 0 && pos.1 < max_y;

        if !in_bounds || grid[pos.0 as usize][pos.1 as usize] != c {
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

fn check_ass(
    grid: &HashMap<(i32, i32), char>,
    pos: Position,
    a: Direction,
    s1: Direction,
    s2: Direction,
) -> bool {
    match (
        grid.get(&(pos.0 + a.0, pos.1 + a.1)),
        grid.get(&(pos.0 + s1.0, pos.1 + s1.1)),
        grid.get(&(pos.0 + s2.0, pos.1 + s2.1)),
    ) {
        (Some('A'), Some('S'), Some('S')) => true,
        _ => false,
    }
}

fn count_shapes(grid: &HashMap<(i32, i32), char>, x: i32, y: i32) -> usize {
    // Find first M
    if grid.get(&(x, y)) != Some(&'M') {
        return 0;
    }

    let mut result = 0;

    // Check for second M two spots right or down. Don't search for M up or left to avoid counting duplicates.
    // Once orientation of the two M's is determined (vertical/horizontal), we know where the rest of the shape can be.
    if let Some('M') = grid.get(&(x + 2, y)) {
        // Check right
        if check_ass(grid, (x, y), (1, 1), (0, 2), (2, 2)) {
            result += 1;
        }
        // Check left
        if check_ass(grid, (x, y), (1, -1), (0, -2), (2, -2)) {
            result += 1;
        }
    }

    if let Some('M') = grid.get(&(x, y + 2)) {
        // Check down
        if check_ass(grid, (x, y), (1, 1), (2, 0), (2, 2)) {
            result += 1;
        }
        // Check up
        if check_ass(grid, (x, y), (-1, 1), (-2, 0), (-2, 2)) {
            result += 1;
        }
    }

    result
}

// How many times does MAS appear in the shape of an X?
fn part_2(lines: Lines) -> usize {
    let mut grid: HashMap<Position, char> = HashMap::new();
    for (x, line) in lines.enumerate() {
        for (y, c) in line.char_indices() {
            grid.insert((x as i32, y as i32), c);
        }
    }

    grid.keys().map(|(x, y)| count_shapes(&grid, *x, *y)).sum()
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
        assert_eq!(part_2(EXAMPLE_INPUT_2.lines()), 9);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_4").lines()), 1873);
    }
}
