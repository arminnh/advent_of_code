use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::usize;

type Position = (usize, usize);
type Grid = HashMap<Position, Tile>;

enum Tile {
    Wall,
    Box,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from(c: char) -> Direction {
        match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Unrecognized direction."),
        }
    }
}

fn next_position((x, y): Position, d: &Direction) -> Position {
    match d {
        Direction::Up => (x - 1, y),
        Direction::Right => (x, y + 1),
        Direction::Down => (x + 1, y),
        Direction::Left => (x, y - 1),
    }
}

fn parse_grid(input: &str) -> (Grid, Position) {
    let mut g: HashMap<Position, Tile> = HashMap::new();
    let mut robot_pos = (0, 0);

    for (x, line) in input.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            let position = (x, y);
            if ch == '#' {
                g.insert(position, Tile::Wall);
            } else if ch == 'O' {
                g.insert(position, Tile::Box);
            } else if ch == '@' {
                robot_pos = position;
            }
        }
    }

    (g, robot_pos)
}

fn display_grid(grid: &Grid, robot: &Position) {
    let max_x = *grid.keys().map(|(x, _)| x).max().unwrap();
    let max_y = *grid.keys().map(|(_, y)| y).max().unwrap();

    for x in 0..=max_x {
        for y in 0..=max_y {
            if let Some(tile) = grid.get(&(x, y)) {
                let c = match tile {
                    Tile::Box => "O",
                    Tile::Wall => "#",
                };
                print!("{}", c);
            } else if &(x, y) == robot {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn move_robot(grid: &mut Grid, pos: Position, d: Direction) -> Position {
    let next_pos = next_position(pos, &d);
    match grid.get(&next_pos) {
        Some(Tile::Wall) => pos,
        Some(Tile::Box) => {
            // Search for the next open spot beyond the box
            let mut beyond_box = next_position(next_pos, &d);
            loop {
                match grid.get(&beyond_box) {
                    // No open spot found, nothing happens.
                    Some(Tile::Wall) => return pos,
                    // Box found, keep moving beyond boxes
                    Some(Tile::Box) => beyond_box = next_position(beyond_box, &d),
                    None => {
                        // Open spot found, move the box into the open spot and move robot to next position
                        grid.insert(beyond_box, Tile::Box);
                        grid.remove(&next_pos);
                        return next_pos;
                    }
                }
            }
        }
        None => next_pos,
    }
}

fn gps_coordinate(tile: &Tile, (x, y): Position) -> usize {
    match tile {
        Tile::Wall => 0,
        Tile::Box => 100 * x + y,
    }
}

fn part_1(input: &str) -> usize {
    let (grid, moves) = input
        .split_once("\n\n")
        .expect("Could not split input in two parts");
    let (mut grid, mut robot) = parse_grid(grid);

    for line in moves.lines() {
        for c in line.chars() {
            robot = move_robot(&mut grid, robot, Direction::from(c));
        }
    }

    grid.iter()
        .map(|(&position, tile)| gps_coordinate(tile, position))
        .sum()
}

fn part_2(input: &str) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_15");
    (
        Solution::from(part_1(&input)),
        Solution::from(part_2(&input)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const EXAMPLE_INPUT_2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(&EXAMPLE_INPUT_1), 2028);
        assert_eq!(part_1(&EXAMPLE_INPUT_2), 10092);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2024/day_15")), 0);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(&EXAMPLE_INPUT_1), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2024/day_15")), 0)
    }
}
