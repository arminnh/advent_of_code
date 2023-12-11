use crate::{Solution, SolutionPair};
use core::fmt;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::str::{FromStr, Lines};
use std::vec;

fn load_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not open file.")
}

type Position = (i32, i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum BendConnections {
    NorthAndEast,
    NorthAndWest,
    SouthAndEast,
    SouthAndWest,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Start,
    Ground,
    Vertical,
    Horizontal,
    Bend(BendConnections),
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Tile::Start => "S",
            Tile::Ground => ".",
            Tile::Vertical => "║",
            Tile::Horizontal => "═",
            Tile::Bend(connections) => match connections {
                BendConnections::NorthAndEast => "╚",
                BendConnections::NorthAndWest => "╝",
                BendConnections::SouthAndWest => "╗",
                BendConnections::SouthAndEast => "╔",
            },
        };
        write!(f, "{}", out)
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Tile::Vertical),
            "-" => Ok(Tile::Horizontal),
            "L" => Ok(Tile::Bend(BendConnections::NorthAndEast)),
            "J" => Ok(Tile::Bend(BendConnections::NorthAndWest)),
            "7" => Ok(Tile::Bend(BendConnections::SouthAndWest)),
            "F" => Ok(Tile::Bend(BendConnections::SouthAndEast)),
            "." => Ok(Tile::Ground),
            "S" => Ok(Tile::Start),
            _ => Err(()),
        }
    }
}

fn parse_network(lines: Lines) -> HashMap<Position, Tile> {
    let network = lines
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices().map(move |(j, char)| {
                (
                    (i as i32, j as i32),
                    Tile::from_str(&char.to_string()[..]).unwrap(),
                )
            })
        })
        .collect();

    network
}

fn print_network(network: &HashMap<Position, Tile>, cycle: Option<&HashSet<Position>>) {
    let (x, y) = network.keys().max_by_key(|pos| *pos).unwrap();

    for i in 0..=*x {
        for j in 0..=*y {
            if cycle.is_some() && cycle.unwrap().contains(&(i, j)) {
                print!("X");
            } else {
                let tile = network.get(&(i, j)).unwrap();
                print!("{}", *tile);
            }
        }
        println!("");
    }
    println!("");
}

fn find_start(network: &HashMap<Position, Tile>) -> &Position {
    network
        .iter()
        .find_map(|(k, v)| if *v == Tile::Start { Some(k) } else { None })
        .unwrap()
}

fn neighbors_basic(current_pos: &Position) -> Vec<Position> {
    vec![
        (current_pos.0 + 1, current_pos.1),
        (current_pos.0 - 1, current_pos.1),
        (current_pos.0, current_pos.1 + 1),
        (current_pos.0, current_pos.1 - 1),
    ]
}

fn neighbors_network(network: &HashMap<Position, Tile>, position: &Position) -> Vec<Position> {
    let mut out = Vec::new();

    let start_tile_neighbors = [
        (
            (-1, 0),
            HashSet::from([
                Tile::Vertical,
                Tile::Bend(BendConnections::SouthAndWest),
                Tile::Bend(BendConnections::SouthAndEast),
            ]),
        ),
        (
            (1, 0),
            HashSet::from([
                Tile::Vertical,
                Tile::Bend(BendConnections::NorthAndWest),
                Tile::Bend(BendConnections::NorthAndEast),
            ]),
        ),
        (
            (0, -1),
            HashSet::from([
                Tile::Horizontal,
                Tile::Bend(BendConnections::NorthAndEast),
                Tile::Bend(BendConnections::SouthAndEast),
            ]),
        ),
        (
            (0, 1),
            HashSet::from([
                Tile::Horizontal,
                Tile::Bend(BendConnections::NorthAndWest),
                Tile::Bend(BendConnections::SouthAndWest),
            ]),
        ),
    ];

    match network.get(&position).unwrap() {
        Tile::Start => {
            for (offset, possible_tiles) in start_tile_neighbors {
                let neighbor_pos = (position.0 + offset.0, position.1 + offset.1);
                if let Some(neighbor_tile) = network.get(&neighbor_pos) {
                    if possible_tiles.contains(neighbor_tile) {
                        out.push(neighbor_pos)
                    }
                }
            }
        }
        Tile::Ground => (),
        Tile::Vertical => {
            out.push((position.0 - 1, position.1));
            out.push((position.0 + 1, position.1));
        }
        Tile::Horizontal => {
            out.push((position.0, position.1 - 1));
            out.push((position.0, position.1 + 1));
        }
        Tile::Bend(connections) => match connections {
            BendConnections::NorthAndEast => {
                out.push((position.0 - 1, position.1));
                out.push((position.0, position.1 + 1));
            }
            BendConnections::NorthAndWest => {
                out.push((position.0 - 1, position.1));
                out.push((position.0, position.1 - 1));
            }
            BendConnections::SouthAndEast => {
                out.push((position.0 + 1, position.1));
                out.push((position.0, position.1 + 1));
            }
            BendConnections::SouthAndWest => {
                out.push((position.0 + 1, position.1));
                out.push((position.0, position.1 - 1));
            }
        },
    }

    // println!("out :{:?}", out);
    out
}

