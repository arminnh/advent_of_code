use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::ops::Range;
use std::str::Lines;
use std::usize;

struct Map {
    // List of source range mapping to destination starting point.
    // If x fits in a range, it gets mapped to the destination + x - source_range.start
    mappings: Vec<(Range<usize>, usize)>,
}

impl Map {
    fn map(&self, x: &usize) -> usize {
        for (range, destination) in self.mappings.iter() {
            if range.contains(x) {
                return destination + x - range.start;
            }
        }

        *x
    }

    fn from_lines(lines: Lines) -> Self {
        let mappings = lines
            .skip(1)
            .map(|line| {
                match line
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()[..]
                {
                    [destination_start, source_start, length] => {
                        (source_start..source_start + length, destination_start)
                    }
                    _ => panic!("Invalid line {:?}", line),
                }
            })
            .collect();

        Map { mappings }
    }
}

fn parse_seeds(splits: &str) -> Vec<usize> {
    splits
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part_1(input: &str) -> usize {
    let mut splits = input.split("\n\n");
    let seeds = parse_seeds(splits.next().unwrap());

    *splits
        .fold(seeds, |seeds, map_lines| {
            let map: Map = Map::from_lines(map_lines.lines());
            seeds.iter().map(|seed| map.map(seed)).collect()
        })
        .iter()
        .min()
        .unwrap()
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_5");
    (
        Solution::from(part_1(&input)),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 35);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/day_5")), 424490994);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_5").lines()), 0);
    }
}
