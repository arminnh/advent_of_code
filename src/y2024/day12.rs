use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::str::Lines;
use std::usize;

type Position = (usize, usize);
type Garden = Vec<Vec<char>>;

fn neighbors(pos: Position, max_x: usize, max_y: usize) -> Vec<Position> {
    let mut neighbors: Vec<Position> = Vec::with_capacity(4);
    if pos.0 < max_x {
        neighbors.push((pos.0 + 1, pos.1));
    }
    if pos.1 < max_y {
        neighbors.push((pos.0, pos.1 + 1));
    }
    if pos.0 > 0 {
        neighbors.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        neighbors.push((pos.0, pos.1 - 1));
    }
    neighbors
}

fn perimeter_of_plant(
    plant: char,
    position: Position,
    garden: &Garden,
    max_x: usize,
    max_y: usize,
) -> usize {
    let mut perimeter = neighbors(position, max_x, max_y)
        .into_iter()
        .filter(|&(next_x, next_y)| garden[next_x][next_y] != plant)
        .count();
    if position.0 == 0 || position.0 == max_x {
        perimeter += 1;
    }
    if position.1 == 0 || position.1 == max_y {
        perimeter += 1;
    }
    perimeter
}

// Follow the neighbors that contain the same plants to detect the whole region
fn fill_region(
    plant: char,
    position: Position,
    garden: &Garden,
    region_positions: &mut HashMap<Position, usize>,
    region_id: usize,
    max_x: usize,
    max_y: usize,
) {
    let mut frontier = Vec::from([position]);
    while let Some(p) = frontier.pop() {
        region_positions.insert(p, region_id);
        for n in neighbors(p, max_x, max_y) {
            if garden[n.0][n.1] == plant && !region_positions.contains_key(&n) {
                frontier.push(n);
            }
        }
    }
}

// What is the total price of fencing all regions on your map?
fn part_1(lines: Lines) -> usize {
    let garden: Garden = lines.map(|line| line.chars().collect()).collect();
    let max_x = garden.len() - 1;
    let max_y = garden[0].len() - 1;
    // Keep track of which positions contain which region of plants
    let mut region_positions: HashMap<Position, usize> = HashMap::new();
    // Keep track of the size and perimiter of each region
    let mut regions: HashMap<usize, (usize, usize)> = HashMap::new();

    for x in 0..=max_x {
        for y in 0..=max_y {
            let current_plant = garden[x][y];
            let current_pos = (x, y);
            let perimeter = perimeter_of_plant(current_plant, current_pos, &garden, max_x, max_y);

            if !region_positions.contains_key(&current_pos) {
                let region_id = regions.len();
                fill_region(
                    current_plant,
                    current_pos,
                    &garden,
                    &mut region_positions,
                    region_id,
                    max_x,
                    max_y,
                );
            }

            let region_id = region_positions
                .get(&current_pos)
                .expect("Could not load region ID which should have been filled by now");
            let (area, total_perimeter) = regions.entry(*region_id).or_default();
            *area += 1;
            *total_perimeter += perimeter;
        }
    }

    regions
        .iter()
        .map(|(region_id, (size, area))| size * area)
        .sum()
}

fn part_2(lines: Lines) -> usize {
    0
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

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 140);
        assert_eq!(part_1(EXAMPLE_INPUT_2.lines()), 772);
        assert_eq!(part_1(EXAMPLE_INPUT_3.lines()), 1930);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_12").lines()), 0);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_3.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_12").lines()), 0)
    }
}
