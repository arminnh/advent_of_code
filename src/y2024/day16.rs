use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{BinaryHeap, HashSet};
use std::str::Lines;
use std::usize;

type Grid = Vec<Vec<char>>;
type Position = (i32, i32);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    cost: usize,
    position: Position,
    direction: Direction,
}

impl Move {
    fn next_moves(&self, grid: &Grid) -> Vec<Self> {
        let mut next = Vec::new();
        for ((dx, dy), next_direction) in [
            ((-1, 0), Direction::North),
            ((0, 1), Direction::East),
            ((1, 0), Direction::South),
            ((0, -1), Direction::West),
        ] {
            let (next_x, next_y) = (self.position.0 + dx, self.position.1 + dy);
            if self.direction.opposite() != next_direction
                && grid[next_x as usize][next_y as usize] != '#'
            {
                let next_cost = if self.direction == next_direction {
                    1
                } else {
                    1001
                };
                next.push(Move {
                    cost: self.cost + next_cost,
                    position: (next_x, next_y),
                    direction: next_direction,
                })
            }
        }
        next
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // BinaryHeap is a max-heap so reverse the comparison
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(lines: Lines) -> (Grid, Position, Position) {
    let g: Grid = lines.map(|line| line.chars().collect()).collect();
    let mut start = (-1, -1);
    let mut goal = (-1, -1);

    for (x, row) in g.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                start = (x as i32, y as i32);
            }
            if *cell == 'E' {
                goal = (x as i32, y as i32);
            }
        }
    }
    (g, start, goal)
}

fn dijkstra<FN1, FN2, IN>(grid: &Grid, start: Move, success: FN1, successors: FN2) -> usize
where
    FN1: Fn(&Move) -> bool,
    FN2: Fn(&Grid, &Move) -> IN,
    IN: IntoIterator<Item = Move>,
{
    let mut visited: HashSet<(Position, Direction)> = HashSet::new();
    let mut frontier: BinaryHeap<Move> = BinaryHeap::from([start]);

    while let Some(current) = frontier.pop() {
        println!("{:?}", current);
        if success(&current) {
            return current.cost;
        }

        if visited.insert((current.position, current.direction)) {
            for next in successors(grid, &current) {
                frontier.push(next);
            }
        }
    }

    panic!("Oh no!")
}

fn part_1(lines: Lines) -> usize {
    let (grid, start, goal) = parse_input(lines);
    println!("Start: {:?}, Goal: {:?}", start, goal);

    let success = |current: &Move| current.position == goal;
    let successors = |grid: &Grid, m: &Move| m.next_moves(grid);
    let start_move = Move {
        cost: 0,
        position: start,
        direction: Direction::East,
    };
    dijkstra(&grid, start_move, success, successors)
}

fn part_2(lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_16");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EXAMPLE_INPUT_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 7036);
        assert_eq!(part_1(EXAMPLE_INPUT_2.lines()), 11048);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_16").lines()), 0);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_16").lines()), 0)
    }
}
