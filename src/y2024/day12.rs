use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::str::Lines;

type Position = (i32, i32);
type Garden = Vec<Vec<char>>;

fn parse_garden(lines: Lines<'_>) -> (Garden, i32, i32) {
    let garden: Garden = lines.map(|line| line.chars().collect()).collect();
    let max_x = (garden.len() - 1) as i32;
    let max_y = (garden[0].len() - 1) as i32;
    (garden, max_x, max_y)
}

fn neighbors(
    pos: Position,
    garden: &Garden,
    max_x: i32,
    max_y: i32,
) -> Vec<(Position, Option<char>)> {
    let mut neighbors = Vec::with_capacity(4);
    // up
    if pos.0 > 0 {
        let up = (pos.0 - 1, pos.1);
        neighbors.push((up, Some(garden[up.0 as usize][up.1 as usize])));
    } else {
        neighbors.push(((pos.0 - 1, pos.1), None));
    }
    // right
    if pos.1 < max_y {
        let right = (pos.0, pos.1 + 1);
        neighbors.push((right, Some(garden[right.0 as usize][right.1 as usize])));
    } else {
        neighbors.push(((pos.0, pos.1 + 1), None));
    }
    // down
    if pos.0 < max_x {
        let down = (pos.0 + 1, pos.1);
        neighbors.push((down, Some(garden[down.0 as usize][down.1 as usize])));
    } else {
        neighbors.push(((pos.0 + 1, pos.1), None));
    }
    // left
    if pos.1 > 0 {
        let left = (pos.0, pos.1 - 1);
        neighbors.push((left, Some(garden[left.0 as usize][left.1 as usize])));
    } else {
        neighbors.push(((pos.0, pos.1 - 1), None));
    }
    neighbors
}

fn perimeter_of_plant(
    plant: char,
    position: Position,
    garden: &Garden,
    max_x: i32,
    max_y: i32,
) -> usize {
    let mut perimeter = neighbors(position, garden, max_x, max_y)
        .into_iter()
        .filter_map(|(_, next_plant)| next_plant)
        .filter(|&next_plant| next_plant != plant)
        .count();
    if position.0 == 0 || position.0 == max_x {
        perimeter += 1;
    }
    if position.1 == 0 || position.1 == max_y {
        perimeter += 1;
    }
    perimeter
}

// What is the total price of fencing all regions on your map?
fn part_1(lines: Lines) -> usize {
    let (garden, max_x, max_y) = parse_garden(lines);
    // Keep track of which positions contain which region of plants
    let mut region_positions: HashMap<Position, usize> = HashMap::new();
    // Keep track of the size and perimiter of each region
    let mut regions: HashMap<usize, (usize, usize)> = HashMap::new();

    for (x, row) in garden.iter().enumerate() {
        for (y, &plant) in row.iter().enumerate() {
            let position = (x as i32, y as i32);
            let perimeter = perimeter_of_plant(plant, position, &garden, max_x, max_y);

            if !region_positions.contains_key(&position) {
                // Follow the neighbors that contain the same plants to detect the whole region
                let region_id = regions.len();
                let mut frontier = Vec::from([position]);
                while let Some(p) = frontier.pop() {
                    region_positions.insert(p, region_id);
                    for (next_pos, next_plant) in neighbors(p, &garden, max_x, max_y) {
                        if next_plant.unwrap_or_default() == plant
                            && !region_positions.contains_key(&next_pos)
                        {
                            frontier.push(next_pos);
                        }
                    }
                }
            }

            let region_id = region_positions
                .get(&position)
                .expect("Could not load region ID which should have been filled by now");
            let (area, total_perimeter) = regions.entry(*region_id).or_default();
            *area += 1;
            *total_perimeter += perimeter;
        }
    }

    regions.iter().map(|(_, (size, area))| size * area).sum()
}

#[derive(Debug)]
struct Edge {
    from: Position,
    to: Position,
    is_vertical: bool,
}

impl Edge {
    fn from(pos1: Position, pos2: Position) -> Edge {
        Edge {
            from: pos1,
            to: pos2,
            is_vertical: pos1.1 == pos2.1,
        }
    }

    fn can_contract(&self, other: &Edge) -> bool {
        (self.from == other.to || self.to == other.from) && self.is_vertical == other.is_vertical
    }