// Find the furthest point from the start position
fn part_1(lines: Lines) -> i32 {
    let network = parse_network(lines);
    let start = find_start(&network);
    // print_network(&network, None);

    let mut frontier: VecDeque<(Position, i32)> = VecDeque::from(vec![(start.clone(), 0)]);
    let mut visited: HashSet<Position> = HashSet::new();
    let mut max_steps = 0;

    while !frontier.is_empty() {
        let (current_pos, steps) = frontier.pop_front().unwrap();
        if steps > max_steps {
            max_steps = steps;
        }

        if !visited.contains(&current_pos) {
            // println!("{:?}, {:?}", current_pos, steps);
            visited.insert(current_pos.clone());
            for next_pos in neighbors_network(&network, &current_pos) {
                if !visited.contains(&next_pos) && network.contains_key(&next_pos) {
                    frontier.push_back((next_pos, steps + 1))
                }
            }
        }
    }

    max_steps
}

// Insert ground between everything so that I don't have to handle special cases in the flood fill
fn add_ground(network: HashMap<(i32, i32), Tile>) -> HashMap<(i32, i32), Tile> {
    let mut out = HashMap::new();

    // Each tile gets moved into a 2x2 block
    let (x, y) = network.keys().max_by_key(|pos| *pos).unwrap();
    for i in 0..=*x {
        for j in 0..=*y {
            let tile = network.get(&(i, j)).unwrap();
            // The tile itself goes bottom right
            out.insert((2 * i, 2 * j), *tile);
            // Top left is always Ground
            out.insert((2 * i - 1, 2 * j - 1), Tile::Ground);

            // Top right, and bottom left depend on tile type
            let (top_right, bottom_left) = match tile {
                Tile::Start => (
                    match network.get(&(i - 1, j)).unwrap_or(&Tile::Ground) {
                        Tile::Vertical
                        | Tile::Bend(BendConnections::SouthAndWest)
                        | Tile::Bend(BendConnections::SouthAndEast) => Tile::Vertical,
                        _ => Tile::Ground,
                    },
                    match network.get(&(i, j - 1)).unwrap_or(&Tile::Ground) {
                        Tile::Horizontal
                        | Tile::Bend(BendConnections::NorthAndEast)
                        | Tile::Bend(BendConnections::SouthAndEast) => Tile::Horizontal,
                        _ => Tile::Ground,
                    },
                ),
                Tile::Ground => (Tile::Ground, Tile::Ground),
                Tile::Vertical => (Tile::Vertical, Tile::Ground),
                Tile::Horizontal => (Tile::Ground, Tile::Horizontal),
                Tile::Bend(connection) => match connection {
                    BendConnections::NorthAndEast => (Tile::Vertical, Tile::Ground),
                    BendConnections::NorthAndWest => (Tile::Vertical, Tile::Horizontal),
                    BendConnections::SouthAndEast => (Tile::Ground, Tile::Ground),
                    BendConnections::SouthAndWest => (Tile::Ground, Tile::Horizontal),
                },
            };

            out.insert((2 * i - 1, 2 * j), top_right);
            out.insert((2 * i, 2 * j - 1), bottom_left);
        }
    }

    out
}

