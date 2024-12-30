use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;
use std::str::Lines;
use std::usize;

type Position = (i32, i32);
type Path = Vec<Position>;
type Obstacles = HashSet<Position>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Move {
    cost: usize,
    position: Position,
    path: Path,
}

fn in_bounds((x, y): Position, max_x: i32, max_y: i32) -> bool {
    x >= 0 && y >= 0 && x <= max_x && y <= max_y
}

fn next_moves(obstacles: &Obstacles, current_move: &Move, max_x: i32, max_y: i32) -> Vec<Move> {
    let mut next = Vec::new();
    for (dx, dy) in [((-1, 0)), ((0, 1)), ((1, 0)), ((0, -1))] {
        let pos = (current_move.position.0 + dx, current_move.position.1 + dy);
        if in_bounds(pos, max_x, max_y) && !obstacles.contains(&pos) {
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
    obstacles: &Obstacles,
    start: Move,
    success: FN1,
    successors: FN2,
) -> Option<(usize, Path)>
where
    FN1: Fn(&Move) -> bool,
    FN2: Fn(&Obstacles, &Move) -> IN,
    IN: IntoIterator<Item = Move>,
{
    let mut visited: HashSet<Position> = HashSet::new();
    let mut frontier: BinaryHeap<Reverse<(usize, Move)>> = BinaryHeap::from([Reverse((0, start))]);

    while let Some(Reverse((total_cost, current))) = frontier.pop() {
        if success(&current) {
            return Some((total_cost, current.path));
        }

        if visited.insert(current.position) {
            for next in successors(obstacles, &current) {
                frontier.push(Reverse((total_cost + next.cost, next)));
            }
        }
    }

    None
}

fn parse_input(input: &str) -> (Vec<(i32, i32)>, i32, i32) {
    let (mut max_x, mut max_y) = (0, 0);
    let bytes: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let (y, x) = line.split_once(",").expect("Could not split coordinate");
            let x = x.parse::<i32>().expect("Could not parse X");
            let y = y.parse::<i32>().expect("Could not parse Y");
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            (x, y)
        })
        .collect();
    (bytes, max_x, max_y)
}

// Return either the cost to reach the end after consuming n bytes,
// or the position of the last consumed byte which makes the end unreachable
fn solve_day(
    input: &str,
    bytes_to_consume: usize,
    consume_until_end_unreachable: bool,
) -> (Option<usize>, Option<Position>) {
    let (bytes, max_x, max_y) = parse_input(input);
    let mut bytes_iter = bytes.into_iter();
    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
    for _ in 0..bytes_to_consume {
        obstacles.insert(bytes_iter.next().unwrap());
    }

    let do_dijkstra = |obstacles: &HashSet<(i32, i32)>| {
        let start = Move {
            position: (0, 0),
            cost: 0,
            path: vec![(0, 0)],
        };
        dijkstra(
            obstacles,
            start,
            |m| m.position == (max_x, max_y),
            |o, m| next_moves(o, &m, max_x, max_y),
        )
    };

    let mut last_path = Vec::new();
    if let Some((cost, path)) = do_dijkstra(&obstacles) {
        last_path = path;
        if !consume_until_end_unreachable {
            // If we don't need to consume bytes, can just return the cost to reach the end
            return (Some(cost), None);
        }
    }

    // Keep inserting more bytes as obstacles while possible
    while let Some(byte) = bytes_iter.next() {
        obstacles.insert(byte);
        // Only need to recompute path if the new byte lies on the last best path
        if last_path.iter().find(|p| **p == byte).is_some() {
            if let Some((_, path)) = do_dijkstra(&obstacles) {
                last_path = path;
            } else {
                // End is unreachable now, return the evil byte
                return (None, Some((byte.1, byte.0)));
            }
        }
    }

    panic!("No solution")
}

pub fn part_1(input: &str) -> usize {
    solve_day(input, 1024, false).0.unwrap()
}

pub fn part_2(input: &str) -> String {
    let position = solve_day(input, 1024, true).1.unwrap();
    format!("{},{}", position.0, position.1)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::util::load_input;

    const EXAMPLE_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            solve_day(EXAMPLE_INPUT, 12, false),
            (Some(22), None)
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2024/day_18")), 404);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(
            solve_day(EXAMPLE_INPUT, 12, true),
            (None, Some((6, 1)))
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2024/day_18")), "27,60")
    }
}
