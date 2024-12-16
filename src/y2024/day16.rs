use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Move {
    cost: usize,
    position: Position,
    direction: Direction,
}

fn next_moves(grid: &Grid, (x, y): Position, dir: Direction) -> Vec<Move> {
    let mut next = Vec::new();
    for ((dx, dy), next_direction) in [
        ((-1, 0), Direction::North),
        ((0, 1), Direction::East),
        ((1, 0), Direction::South),
        ((0, -1), Direction::West),
    ] {
        let (next_x, next_y) = (x + dx, y + dy);
        if dir.opposite() != next_direction && grid[next_x as usize][next_y as usize] != '#' {
            next.push(Move {
                cost: if dir == next_direction { 1 } else { 1001 },
                position: (next_x, next_y),
                direction: next_direction,
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

fn parse_input(lines: Lines) -> (Grid, Position, Position) {
    let g: Grid = lines.map(|line| line.chars().collect()).collect();
    let mut start = None;
    let mut goal = None;

    for (x, row) in g.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if *cell == 'S' {
                start = Some((x as i32, y as i32));
            }
            if *cell == 'E' {
                goal = Some((x as i32, y as i32));
            }
        }
    }
    (g, start.unwrap(), goal.unwrap())
}

fn dijkstra<FN1, FN2, IN>(grid: &Grid, start: Move, success: FN1, successors: FN2) -> usize
where
    FN1: Fn(&Move) -> bool,
    FN2: Fn(&Grid, &Move) -> IN,
    IN: IntoIterator<Item = Move>,
{
    let mut visited: HashSet<Move> = HashSet::new();
    let mut frontier: BinaryHeap<Reverse<(usize, Move)>> = BinaryHeap::from([Reverse((0, start))]);

    while let Some(Reverse((total_cost, current))) = frontier.pop() {
        if success(&current) {
            return total_cost;
        }

        if visited.insert(current) {
            for next in successors(grid, &current) {
                frontier.push(Reverse((total_cost + next.cost, next)));
            }
        }
    }

    panic!("Oh no!")
}

// What is the lowest score a Reindeer could possibly get?
fn part_1(lines: Lines) -> usize {
    let (grid, start, goal) = parse_input(lines);

    let success = |current: &Move| current.position == goal;
    let successors = |grid: &Grid, m: &Move| next_moves(grid, m.position, m.direction);
    let start_move = Move {
        cost: 0,
        position: start,
        direction: Direction::East,
    };
    dijkstra(&grid, start_move, success, successors)
}

// Get all cells that lie on one of the possible best paths
fn all_cells_on_a_best_path(grid: &Grid, start: Position, goal: Position) -> HashSet<Position> {
    // For each position, keep the lowest cost to reach it and the preceding move(s) that lead to it
    let mut best: HashMap<Position, (usize, Vec<Move>)> = HashMap::new();
    let mut visited: HashSet<Move> = HashSet::new();
    let start_move = Move {
        cost: 0,
        position: start,
        direction: Direction::East,
    };
    let mut frontier: BinaryHeap<Reverse<(usize, Move)>> =
        BinaryHeap::from([Reverse((0, start_move))]);

    while let Some(Reverse((total_cost, current))) = frontier.pop() {
        if !visited.insert(current) {
            continue;
        }
        // !("{}, {:?}", total_cost, current);

        for next in next_moves(grid, current.position, current.direction) {
            let new_cost = total_cost + next.cost;
            // println!("\t next cost: {}, {:?}", new_cost, next);
            frontier.push(Reverse((new_cost, next)));

            if let Some((previous_cost, _)) = best.get(&next.position) {
                if new_cost < *previous_cost {
                    best.insert(next.position, (new_cost, vec![current]));
                } else if next_moves(grid, next.position, next.direction)
                    .iter()
                    .filter_map(|n| best.get(&n.position))
                    .find(|(cost, _)| *cost == new_cost + 1)
                    .is_some()
                {
                    // In case the current position of the best path is making an expensive turn,
                    // peek ahead and see if the cost evens out again on the turn after.
                    best.get_mut(&next.position).unwrap().1.push(current);
                }
            } else {
                best.insert(next.position, (new_cost, vec![current]));
            }
        }
    }

    // Follow the best predecessor(s) for each position starting from the end
    let mut output: HashSet<Position> = HashSet::from([start, goal]);
    let mut reverse_frontier = vec![goal];
    while let Some(current) = reverse_frontier.pop() {
        if let Some((_, predecessors)) = best.get(&current) {
            for prev_move in predecessors {
                // println!("{:?}: {} -> {:?}", current, cost, prev_move);
                if prev_move.position != start && output.insert(prev_move.position) {
                    reverse_frontier.push(prev_move.position);
                }
            }
        }
    }

    output
}

fn display_grid(grid: &Grid, best_spots: &HashSet<Position>) {
    for (x, row) in grid.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if best_spots.contains(&(x as i32, y as i32)) {
                print!("@");
            } else if *cell == '#' {
                print!("▒");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

// How many tiles are part of at least one of the best paths through the maze?
fn part_2(lines: Lines) -> usize {
    let (grid, start, goal) = parse_input(lines);
    let cells = all_cells_on_a_best_path(&grid, start, goal);
    // display_grid(&grid, &cells);
    cells.len()
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

    const EXAMPLE_INPUT_3: &str = "#################
##.............E#
##.#.###.#####.##
##.#.#...#.....##
##.#.#.###.#.#.##
#S.............##
#################
";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 7036);
        assert_eq!(part_1(EXAMPLE_INPUT_2.lines()), 11048);
        assert_eq!(part_1(EXAMPLE_INPUT_3.lines()), 2018);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_16").lines()), 93436);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 45);
        assert_eq!(part_2(EXAMPLE_INPUT_2.lines()), 64);
        assert_eq!(part_2(EXAMPLE_INPUT_3.lines()), 37);
    }

    #[test]
    fn test_part_2() {
        // 437, 465 too low, 507 too high
        // 487 incorrect, extra step next to start pos was included in reverse path construction
        assert_eq!(part_2(load_input("inputs/2024/day_16").lines()), 486)
    }
}
