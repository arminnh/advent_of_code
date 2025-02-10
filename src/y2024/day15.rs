use std::collections::HashMap;
use std::usize;

type Position = (usize, usize);
type Grid = HashMap<Position, Tile>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    // Part 1: 1 box = 1 position
    Box,
    // Part 2: 1 box = 2 positions
    LeftHalfBox,
    RightHalfBox,
}

#[derive(PartialEq, Eq, Debug)]
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
            } else if ch == '[' {
                g.insert(position, Tile::LeftHalfBox);
            } else if ch == ']' {
                g.insert(position, Tile::RightHalfBox);
            } else if ch == '@' {
                robot_pos = position;
            }
        }
    }

    (g, robot_pos)
}

fn display_grid(grid: &Grid, robot: &Position, d: &Direction) {
    let max_x = *grid.keys().map(|(x, _)| x).max().unwrap();
    let max_y = *grid.keys().map(|(_, y)| y).max().unwrap();

    for x in 0..=max_x {
        for y in 0..=max_y {
            if let Some(tile) = grid.get(&(x, y)) {
                let c = match tile {
                    Tile::Box => "O",
                    Tile::Wall => "#",
                    Tile::LeftHalfBox => "[",
                    Tile::RightHalfBox => "]",
                };
                print!("{}", c);
            } else if &(x, y) == robot {
                match d {
                    Direction::Up => print!("^"),
                    Direction::Right => print!(">"),
                    Direction::Down => print!("v"),
                    Direction::Left => print!("<"),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

// Part 1 moves all boxes in front of the robot until they block each other against a wall
fn box_move_part1(
    grid: &mut Grid,
    robot_pos: Position,
    box_pos: Position,
    d: &Direction,
) -> Position {
    // Search for the next open spot beyond the box
    let mut beyond_box = next_position(box_pos, d);
    loop {
        match grid.get(&beyond_box) {
            // No open spot found, nothing happens
            Some(Tile::Wall) => return robot_pos,
            // Keep moving beyond boxes
            Some(Tile::Box) => beyond_box = next_position(beyond_box, d),
            _ => {
                // Open spot found, move the box into the open spot and move robot to next position
                grid.insert(beyond_box, Tile::Box);
                grid.remove(&box_pos);
                return box_pos;
            }
        }
    }
}

// Part 2 moves two halves of boxes
fn box_move_part2(
    grid: &mut Grid,
    robot_pos: Position,
    box_pos: Position,
    d: &Direction,
) -> Position {
    match d {
        // For horizontal moves, only need to find the next open spot to move both halves of the box into
        Direction::Left | Direction::Right => {
            // Can jump in steps of 2
            let mut beyond_box = next_position(next_position(box_pos, &d), &d);
            loop {
                match grid.get(&beyond_box) {
                    // No open spot found, nothing happens
                    Some(Tile::Wall) => return robot_pos,
                    // Keep moving beyond boxes
                    Some(Tile::LeftHalfBox) | Some(Tile::RightHalfBox) => {
                        beyond_box = next_position(next_position(beyond_box, &d), &d)
                    }
                    _ => {
                        // Open spot found. Can move all boxes between robot and this spot.
                        let y_range: Vec<_> = if *d == Direction::Left {
                            (beyond_box.1..robot_pos.1).collect()
                        } else {
                            ((robot_pos.1 + 1)..=beyond_box.1).rev().collect()
                        };
                        // Slide over the positions to move in windows of 2
                        for w in y_range
                            .into_iter()
                            .map(|y| (robot_pos.0, y))
                            .collect::<Vec<_>>()
                            .windows(2)
                        {
                            let tile = *grid.get(&w[1]).unwrap();
                            grid.insert(w[0], tile);
                        }
                        // All blocks have been copied over, clear the last remaining cell
                        grid.remove(&box_pos);
                        // Move robot to where the box was
                        return box_pos;
                    }
                }
            }
        }
        // Halves of boxes can touch each other.
        // Find all boxes that have to be moved if the vertical move is possible
        Direction::Up | Direction::Down => {
            if let Some(moves) = boxes_to_move_vertical(grid, box_pos, d) {
                // Clear old positions
                for old in moves.keys() {
                    grid.remove(old);
                }
                for &(tile, new_position) in moves.values() {
                    grid.insert(new_position, tile);
                }
                box_pos
            } else {
                robot_pos
            }
        }
    }
}

// Find all boxes that have to be moved if the vertical move is possible.
// Map old positions to the tile that will be on the next position.
fn boxes_to_move_vertical(
    grid: &mut Grid,
    box_pos: Position,
    d: &Direction,
) -> Option<HashMap<Position, (Tile, Position)>> {
    let mut box_moves: HashMap<Position, (Tile, Position)> = HashMap::new();
    let mut frontier: Vec<Position> = Vec::from([box_pos]);

    while let Some(pos) = frontier.pop() {
        if box_moves.contains_key(&pos) {
            continue;
        }
        if let Some(&tile) = grid.get(&pos) {
            if tile == Tile::Wall {
                // Wall blocks the moves -> nothing happens
                return None;
            } else if tile == Tile::LeftHalfBox || tile == Tile::RightHalfBox {
                // Move both halves of the boxes and explore beyond them
                let (pos_left, pos_right) = if tile == Tile::LeftHalfBox {
                    (pos, (pos.0, pos.1 + 1))
                } else {
                    ((pos.0, pos.1 - 1), pos)
                };
                let next_pos_left = next_position(pos_left, &d);
                let next_pos_right = next_position(pos_right, &d);
                box_moves.insert(pos_left, (Tile::LeftHalfBox, next_pos_left));
                box_moves.insert(pos_right, (Tile::RightHalfBox, next_pos_right));
                frontier.push(next_pos_left);
                frontier.push(next_pos_right);
            }
        }
    }
    Some(box_moves)
}

fn move_robot(grid: &mut Grid, pos: Position, d: &Direction) -> Position {
    let next_pos = next_position(pos, &d);
    match grid.get(&next_pos) {
        Some(Tile::Wall) => pos,
        Some(Tile::Box) => box_move_part1(grid, pos, next_pos, d),
        Some(Tile::LeftHalfBox) | Some(Tile::RightHalfBox) => {
            box_move_part2(grid, pos, next_pos, d)
        }
        None => next_pos,
    }
}

fn do_moves(moves: &str, mut robot: Position, grid: &mut HashMap<(usize, usize), Tile>) {
    // println!("Begin:");
    // display_grid(&grid, &robot, &Direction::from(moves.chars().nth(0).unwrap()));
    for line in moves.lines() {
        for c in line.chars() {
            let d = Direction::from(c);
            robot = move_robot(grid, robot, &d);
            // display_grid(grid, &robot, &d);
        }
    }
    // println!("\nEnd:");
    // display_grid(&grid, &robot, &Direction::from(moves.trim_end().chars().last().unwrap()));
}

fn gps_coordinate(tile: &Tile, (x, y): Position) -> usize {
    match tile {
        Tile::Box | Tile::LeftHalfBox => 100 * x + y,
        _ => 0,
    }
}

fn double_everything(grid: &str) -> String {
    let mut new_grid = String::with_capacity(grid.len() * 2);
    for c in grid.chars() {
        match c {
            'O' => new_grid += "[]",
            '@' => new_grid += "@.",
            '\n' => new_grid += "\n",
            _ => new_grid += &c.to_string().repeat(2),
        }
    }
    new_grid
}

fn solution(input: &str, double_all_the_things: bool) -> usize {
    let (mut grid, moves) = input
        .split_once("\n\n")
        .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
        .expect("Could not split input in two parts");
    if double_all_the_things {
        grid = double_everything(&grid);
    }
    let (mut grid, robot) = parse_grid(&grid);
    do_moves(&moves, robot, &mut grid);
    grid.iter()
        .map(|(&position, tile)| gps_coordinate(tile, position))
        .sum()
}

pub fn part_1(input: &str) -> usize {
    solution(input, false)
}

pub fn part_2(input: &str) -> usize {
    solution(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::util::load_input;

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

    const EXAMPLE_INPUT_3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(&EXAMPLE_INPUT_1), 2028);
        assert_eq!(part_1(&EXAMPLE_INPUT_2), 10092);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2024/day_15")), 1412971);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(&EXAMPLE_INPUT_3), 105 + 207 + 306);
        assert_eq!(part_2(&EXAMPLE_INPUT_2), 9021);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2024/day_15")), 1429299)
    }
}
