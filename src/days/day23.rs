use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{BinaryHeap, HashSet};
use std::str::Lines;
use std::usize;

type Position = (i32, i32);

fn neighbors(p: &Position, grid: &Grid, visited: &HashSet<Position>) -> Vec<Position> {
    let mut out: Vec<Position> = Vec::new();
    let neighbors = [
        (p.0 + 1, p.1),
        (p.0 - 1, p.1),
        (p.0, p.1 + 1),
        (p.0, p.1 - 1),
    ];

    for neighbor in neighbors {
        if let Some(c) = grid.at(&neighbor) {
            // dbg!(&c, &p, &neighbor);
            if c == b'#'
                || visited.contains(&neighbor)
                || (c == b'>' && neighbor.1 < p.1)
                || (c == b'v' && neighbor.0 < p.0)
            {
                continue;
            }
            out.push(neighbor);
        };
    }

    out
}

struct Grid {
    grid: Vec<u8>,
    max_x: i32,
    max_y: i32,
}

impl Grid {
    fn from_lines(lines: Lines<'_>) -> Self {
        let grid: Vec<Vec<u8>> = lines
            .map(|line| line.chars().map(|c| c as u8).collect::<Vec<u8>>())
            .collect();
        let max_x = grid.len() as i32;
        let max_y = grid[0].len() as i32;

        Grid {
            grid: grid.into_iter().flat_map(|x| x).collect(),
            max_x,
            max_y,
        }
    }

    fn at(&self, p: &Position) -> Option<u8> {
        if p.0 >= 0 && p.0 < self.max_x as i32 && p.1 >= 0 && p.1 < self.max_y as i32 {
            Some(self.grid[(p.0 * self.max_y + p.1) as usize])
        } else {
            None
        }
    }

    fn is_slope_at(&self, p: &Position) -> bool {
        match self.at(&p) {
            Some(b'>') | Some(b'v') => true,
            _ => false,
        }
    }

    fn print(&self, visited: &HashSet<Position>, current: Position) {
        for x in 0..self.max_x {
            for y in 0..self.max_y {
                let p = (x, y);
                if current == p {
                    print!("█");
                } else if visited.contains(&p) {
                    match self.at(&p).unwrap() {
                        b'>' => print!("⯮"),
                        b'v' => print!("⯯"),
                        _ => print!("O"),
                    }
                } else {
                    print!("{}", self.at(&p).unwrap() as char);
                }
            }
            println!("");
        }
        println!("");
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Move {
    cost: usize,
    position: Position,
}

impl Move {
    fn next(&self, grid: &Grid, visited: &HashSet<Position>) -> Vec<Self> {
        match grid.at(&self.position).unwrap() {
            b'>' => vec![Move {
                cost: self.cost + 1,
                position: (self.position.0, self.position.1 + 1),
            }],
            b'v' => vec![Move {
                cost: self.cost + 1,
                position: (self.position.0 + 1, self.position.1),
            }],
            _ => neighbors(&self.position, grid, visited)
                .into_iter()
                .map(|p| Move {
                    cost: self.cost + 1,
                    position: p,
                })
                .collect(),
        }
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // BinaryHeap is a max-heap, we want to search by highest cost first
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn recursive_dijkstra(
    grid: &mut Grid,
    start: Move,
    goal: Position,
    visited: HashSet<Position>,
) -> usize {
    let mut vis = visited;
    let mut frontier: BinaryHeap<Move> = BinaryHeap::from([start]);

    while let Some(current) = frontier.pop() {
        if current.position == goal {
            // grid.print(&vis, current.position);
            // println!("Cost: {:?}\n\n", current.cost);
            return current.cost;
        }

        if vis.insert(current.position) {
            let successors = current.next(grid, &vis);
            let contains_slope = successors.iter().any(|m| grid.is_slope_at(&m.position));

            if successors.len() > 1 && contains_slope {
                return successors
                    .into_iter()
                    .map(|m| recursive_dijkstra(grid, m, goal, vis.clone()))
                    .max().unwrap();
            } else {
                successors.into_iter().for_each(|next| frontier.push(next));
            }
        }
    }

    panic!("Oh no!")
}

fn part_1(lines: Lines) -> usize {
    let mut grid = Grid::from_lines(lines);
    let start = Move {
        cost: 0,
        position: (0, 1),
    };
    let goal = (grid.max_x - 1, grid.max_y - 2);
    recursive_dijkstra(&mut grid, start, goal, HashSet::new())
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_23");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 94);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_23").lines()), 2334);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_23").lines()), 0);
    }
}
