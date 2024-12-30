use std::collections::HashSet;

type Position = (i32, i32);
type Garden = Vec<Vec<char>>;

fn parse_garden(input: &str) -> (Garden, i32, i32) {
    let garden: Garden = input.lines().map(|line| line.chars().collect()).collect();
    let max_x = (garden.len() - 1) as i32;
    let max_y = (garden[0].len() - 1) as i32;
    (garden, max_x, max_y)
}

// Get the neighboring positions along with the plant value on those positions if they exist
fn neighbors(
    pos: Position,
    garden: &Garden,
    max_x: i32,
    max_y: i32,
) -> Vec<(Position, Option<char>)> {
    [
        ((pos.0 > 0), (pos.0 - 1, pos.1)),     //up
        ((pos.1 < max_y), (pos.0, pos.1 + 1)), //right
        ((pos.0 < max_x), (pos.0 + 1, pos.1)), //down
        ((pos.1 > 0), (pos.0, pos.1 - 1)),     //left
    ]
    .into_iter()
    .map(|(condition, next_position)| {
        if condition {
            (
                next_position,
                Some(garden[next_position.0 as usize][next_position.1 as usize]),
            )
        } else {
            (next_position, None)
        }
    })
    .collect()
}

// Fences are placed between plants of different types.
// In other words, the peimiter is the amount of possible neighbors - nr of neighbors of the same type.
// This accounts for edge cases where there are no neighbors.
fn perimeter_of_plant(
    plant: char,
    position: Position,
    garden: &Garden,
    max_x: i32,
    max_y: i32,
) -> usize {
    let possible_neighbors = 4;
    let same_neighbors = neighbors(position, garden, max_x, max_y)
        .into_iter()
        .filter_map(|(_, next_plant)| next_plant)
        .filter(|&next_plant| next_plant == plant)
        .count();
    possible_neighbors - same_neighbors
}

// What is the total price of fencing all regions on your map?
pub fn part_1(input: &str) -> usize {
    let (garden, max_x, max_y) = parse_garden(input);
    let mut seen: HashSet<Position> = HashSet::new();
    let mut result = 0;

    for (x, row) in garden.iter().enumerate() {
        for (y, &plant) in row.iter().enumerate() {
            let position = (x as i32, y as i32);

            if seen.insert(position) {
                // Follow the neighbors that contain the same plants to detect the whole region
                let mut frontier = Vec::from([position]);
                let (mut area, mut perimeter) = (0, 0);

                while let Some(current_pos) = frontier.pop() {
                    area += 1;
                    perimeter += perimeter_of_plant(plant, current_pos, &garden, max_x, max_y);

                    for (next_pos, next_plant) in neighbors(current_pos, &garden, max_x, max_y) {
                        if next_plant.unwrap_or_default() == plant && seen.insert(next_pos) {
                            frontier.push(next_pos);
                        }
                    }
                }
                result += area * perimeter
            }
        }
    }

    result
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

    // Create the fence between two neighbors == edge orthogonal to edge between them
    // Example:
    //   (0, 0) to (0, 1)  becomes (0, 1) to (1, 1)
    //   (0, 0) to (1, 0)  becomes (1, 1) to (1, 0)
    //   (0, 0) to (0, -1) becomes (1, 0) to (0, 0)
    //   (0, 0) to (-1, 0) becomes (0, 0) to (0, 1)
    fn from_neighbors(from: (i32, i32), to: (i32, i32)) -> Edge {
        let (from, to) = if from.0 == to.0 {
            if to.1 < from.1 {
                // left
                ((from.0 + 1, from.1), from)
            } else {
                // right
                ((from.0, from.1 + 1), (from.0 + 1, from.1 + 1))
            }
        } else {
            if to.0 < from.0 {
                // up
                (from, (from.0, from.1 + 1))
            } else {
                // down
                ((from.0 + 1, from.1 + 1), (from.0 + 1, from.1))
            }
        };

        Edge::from(from, to)
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
// To get total nr of sides: build a set of edges while visiting the region and contract edges.
// Build edges by placing them around each plant in different directions. Possible situations:
//   * Plant has no same neighbors: 4 edges around the plant
//   * Plant has 1 same neighbor: 3 edges around and none between the same neighbor
//   * Plant has 2 same neighbors: 2 edges on the sides touching different neighbors
//   * Plant has 3 same neighbors: 1 edge on side touching other neighbor
//   * Plant surrounded by neighbors: no edge
//   At most 4 edges. Edges can only go on sides that touch other neighbors.
//   In other words, always an edge between a plant and a neighboring other plant.
fn area_and_sides_of_region(
    start_position: Position,
    plant: char,
    garden: &Garden,
    seen: &mut HashSet<Position>,
    max_x: i32,
    max_y: i32,
) -> (usize, usize) {
    let mut area = 0;
    let mut edges: Vec<Edge> = Vec::new();

    let mut frontier = Vec::from([start_position]);
    while let Some(current_pos) = frontier.pop() {
        if seen.insert(current_pos) {
            area += 1;

            for (next_pos, next_plant) in neighbors(current_pos, garden, max_x, max_y) {
                if next_plant.unwrap_or_default() == plant {
                    // Visit same neighbor
                    frontier.push(next_pos);
                } else {
                    // Create edge between differing neighbors
                    edges.push(Edge::from_neighbors(current_pos, next_pos));
                }
            }
        }
    }
    contract_edges(&mut edges);

    (area, edges.len())
}

// Total price if counting sides of regions instead of perimeter
pub fn part_2(input: &str) -> usize {
    let (garden, max_x, max_y) = parse_garden(input);
    let mut seen: HashSet<Position> = HashSet::new();
    let mut result = 0;

    for (x, row) in garden.iter().enumerate() {
        for (y, plant) in row.iter().enumerate() {
            let position = (x as i32, y as i32);
            if !seen.contains(&position) {
                let (area, edges) =
                    area_and_sides_of_region(position, *plant, &garden, &mut seen, max_x, max_y);
                result += area * edges;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::util::load_input;

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
        assert_eq!(part_1(EXAMPLE_INPUT_1), 140);
        assert_eq!(part_1(EXAMPLE_INPUT_2), 772);
        assert_eq!(part_1(EXAMPLE_INPUT_3), 1930);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2024/day_12")), 1550156);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1), 80);
        assert_eq!(part_2(EXAMPLE_INPUT_2), 436);
        assert_eq!(part_2(EXAMPLE_INPUT_4), 236);
        assert_eq!(part_2(EXAMPLE_INPUT_5), 368);
        assert_eq!(part_2(EXAMPLE_INPUT_3), 1206);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2024/day_12")), 946084)
    }
}
