use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::str::Lines;
use std::usize;

type Position = (i32, i32);
type Grid = HashSet<(i32, i32)>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Move {
    cost: usize,
    position: Position,
}

fn next_moves(grid: &Grid, (x, y): Position) -> Vec<Move> {
    let mut next = Vec::new();
    for (dx, dy) in [((-1, 0)), ((0, 1)), ((1, 0)), ((0, -1))] {
        let (next_x, next_y) = (x + dx, y + dy);
        if next_x >= 0 && next_y >= 0 && next_x <= 70 && next_y <= 70 && !grid.contains(&(next_x, next_y)) {
            next.push(Move {
                cost: 1,
                position: (next_x, next_y),
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

fn dijkstra<FN1, FN2, IN>(grid: &Grid, start: Move, success: FN1, successors: FN2) -> Option<usize>
where
    FN1: Fn(&Move) -> bool,
    FN2: Fn(&Grid, &Move) -> IN,
    IN: IntoIterator<Item = Move>,
{
    let mut visited: HashSet<Move> = HashSet::new();
    let mut frontier: BinaryHeap<Reverse<(usize, Move)>> = BinaryHeap::from([Reverse((0, start))]);

    while let Some(Reverse((total_cost, current))) = frontier.pop() {
        if success(&current) {
            return Some(total_cost);
        }

        if visited.insert(current) {
            for next in successors(grid, &current) {
                frontier.push(Reverse((total_cost + next.cost, next)));
            }
        }
    }

    None
}

fn display_grid(grid: &Grid) {
    for x in 0..=70 {
        for y in 0..=70 {
            if grid.contains(&(x as i32, y as i32)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn part_1(lines: Lines) -> usize {
    let (mut max_x, mut max_y) = (0, 0);
    let bytes: Vec<(i32, i32)> = lines
        .map(|line| {
            let (y, x) = line.split_once(",").unwrap();
            let x = x.parse::<i32>().expect("Could not parse X");
            let y = y.parse::<i32>().expect("Could not parse Y");
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            (x, y)
        })
        .collect();

    let positions: HashSet<(i32, i32)> = bytes.into_iter().take(1024).collect();
    println!("{:?}", positions);
    println!("{:?}", (max_x, max_y));

    let start_move = Move {
        position: (0, 0),
        cost: 0,
    };
    display_grid(&positions);
    if let Some(result) = dijkstra(
        &positions,
        start_move,
        |m| m.position == (max_x, max_y),
        |g, m| next_moves(g, m.position),
    ) {
        result
    } else {
        0
    }
}

fn part_2(lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_18");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 22);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_18").lines()), 404);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_18").lines()), 0)
    }
}
