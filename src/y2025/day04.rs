use std::collections::{HashMap, HashSet};

type Position = (i32, i32);

// How many rolls of paper can be accessed?
// A roll can be accessed if there are fewer than four rolls of paper in the adjacent positions
pub fn part_1(input: &str) -> usize {
    let paper = parse_input(input);

    paper
        .iter()
        .filter(|&p| neighbors(p, &paper).iter().count() < 4)
        .count()
}

fn neighbors(pos: &Position, paper: &HashSet<Position>) -> Vec<Position> {
    [
        (pos.0 + 1, pos.1 - 1),
        (pos.0 + 1, pos.1),
        (pos.0 + 1, pos.1 + 1),
        (pos.0, pos.1 - 1),
        (pos.0, pos.1 + 1),
        (pos.0 - 1, pos.1 - 1),
        (pos.0 - 1, pos.1),
        (pos.0 - 1, pos.1 + 1),
    ]
    .into_iter()
    .filter(|&p| paper.contains(&p))
    .collect()
}

fn parse_input(input: &str) -> HashSet<Position> {
    let mut paper = HashSet::new();

    for (x, line) in input.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            if ch == '@' {
                paper.insert((x as i32, y as i32));
            }
        }
    }

    paper
}

// Once a roll of paper can be accessed, it can be removed.
// How many rolls can be removed in total?
// Instead of iterating through all positions every time to find what to remove,
// Keep track of which positions to remove next by following neighbors
// 6-7ms
pub fn part_2(input: &str) -> usize {
    let mut paper = parse_input(input);
    let mut removed = 0;
    let mut to_remove: HashSet<Position> = HashSet::new();
    let mut neighbor_counts: HashMap<Position, usize> = paper
        .iter()
        .map(|p| {
            let c = neighbors(p, &paper).iter().count();
            if c < 4 {
                to_remove.insert(*p);
            }
            (*p, c)
        })
        .collect();

    while !to_remove.is_empty() {
        removed += to_remove.len();
        for p in to_remove.iter().copied() {
            paper.remove(&p);
        }
        to_remove = to_remove
            .iter()
            .flat_map(|p| {
                neighbors(&p, &paper)
                    .into_iter()
                    .filter_map(|neighbor| {
                        let c = neighbor_counts.get_mut(&neighbor).unwrap();
                        *c -= 1;
                        if *c < 4 {
                            Some(neighbor)
                        } else {
                            None
                        }
                    })
                    .collect::<HashSet<Position>>()
            })
            .collect();
    }

    removed
}

// // Simpler approach, but  9-10 ms.
// pub fn part_2(input: &str) -> usize {
//     let mut paper = parse_input(input);
//     let mut removed = 0;

//     // Instead of iterating through all positions every time to find what to remove,
//     // Keep track of which positions to remove next by following neighbors
//     let mut to_remove: Vec<Position> = paper
//         .iter()
//         .filter(|&p| neighbors(p, &paper).iter().count() < 4)
//         .copied()
//         .collect();

//     while let Some(p) = to_remove.pop() {
//         if paper.remove(&p) {
//             removed += 1;

//             for n in neighbors(&p, &paper) {
//                 if neighbors(&n, &paper).iter().count() < 4 {
//                     to_remove.push(n);
//                 }
//             }
//         }
//     }

//     removed
// }

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

    use super::*;

    const EXAMPLE_INPUT_1: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 13);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_4")), 1518);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1), 43);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2025/day_4")), 8665);
    }
}