    fn contract(&self, other: &Edge) -> Edge {
        if self.to == other.from {
            Edge {
                from: self.from,
                to: other.to,
                is_vertical: self.is_vertical,
            }
        } else if self.from == other.to {
            Edge {
                from: other.from,
                to: self.to,
                is_vertical: self.is_vertical,
            }
        } else {
            panic!("Edges cannot be contracted");
        }
    }
}

fn contract_edges(edges: &mut Vec<Edge>) {
    loop {
        let previous_count = edges.len();
        let mut new_edges = Vec::with_capacity(previous_count);
        while let Some(e1) = edges.pop() {
            if let Some(i) = edges.iter().position(|e2| e1.can_contract(e2)) {
                new_edges.push(e1.contract(&edges[i]));
                edges.remove(i);
            } else {
                new_edges.push(e1);
            }
        }
        *edges = new_edges;

        if edges.len() == previous_count {
            break;
        }
    }
}

// Count the size and total nr of sides of the given region.
// To get total nr of sides: build a set of edges while visiting the region and contract edges when possible. sides = nr of edges after contraction
// Build edges by placing them around each plant in different directions. Possible situations:
//   * Plant has no same neighbors: 4 edges around the plant
//   * Plant has 1 same neighbor: 3 edges around and none between the same neighbor
//   * Plant has 2 same neighbors: 2 edges on the sides touching different neighbors
//   * Plant has 3 same neighbors: 1 edge on side touching other neighbor
//   * Plant surrounded by neighbors: no edge
//   At most 4 edges. Edges can only go on sides that touch other neighbors.
//   In other words, always an edge between a plant and a neighboring other plant.
fn size_and_sides_of_region(
    start_position: Position,
    plant: char,
    garden: &Garden,
    seen: &mut HashSet<Position>,
    max_x: i32,
    max_y: i32,
) -> (usize, usize) {
    let mut size = 0;
    let mut edges: Vec<Edge> = Vec::new();

    let mut frontier = Vec::from([start_position]);
    while let Some(current_pos) = frontier.pop() {
        if seen.insert(current_pos) {
            size += 1;

            for (next_pos, next_plant) in neighbors(current_pos, garden, max_x, max_y) {
                if next_plant.unwrap_or_default() == plant {
                    frontier.push(next_pos);
                } else {
                    if current_pos.0 == next_pos.0 {
                        if next_pos.1 < current_pos.1 {
                            // left
                            edges.push(Edge::from((current_pos.0 + 1, current_pos.1), current_pos))
                        } else {
                            // right
                            edges.push(Edge::from(
                                (current_pos.0, current_pos.1 + 1),
                                (current_pos.0 + 1, current_pos.1 + 1),
                            ))
                        }
                    } else {
                        if next_pos.0 < current_pos.0 {
                            // up
                            edges.push(Edge::from(current_pos, (current_pos.0, current_pos.1 + 1)))
                        } else {
                            // down
                            edges.push(Edge::from(
                                (current_pos.0 + 1, current_pos.1 + 1),
                                (current_pos.0 + 1, current_pos.1),
                            ))
                        }
                    }
                }
            }
        }
    }
    contract_edges(&mut edges);

    (size, edges.len())
}

// Total price if counting sides of regions instead of perimeter
fn part_2(lines: Lines) -> usize {
    let (garden, max_x, max_y) = parse_garden(lines);
    let mut seen: HashSet<Position> = HashSet::new();
    let mut result = 0;

    for (x, row) in garden.iter().enumerate() {
        for (y, plant) in row.iter().enumerate() {
            let position = (x as i32, y as i32);
            if !seen.contains(&position) {
                let (size, edges) =
                    size_and_sides_of_region(position, *plant, &garden, &mut seen, max_x, max_y);
                result += size * edges;
            }
        }
    }

    result
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_12");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "AAAA
BBCD
BBCC
EEEC";

    const EXAMPLE_INPUT_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const EXAMPLE_INPUT_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const EXAMPLE_INPUT_4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const EXAMPLE_INPUT_5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 140);
        assert_eq!(part_1(EXAMPLE_INPUT_2.lines()), 772);
        assert_eq!(part_1(EXAMPLE_INPUT_3.lines()), 1930);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_12").lines()), 1550156);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 80);
        assert_eq!(part_2(EXAMPLE_INPUT_2.lines()), 436);
        assert_eq!(part_2(EXAMPLE_INPUT_4.lines()), 236);
        assert_eq!(part_2(EXAMPLE_INPUT_5.lines()), 368);
        assert_eq!(part_2(EXAMPLE_INPUT_3.lines()), 1206);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_12").lines()), 946084)
    }
}
