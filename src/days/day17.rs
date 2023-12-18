use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{BinaryHeap, HashSet};
use std::str::Lines;
use std::usize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position(usize, usize);

impl Position {
    // Return the next position after moving in the direction.
    fn move_in_direction(
        &self,
        direction: Direction,
        max_x: usize,
        max_y: usize,
    ) -> Option<Position> {
        match direction {
            Direction::Up if self.0 > 0 => Some(Position(self.0 - 1, self.1)),
            Direction::Down if self.0 < max_x - 1 => Some(Position(self.0 + 1, self.1)),
            Direction::Left if self.1 > 0 => Some(Position(self.0, self.1 - 1)),
            Direction::Right if self.1 < max_y - 1 => Some(Position(self.0, self.1 + 1)),
            _ => None,
        }
    }
}

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
struct Node {
    cost: usize,
    total_cost: usize,
    previous: Option<Position>,
}

struct Grid {
    grid: Vec<Vec<Node>>,
    max_x: usize,
    max_y: usize,
}

impl Grid {
    fn from_lines(lines: Lines<'_>) -> Self {
        let grid: Vec<Vec<Node>> = lines
            .map(|line| {
                line.chars()
                    .map(|c| Node {
                        cost: c as usize - b'0' as usize,
                        total_cost: usize::MAX,
                        previous: None,
                    })
                    .collect::<Vec<Node>>()
            })
            .collect();

        let max_x = grid.len();
        let max_y = grid[0].len();
        Grid { grid, max_x, max_y }
    }

    fn find_solution(&self) -> HashSet<Position> {
        let mut path: HashSet<Position> = HashSet::new();

        let mut current = self.grid.last().unwrap().last().unwrap();
        while let Some(pos) = current.previous {
            if path.insert(pos) {
                current = &self.grid[pos.0][pos.1];
            } else {
                break;
            }
        }

        path
    }

    fn print_solution(&self) {
        let path = self.find_solution();
        self.grid.iter().enumerate().for_each(|(x, row)| {
            row.iter().enumerate().for_each(|(y, node)| {
                if path.contains(&Position(x, y)) {
                    print!("•");
                } else {
                    print!("{}", node.cost);
                }
            });
            println!();
        });
    }

    fn at(&self, x: usize, y: usize) -> &Node {
        &self.grid[x][y]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut Node {
        &mut self.grid[x][y]
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    cost: usize,
    position: Position,
    direction: Direction,
    steps_in_direction: i32,
}

impl Move {
    fn next(&self, direction: Direction, grid: &Grid) -> Option<Self> {
        if self.direction == direction && self.steps_in_direction >= 3 {
            return None;
        }
        let position = self
            .position
            .move_in_direction(direction, grid.max_x, grid.max_y)?;
        let cost = self.cost + grid.at(position.0, position.1).cost;
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

    fn next_moves(&self, grid: &Grid) -> Vec<Self> {
        self.direction
            .next_directions()
            .iter()
            .flat_map(|d| self.next(*d, &grid))
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

fn dijkstra<FN, IN>(grid: &mut Grid, goal: Position, successors: FN) -> usize
where
    FN: Fn(&Grid, &Move) -> IN,
    IN: IntoIterator<Item = Move>,
{
    let mut visited: HashSet<(Position, Direction, i32)> = HashSet::new();
    let mut frontier: BinaryHeap<Move> = BinaryHeap::from([Move {
        cost: 0,
        position: Position(0, 0),
        direction: Direction::Down,
        steps_in_direction: 0,
    }]);

    while let Some(current) = frontier.pop() {
        // println!("{:?}", current);
        if current.position == goal {
            return current.cost;
        }

        if !visited.insert((
            current.position,
            current.direction,
            current.steps_in_direction,
        )) {
            // println!("skipping");
            continue;
        }

        for next in successors(grid, &current) {
            let node = grid.at_mut(next.position.0, next.position.1);
            if next.cost < node.total_cost {
                node.total_cost = next.cost;
                node.previous = Some(current.position);
                // println!("   {:?}, {:?}", next.position, node);
            }
            frontier.push(next);
        }
    }

    panic!("Oh no!")
}

fn part_1(lines: Lines) -> usize {
    let mut grid = Grid::from_lines(lines);
    let goal = Position(grid.max_x - 1, grid.max_y - 1);

    let cost = dijkstra(&mut grid, goal, |grid: &Grid, m: &Move| m.next_moves(grid));
    grid.print_solution();
    cost
}

fn part_2(_lines: Lines) -> usize {
    0
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
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_17").lines()), 0);
    }
}
