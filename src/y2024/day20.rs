use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;
use std::usize;

type Position = (i32, i32);
type Path = Vec<Position>;
type Grid = HashSet<Position>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Move {
    cost: usize,
    position: Position,
    path: Path,
}

fn next_moves(grid: &Grid, current_move: &Move) -> Vec<Move> {
    let mut next = Vec::new();
    for (dx, dy) in [((-1, 0)), ((0, 1)), ((1, 0)), ((0, -1))] {
        let pos = (current_move.position.0 + dx, current_move.position.1 + dy);
        if grid.contains(&pos) {
            next.push(Move {
                cost: 1,
                position: pos,
                path: current_move
                    .path
                    .clone()
                    .into_iter()
                    .chain(vec![pos].into_iter())
                    .collect(),
            })
        }
    }
    next
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // BinaryHeap is a max-heap
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra<FN1, FN2, IN>(
    grid: &Grid,
    start: Move,
    success: FN1,
    successors: FN2,
) -> Option<(usize, Path)>
where
    FN1: Fn(&Move) -> bool,
    FN2: Fn(&Grid, &Move) -> IN,
    IN: IntoIterator<Item = Move>,
{
    let mut visited: HashSet<Position> = HashSet::new();
    let mut frontier: BinaryHeap<Reverse<(usize, Move)>> = BinaryHeap::from([Reverse((0, start))]);

    while let Some(Reverse((total_cost, current))) = frontier.pop() {
        if success(&current) {
            return Some((total_cost, current.path));
        }

        if visited.insert(current.position) {
            for next in successors(grid, &current) {
                frontier.push(Reverse((total_cost + next.cost, next)));
            }
        }
    }

    None
}

fn parse_input(input: &str) -> (Grid, Position, Position) {
    let mut grid = HashSet::new();
    let mut start = (-1, -1);
    let mut end = (-1, -1);

    for (x, line) in input.lines().enumerate() {
        for (y, ch) in line.char_indices() {
            let pos = (x as i32, y as i32);
            if ch == 'S' {
                grid.insert(pos);
                start = pos;
            } else if ch == 'E' {
                grid.insert(pos);
                end = pos;
            } else if ch == '.' {
                grid.insert(pos);
            }
        }
    }

    (grid, start, end)
}

fn find_best_path(grid: HashSet<(i32, i32)>, start: (i32, i32), end: (i32, i32)) -> Path {
    let start_move = Move {
        position: start,
        cost: 0,
        path: Vec::from([start]),
    };

    let result = dijkstra(
        &grid,
        start_move,
        |m| m.position == end,
        |g, m| next_moves(g, m),
    );
    result.expect("No path found!").1
}

fn distance(a: &(i32, i32), b: &(i32, i32)) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn nr_of_cheats(path: Vec<(i32, i32)>, max_cheat_distance: usize) -> usize {
    let mut result = 0;
    for (i, pos) in path.iter().enumerate() {
        // Want to save at least 100 picoseconds
        for j in (i + 100)..path.len() {
            let d = distance(pos, &path[j]) as usize;
            if d <= max_cheat_distance && j - i - d >= 100 {
                result += 1;
            }
        }
    }
    result
}

// You aren't sure what the conditions of the racetrack will be like, so to give yourself as many options as possible,
// you'll need a list of the best cheats. How many cheats would save you at least 100 picoseconds?
fn part_1(input: &str) -> usize {
    let (grid, start, end) = parse_input(input);
    let path = find_best_path(grid, start, end);
    nr_of_cheats(path, 2)
}
// Find the best cheats using the updated cheating rules. How many cheats would save you at least 100 picoseconds?
fn part_2(input: &str) -> usize {
    let (grid, start, end) = parse_input(input);
    let path = find_best_path(grid, start, end);
    nr_of_cheats(path, 20)
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_20");
    (
        Solution::from(part_1(&input)),
        Solution::from(part_2(&input)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT), 0);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2024/day_20")), 1381);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2024/day_20")), 982124)
    }
}
