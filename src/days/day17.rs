use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{BinaryHeap, HashSet};
use std::str::Lines;
use std::usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // Cannot turn back, so return the other three directions.
    fn next_directions(&self) -> [Self; 3] {
        match self {
            Direction::Up => [Direction::Up, Direction::Left, Direction::Right],
            Direction::Down => [Direction::Down, Direction::Left, Direction::Right],
            Direction::Left => [Direction::Up, Direction::Down, Direction::Left],
            Direction::Right => [Direction::Up, Direction::Down, Direction::Right],
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<u8>,
    height: usize,
    width: usize,
}

impl Grid {
    fn from_lines(lines: Lines) -> Self {
        let mut lines = lines.peekable();
        let width = lines.peek().map_or(0, |line| line.len());
        let grid: Vec<u8> = lines
            .flat_map(|line| {
                line.chars()
                    .map(|c| c as u8 - b'0' as u8)
                    .collect::<Vec<u8>>()
            })
            .collect();
        let height: usize = grid.len() / width;

        Grid {
            grid,
            height,
            width,
        }
    }

    // Return the next position after moving in the given direction
    fn next_position(&self, position: usize, direction: Direction) -> Option<usize> {
        match direction {
            Direction::Up if position >= self.width => Some(position - self.width),
            Direction::Down if position < self.width * (self.height - 1) => {
                Some(position + self.width)
            }
            Direction::Left if position % self.width != 0 => Some(position - 1),
            Direction::Right if (position + 1) % self.width != 0 => Some(position + 1),
            _ => None,
        }
    }

    fn at(&self, index: usize) -> u8 {
        self.grid[index]
    }

    #[allow(dead_code)]
    fn at_coordinates(&self, x: usize, y: usize) -> &u8 {
        &self.grid[x * self.width + y]
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    cost: usize,
    position: usize,
    direction: Direction,
    steps_in_direction: i32,
}

impl Move {
    fn next(
        &self,
        direction: Direction,
        grid: &Grid,
        min_steps: i32,
        max_steps: i32,
    ) -> Option<Self> {
        if self.direction == direction {
            if self.steps_in_direction >= max_steps {
                return None;
            }
        } else if self.steps_in_direction < min_steps {
            return None;
        }
        let position = grid.next_position(self.position, direction)?;
        let cost = self.cost + grid.at(position) as usize;
        let steps_in_direction = if self.direction == direction {
            self.steps_in_direction + 1
        } else {
            1
        };

        Some(Move {
            cost,
            position,
            direction,
            steps_in_direction,
        })
    }

    fn next_moves(&self, grid: &Grid, min_steps: i32, max_steps: i32) -> Vec<Self> {
        self.direction
            .next_directions()
            .into_iter()
            .flat_map(|d| self.next(d, &grid, min_steps, max_steps))
            .collect()
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

fn dijkstra<FN1, FN2, IN>(grid: &mut Grid, success: FN1, successors: FN2) -> usize
where
    FN1: Fn(&Move) -> bool,
    FN2: Fn(&Grid, &Move) -> IN,
    IN: IntoIterator<Item = Move>,
{
    let mut visited: HashSet<(usize, Direction, i32)> = HashSet::new();
    let mut frontier: BinaryHeap<Move> = BinaryHeap::from([
        Move {
            cost: 0,
            position: 0,
            direction: Direction::Down,
            steps_in_direction: 0,
        },
        Move {
            cost: 0,
            position: 0,
            direction: Direction::Right,
            steps_in_direction: 0,
        },
    ]);

    while let Some(current) = frontier.pop() {
        if success(&current) {
            return current.cost;
        }

        if visited.insert((
            current.position,
            current.direction,
            current.steps_in_direction,
        )) {
            successors(grid, &current)
                .into_iter()
                .for_each(|next| frontier.push(next));
        }
    }

    panic!("Oh no!")
}

fn part_1(lines: Lines) -> usize {
    let mut grid = Grid::from_lines(lines);
    let goal = grid.grid.len() - 1;
    let success = |current: &Move| current.position == goal;
    let successors = |grid: &Grid, m: &Move| m.next_moves(grid, 0, 3);

    dijkstra(&mut grid, success, successors)
}

fn part_2(lines: Lines) -> usize {
    let mut grid = Grid::from_lines(lines);
    let goal = grid.grid.len() - 1;
    let success = |current: &Move| current.position == goal && current.steps_in_direction >= 4;
    let successors = |grid: &Grid, m: &Move| m.next_moves(grid, 4, 10);

    dijkstra(&mut grid, success, successors)
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_17");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    const EXAMPLE_INPUT_SIMPLE: &str = "112999
911111";

    #[test]
    fn test_part_1_example_simple() {
        assert_eq!(part_1(EXAMPLE_INPUT_SIMPLE.lines()), 7);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 102);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_17").lines()), 1001);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), 94);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_17").lines()), 1197);
    }
}
