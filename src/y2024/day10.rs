use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::str::Lines;
use std::usize;

type Position = (i32, i32);
type HeightMap = HashMap<Position, i32>;

fn neighbors(pos: Position) -> [Position; 4] {
    [
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
    ]
}

fn parse_input(lines: Lines) -> (HeightMap, Vec<Position>) {
    let mut topographic_map: HeightMap = HashMap::new();
    let mut start_positions: Vec<Position> = Vec::new();

    for (x, line) in lines.enumerate() {
        for (y, ch) in line.chars().enumerate() {
            let height = ch.to_digit(10).expect("Could not parse height");
            topographic_map.insert((x as i32, y as i32), height as i32);
            if height == 0 {
                start_positions.push((x as i32, y as i32));
            }
        }
    }

    (topographic_map, start_positions)
}

fn trailhead_score(topographic_map: &HeightMap, start_pos: Position) -> usize {
    let mut seen: HashSet<Position> = HashSet::new();
    let mut nines: HashSet<Position> = HashSet::new();
    let mut frontier = Vec::from([start_pos]);

    while let Some(current) = frontier.pop() {
        seen.insert(current);
        let height = *topographic_map.get(&current).unwrap();
        if height == 9 {
            nines.insert(current);
            continue;
        }
        for next in neighbors(current) {
            if let Some(next_height) = topographic_map.get(&next) {
                if next_height - height == 1 && !seen.contains(&next) {
                    frontier.push(next);
                }
            }
        }
    }

    nines.len()
}

fn part_1(lines: Lines) -> usize {
    let (topographic_map, start_positions) = parse_input(lines);
    start_positions
        .into_iter()
        .map(|pos| trailhead_score(&topographic_map, pos))
        .sum()
}

fn part_2(lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_10");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "0123
1234
8765
9876";

    const EXAMPLE_INPUT_2: &str = "5550555
5551555
5552555
6543456
7555557
8555558
9555559";

    const EXAMPLE_INPUT_3: &str = "5590559
5551598
5552557
6543456
7655987
8765555
9875555";

    const EXAMPLE_INPUT_4: &str = "1055955
2555855
3555755
4567654
5558553
5559552
5555501";

    const EXAMPLE_INPUT_5: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 1);
        assert_eq!(part_1(EXAMPLE_INPUT_2.lines()), 2);
        assert_eq!(part_1(EXAMPLE_INPUT_3.lines()), 4);
        assert_eq!(part_1(EXAMPLE_INPUT_4.lines()), 3);
        assert_eq!(part_1(EXAMPLE_INPUT_5.lines()), 36);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_10").lines()), 659);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_10").lines()), 0)
    }
}
