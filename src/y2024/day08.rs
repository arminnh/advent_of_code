use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::str::Lines;
use std::usize;

type Position = (i32, i32);

fn parse_input(lines: Lines) -> (HashMap<char, Vec<Position>>, i32, i32) {
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (x, line) in lines.enumerate() {
        for (y, c) in line.char_indices() {
            if c != '.' {
                antennas.entry(c).or_default().push((x as i32, y as i32));
            }

            max_y = max_y.max((y + 1) as i32);
        }
        max_x = max_x.max((x + 1) as i32);
    }
    (antennas, max_x, max_y)
}

fn find_closest_antinodes(pos1: Position, pos2: Position) -> [Position; 2] {
    let dx = pos1.0 - pos2.0;
    let dy = pos1.1 - pos2.1;

    [(pos1.0 + dx, pos1.1 + dy), (pos2.0 - dx, pos2.1 - dy)]
}

fn in_bounds(pos: Position, max_x: i32, max_y: i32) -> bool {
    pos.0 >= 0 && pos.0 < max_x && pos.1 >= 0 && pos.1 < max_y
}

fn find_all_antinodes(positions: Vec<Position>, max_x: i32, max_y: i32) -> Vec<Position> {
    let mut antinodes = Vec::new();

    for (i, (x1, y1)) in positions.iter().enumerate() {
        for (x2, y2) in &positions[i + 1..] {
            let (dx, dy) = (x1 - x2, y1 - y2);

            let mut next_pos = (x1 + dx, y1 + dy);
            while in_bounds(next_pos, max_x, max_y) {
                antinodes.push(next_pos);
                next_pos = (next_pos.0 + dx, next_pos.1 + dy);
            }

            next_pos = (x2 - dx, y2 - dy);
            while in_bounds(next_pos, max_x, max_y) {
                antinodes.push(next_pos);
                next_pos = (next_pos.0 - dx, next_pos.1 - dy);
            }
        }
    }

    antinodes
}

fn part_1(lines: Lines) -> usize {
    let (antennas, max_x, max_y) = parse_input(lines);
    let mut antinodes: HashSet<Position> = HashSet::new();

    for (_, positions) in antennas {
        for (i, &pos1) in positions.iter().enumerate() {
            for &pos2 in &positions[i + 1..] {
                let [first, second] = find_closest_antinodes(pos1, pos2);
                if in_bounds(first, max_x, max_y) {
                    antinodes.insert(first);
                }
                if in_bounds(second, max_x, max_y) {
                    antinodes.insert(second);
                }
            }
        }
    }

    antinodes.len()
}

fn part_2(lines: Lines) -> usize {
    let (antennas, max_x, max_y) = parse_input(lines);

    antennas
        .into_iter()
        .filter(|(_, positions)| positions.len() > 1)
        .flat_map(|(_, positions)| {
            positions
                .clone()
                .into_iter()
                .chain(find_all_antinodes(positions, max_x, max_y))
                .collect::<Vec<_>>()
        })
        .collect::<HashSet<_>>()
        .len()
}
pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_8");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........";

    const EXAMPLE_INPUT_2: &str = "..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........";

    const EXAMPLE_INPUT_3: &str = "..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........";

    const EXAMPLE_INPUT_4: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    const EXAMPLE_INPUT_5: &str = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 2);
        assert_eq!(part_1(EXAMPLE_INPUT_2.lines()), 4);
        assert_eq!(part_1(EXAMPLE_INPUT_3.lines()), 4);
        assert_eq!(part_1(EXAMPLE_INPUT_4.lines()), 14);
    }

    #[test]
    fn test_find_antinode() {
        assert_eq!(find_closest_antinodes((3, 4), (5, 5)), [(1, 3), (7, 6)]);
        assert_eq!(find_closest_antinodes((3, 4), (4, 8)), [(2, 0), (5, 12)]);
        assert_eq!(find_closest_antinodes((5, 5), (4, 8)), [(6, 2), (3, 11)]);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_8").lines()), 254);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_5.lines()), 9);
        assert_eq!(part_2(EXAMPLE_INPUT_4.lines()), 34);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_8").lines()), 951)
    }
}
