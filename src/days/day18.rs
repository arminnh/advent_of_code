use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashSet;
use std::str::Lines;

type Position = (i32, i32);

fn in_bounds(p: &Position, max_x: i32, max_y: i32) -> bool {
    p.0 > 0 && p.0 < max_x && p.1 > 0 && p.1 < max_y
}

fn parse_direction(s: &str) -> (i32, i32) {
    match s {
        "U" => (-1, 0),
        "D" => (1, 0),
        "L" => (0, -1),
        "R" => (0, 1),
        _ => panic!("Invalid direction {:?}", s),
    }
}

fn parse_lagoon(lines: Lines<'_>) -> HashSet<(i32, i32)> {
    let mut lagoon: HashSet<Position> = HashSet::new();

    lines.fold((0, 0), |position, line| {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [direction, count, _] => (0..count.parse::<i32>().unwrap()).fold(position, |p, _| {
                let (x_diff, y_diff) = parse_direction(direction);
                let next = (p.0 + x_diff, p.1 + y_diff);
                lagoon.insert(next);
                next
            }),
            _ => panic!("Invalid line {:?}", line),
        }
    });

    // shift all coordinates to be >= 0
    let min_x = lagoon.iter().min_by_key(|pos| pos.0).unwrap().0;
    let min_y = lagoon.iter().min_by_key(|pos| pos.1).unwrap().1;
    lagoon = lagoon.iter().map(|p| (p.0 - min_x, p.1 - min_y)).collect();
    lagoon
}

fn neighbors(p: &Position, max_x: i32, max_y: i32) -> Vec<Position> {
    vec![
        (p.0 + 1, p.1),
        (p.0 - 1, p.1),
        (p.0, p.1 + 1),
        (p.0, p.1 - 1),
    ]
    .into_iter()
    .filter(|p| in_bounds(p, max_x, max_y))
    .collect()
}

fn print_lagoon(lagoon: &HashSet<Position>, highlight: Option<&HashSet<Position>>) {
    let max_x = lagoon.iter().max_by_key(|pos| pos.0).unwrap().0;
    let max_y = lagoon.iter().max_by_key(|pos| pos.1).unwrap().1;

    for i in 0..=max_x {
        for j in 0..=max_y {
            if highlight.is_some() && highlight.unwrap().contains(&(i, j)) {
                print!("*");
            } else if lagoon.contains(&(i, j)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    println!("");
}

// Flood fill from the edges of the bounding box around the lagoon
fn flood_fill(lagoon: &HashSet<Position>) -> usize {
    let mut frontier: Vec<Position> = Vec::new();
    let mut visited: HashSet<Position> = HashSet::new();

    // fill frontier with edges of the map
    let max_x = lagoon.iter().max_by_key(|pos| pos.0).unwrap().0;
    let max_y = lagoon.iter().max_by_key(|pos| pos.1).unwrap().1;
    let left_and_right_edges = (0..=max_x).into_iter().flat_map(|i| [(i, 0), (i, max_y)]);
    let top_and_bottom_edges = (0..=max_y).into_iter().flat_map(|j| [(0, j), (max_x, j)]);
    left_and_right_edges
        .chain(top_and_bottom_edges)
        .filter(|pos| !lagoon.contains(pos))
        .for_each(|pos| frontier.push(pos));

    while let Some(current) = frontier.pop() {
        if !visited.contains(&current) {
            visited.insert(current);

            for next in neighbors(&current, max_x, max_y) {
                if !visited.contains(&next) && !lagoon.contains(&next) {
                    frontier.push(next);
                }
            }
        }
    }

    print_lagoon(&lagoon, Some(&visited));
    (max_x as usize + 1) * (max_y as usize + 1) - visited.len()
}

// Calculate the volume of the lagoon formed by the perimeter. Each position is a 1 meter cube.
fn part_1(lines: Lines) -> usize {
    let lagoon = parse_lagoon(lines);

    print_lagoon(&lagoon, None);
    flood_fill(&lagoon)
}

fn part_2(_lines: Lines) -> i32 {
    0
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_1(load_input("inputs/day_18").lines())),
        Solution::from(part_2(load_input("inputs/day_18").lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 62);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_18").lines()), 33491);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_18").lines()), 0);
    }
}
