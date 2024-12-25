use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::usize;

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for block in input.split("\n\n") {
        let block: Vec<Vec<char>> = block.lines().map(|line| line.chars().collect()).collect();
        let is_lock = block[0][0] == '#';
        let heights: Vec<usize> = (0..block[0].len())
            .map(|col| {
                (0..block.len())
                    .filter(|row| block[*row][col] == '#')
                    .count()
                    - 1
            })
            .collect();
        if is_lock {
            locks.push(heights);
        } else {
            keys.push(heights);
            // keys.push(heights.iter().map(|height| 5 - height).collect());
        }
    }

    (locks, keys)
}

fn part_1(input: &str) -> usize {
    let (locks, keys) = parse_input(input);
    let mut matches = 0;
    for lock in locks {
        for key in &keys {
            if lock.iter().zip(key.iter()).all(|(l, k)| l + k < 6) {
                matches += 1;
            }
        }
    }
    matches
}

fn part_2(input: &str) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_25");
    (
        Solution::from(part_1(&input)),
        Solution::from(part_2(&input)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT), 3);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2024/day_25")),3508);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2024/day_25")), 0)
    }
}