fn find_cycle(network: &HashMap<Position, Tile>, start: &Position) -> HashSet<Position> {
    let mut frontier: VecDeque<Position> = VecDeque::from([start.clone()]);
    let mut visited: HashSet<Position> = HashSet::new();

    while let Some(current_pos) = frontier.pop_front() {
        if !visited.contains(&current_pos) {
            visited.insert(current_pos.clone());
            for next_pos in neighbors_network(&network, &current_pos) {
                if !visited.contains(&next_pos) && network.contains_key(&next_pos) {
                    frontier.push_back(next_pos);
                }
            }
        }
    }

    visited
}

fn flood_fill(network: &HashMap<Position, Tile>, cycle: &HashSet<Position>) -> HashSet<Position> {
    let mut frontier: VecDeque<Position> = VecDeque::new();
    let mut visited: HashSet<Position> = HashSet::new();

    // fill frontier with edges of the map
    let (x, y) = network.keys().max_by_key(|pos| *pos).unwrap();
    let left_and_right_edges = (0..=*x).into_iter().flat_map(|i| [(i, 0), (i, *y)]);
    let top_and_bottom_edges = (0..=*y).into_iter().flat_map(|j| [(0, j), (*x, j)]);
    left_and_right_edges
        .chain(top_and_bottom_edges)
        .filter(|pos| !cycle.contains(pos))
        .for_each(|pos| frontier.push_back(pos));

    while let Some(current_pos) = frontier.pop_front() {
        if !visited.contains(&current_pos) {
            visited.insert(current_pos);

            for next_pos in neighbors_basic(&current_pos) {
                if !visited.contains(&next_pos)
                    && network.contains_key(&next_pos)
                    && !cycle.contains(&next_pos)
                {
                    frontier.push_back(next_pos);
                }
            }
        }
    }

    visited
}

// Find the number of tiles enclosed by the loop that contains the start position
fn part_2(lines: Lines) -> usize {
    let original_network = parse_network(lines);
    // print_network(&original_network, None);

    let network = add_ground(original_network);
    // print_network(&network, None);

    let start = find_start(&network);
    let cycle: HashSet<Position> = find_cycle(&network, start);
    // print_network(&network, Some(&cycle));

    let flood: HashSet<Position> = flood_fill(&network, &cycle);
    // print_network(&network, Some(&flood));

    let remaining: HashSet<Position> = network
        .keys()
        .cloned()
        .collect::<HashSet<Position>>()
        .difference(&cycle.union(&flood).cloned().collect())
        .cloned()
        .collect();
    // print_network(&network, Some(&remaining));

    let remaining_from_original: HashSet<Position> = remaining
        .iter()
        .filter(|(x, y)| x % 2 == 0 && y % 2 == 0)
        .cloned()
        .collect();
    // print_network(&network, Some(&remaining_from_original));

    remaining_from_original.len()
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_1(load_input("inputs/day_10").lines())),
        Solution::from(part_2(load_input("inputs/day_10").lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example_1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        assert_eq!(part_1(input.lines()), 4);
    }

    #[test]
    fn test_part_1_example_2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

        assert_eq!(part_1(input.lines()), 8);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_10").lines()), 6786);
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(part_2(input.lines()), 4);
    }

    #[test]
    fn test_part_2_example_2() {
        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";

        assert_eq!(part_2(input.lines()), 4);
    }

    #[test]
    fn test_part_2_example_3() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

        assert_eq!(part_2(input.lines()), 8);
    }

    #[test]
    fn test_part_2_example_4() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ.F7FJ-
L---JF-JLJ....FJLJJ7
|F|F-JF---7...L7L|7|
|FFJF7L7F-JF7..L---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

        assert_eq!(part_2(input.lines()), 10);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_10").lines()), 495);
    }
}
