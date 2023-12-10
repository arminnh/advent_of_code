use crate::{Solution, SolutionPair};
use core::fmt;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::str::{FromStr, Lines};

fn load_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not open file.")
}

type Position = (i32, i32);

#[derive(Debug, PartialEq, Eq, Hash)]
enum BendConnections {
    NorthAndEast,
    NorthAndWest,
    SouthAndEast,
    SouthAndWest,
}

#[derive(Debug, PartialEq, Eq, Hash)]
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
            Tile::Vertical => "|",
            Tile::Horizontal => "-",
            Tile::Bend(connections) => match connections {
                BendConnections::NorthAndEast => "L",
                BendConnections::NorthAndWest => "J",
                BendConnections::SouthAndWest => "7",
                BendConnections::SouthAndEast => "F",
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

fn print_network(network: &HashMap<Position, Tile>) -> () {
    let (x, _) = *network.keys().max_by_key(|(x, _)| x).unwrap();

    for i in 0..=x {
        for j in 0..=x {
            let tile = network.get(&(i, j)).unwrap();
            print!("{}", *tile);
        }
        println!("");
    }
}

fn find_start(network: &HashMap<Position, Tile>) -> &Position {
    network
        .iter()
        .find_map(|(k, v)| if *v == Tile::Start { Some(k) } else { None })
        .unwrap()
}

fn neighbor_positions(network: &HashMap<(i32, i32), Tile>, position: &Position) -> Vec<Position> {
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

fn part_1(lines: Lines) -> i32 {
    let network = parse_network(lines);
    let start = find_start(&network);
    // print_network(&network);

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
            for next_pos in neighbor_positions(&network, &current_pos) {
                if !visited.contains(&next_pos) && network.contains_key(&next_pos) {
                    frontier.push_back((next_pos, steps + 1))
                }
            }
        }
    }

    max_steps
}

fn part_2(_lines: Lines) -> i32 {
    0
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

    const EXAMPLE_INPUT_1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const EXAMPLE_INPUT_2: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_part_1_example_1() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 4);
    }

    #[test]
    fn test_part_1_example_2() {
        assert_eq!(part_1(EXAMPLE_INPUT_2.lines()), 8);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_10").lines()), 0);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(load_input("inputs/day_10").lines()), 0);
    // }
}
