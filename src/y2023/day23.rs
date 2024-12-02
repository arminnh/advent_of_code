use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::str::Lines;
use std::usize;

type Position = (i32, i32);
type Graph = HashMap<Position, HashSet<(Position, usize)>>;

fn parse_input(lines: Lines, ignore_slopes: bool) -> (Position, Position, Graph) {
    let grid: Grid = Grid::from_lines(lines);
    let start = (0, 1);
    let goal = (grid.max_x - 1, grid.max_y - 2);
    let graph: Graph = grid_to_graph(&grid, start, goal, ignore_slopes);

    (start, goal, graph)
}

struct Grid {
    grid: Vec<u8>,
    max_x: i32,
    max_y: i32,
}

impl Grid {
    fn from_lines(lines: Lines) -> Self {
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

    fn neighbors(&self, p: &Position, ignore_slopes: bool) -> Vec<Position> {
        let mut out: Vec<Position> = Vec::new();
        let neighbors = [
            (p.0 + 1, p.1),
            (p.0 - 1, p.1),
            (p.0, p.1 + 1),
            (p.0, p.1 - 1),
        ];

        for neighbor in neighbors {
            if let Some(c) = self.at(&neighbor) {
                if c == b'#'
                    || !ignore_slopes
                    // Don't move up slippery slopes
                    && ((c == b'>' && neighbor.1 < p.1) || (c == b'v' && neighbor.0 < p.0))
                {
                    continue;
                }
                out.push(neighbor);
            };
        }

        out
    }

    #[allow(dead_code)]
    fn print(
        &self,
        visited: Option<&HashSet<Position>>,
        current: Option<Position>,
        nodes: Option<&HashSet<Position>>,
        edge_ids: Option<&HashMap<Position, (usize, usize)>>,
    ) {
        for x in 0..self.max_x {
            for y in 0..self.max_y {
                let p = (x, y);
                if current.map_or(false, |c| c == p) {
                    print!("█");
                } else if visited.map_or(false, |v| v.contains(&p)) {
                    match self.at(&p).unwrap() {
                        b'>' => print!("⯮"),
                        b'v' => print!("⯯"),
                        _ => print!("O"),
                    }
                } else if nodes.map_or(false, |n| n.contains(&p)) {
                    print!("*")
                } else if edge_ids.map_or(false, |e| e.contains_key(&p)) {
                    print!("{}", edge_ids.unwrap().get(&p).unwrap().0 % 10);
                } else if self.at(&p).unwrap() == b'#' {
                    print!(" ");
                } else {
                    print!("{}", self.at(&p).unwrap() as char);
                }
            }
            println!("");
        }
        println!("");
    }
}

fn grid_to_graph(grid: &Grid, start: Position, goal: Position, ignore_slopes: bool) -> Graph {
    let mut nodes: HashSet<Position> = HashSet::from([goal]);
    let mut edges: Vec<(Position, Position, usize)> = Vec::new();
    // For each visited position, store edge_id being explored and cost of the edge so far
    let mut visited: HashMap<Position, (usize, usize)> = HashMap::new();
    let mut frontier: Vec<(Position, Position, usize)> = Vec::from([(start, start, 0)]);

    while let Some((last_node, current, cost)) = frontier.pop() {
        // If visiting existing node again, create new edge
        if nodes.contains(&current) {
            edges.push((last_node, current, cost));
            continue;
        }

        // If visited this position for another edge -> split into two and add new one
        // Only applicable for part 1, since paths are not blocked off by slopes in part 2
        if let Some((edge_id, cost_at_pos)) = visited.insert(current, (edges.len(), cost)) {
            if !ignore_slopes && edge_id != edges.len() {
                // Register new node
                nodes.insert(current);
                let existing_edge = edges[edge_id].clone();
                // Push new edge that was just discovered
                edges.push((last_node, current, cost));
                // Push second part of existing edge
                edges.push((current, existing_edge.1, existing_edge.2 - cost_at_pos));
                // Shorten the existing edge to meet the new edge
                edges[edge_id] = (existing_edge.0, current, cost_at_pos);
                // Update costs for rest of existing path
                visited
                    .iter_mut()
                    .filter(|(_, (id, c))| id == &edge_id && c > &cost_at_pos)
                    .for_each(|(_, (id, c))| {
                        *id = edges.len() - 1;
                        *c -= cost_at_pos;
                    });
                continue;
            }
        }
        let neighbors: Vec<Position> = grid.neighbors(&current, ignore_slopes);
        let unvisited_neighbors: Vec<&Position> = neighbors
            .iter()
            .filter(|&n| !visited.contains_key(n))
            .collect();

        if unvisited_neighbors.len() > 1 {
            // fork in the road -> create edge until current position
            edges.push((last_node, current, cost));
            nodes.insert(last_node);
            nodes.insert(current);
            unvisited_neighbors.into_iter().for_each(|&p| {
                frontier.push((current, p, 1));
            });
        } else {
            // continue exploring to unvisited positions, to nodes, or to positions covered by another edge
            let f = |pos: &Position| -> bool {
                *pos != last_node
                    && (nodes.contains(pos)
                        || visited.get(pos).map_or(true, |(id, _)| id != &edges.len()))
            };
            neighbors.into_iter().filter(|n| f(n)).for_each(|p| {
                frontier.push((last_node, p, cost + 1));
            });
        }
        // grid.print(None, Some(current), Some(&nodes), Some(&visited));
    }
    // grid.print(None, None, Some(&nodes), Some(&visited));

    let mut graph: Graph = HashMap::new();
    edges.into_iter().for_each(|(from, to, c)| {
        graph.entry(from).or_default().insert((to, c));
        if ignore_slopes {
            graph.entry(to).or_default().insert((from, c));
        }
    });
    graph
}

fn find_longest_path(
    graph: &Graph,
    current: Position,
    cost: usize,
    goal: Position,
    visited: &mut HashSet<Position>,
) -> usize {
    visited.insert(current);
    let mut max_cost = cost;

    if let Some(successors) = graph.get(&current) {
        successors.into_iter().for_each(|&(next, c)| {
            if !visited.contains(&next) {
                let new_cost = cost + find_longest_path(graph, next, c, goal, visited);
                max_cost = std::cmp::max(max_cost, new_cost);
            }
        });
    }

    visited.remove(&current);
    max_cost
}

fn part_1(lines: Lines) -> usize {
    let ignore_slopes = false;
    let (start, goal, graph): (Position, Position, Graph) = parse_input(lines, ignore_slopes);
    find_longest_path(&graph, start, 0, goal, &mut HashSet::new())
}

fn part_2(lines: Lines) -> usize {
    let ignore_slopes = true;
    let (start, goal, graph): (Position, Position, Graph) = parse_input(lines, ignore_slopes);
    find_longest_path(&graph, start, 0, goal, &mut HashSet::new())
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2023/day_23");
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
        assert_eq!(part_1(load_input("inputs/2023/day_23").lines()), 2334);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 154);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2023/day_23").lines()), 6422);
    }
}
