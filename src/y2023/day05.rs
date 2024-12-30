use std::ops::Range;
use std::str::Lines;
use std::{cmp, usize};

struct Map {
    // List of source range mapping to destination starting point.
    // If x fits in a range, it gets mapped to the destination + x - source_range.start
    mappings: Vec<(Range<usize>, usize)>,
}

impl Map {
    fn map(&self, x: &usize) -> usize {
        for (range, destination) in self.mappings.iter() {
            if range.contains(&(x - 1)) {
                return destination + x - range.start;
            }
        }

        *x
    }

    fn from_lines(lines: Lines) -> Self {
        let mut mappings: Vec<(Range<usize>, usize)> = lines
            .skip(1)
            .map(|line| {
                match line
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()[..]
                {
                    [destination_start, source_start, length] => {
                        (source_start..source_start + length - 1, destination_start)
                    }
                    _ => panic!("Invalid line {:?}", line),
                }
            })
            .collect();
        mappings.sort_by_key(|(source, _)| source.start);

        Map { mappings }
    }

    fn map_range(&self, range: Range<usize>) -> Vec<Range<usize>> {
        let mut result = Vec::new();
        let relevant_mappings: Vec<&(Range<usize>, usize)> = self
            .mappings
            .iter()
            .filter(|(source, _)| source.start <= range.end && source.end >= range.start)
            .collect();

        if relevant_mappings.len() == 0 {
            return vec![range];
        }

        for (i, (source, destination)) in relevant_mappings.iter().enumerate() {
            if i == 0 && range.start < source.start {
                result.push(range.start..source.start - 1);
            }
            let new_range_start = cmp::max(range.start, source.start) + destination - source.start;
            let new_range_end = cmp::min(range.end, source.end) + destination - source.start;
            result.push(new_range_start..new_range_end);
            if i == relevant_mappings.len() - 1 && source.end < range.end {
                result.push(source.end + 1..range.end);
            }
        }
        result
    }
}

fn parse_seeds(splits: &str) -> Vec<usize> {
    splits
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_maps(splits: std::str::Split<'_, &str>) -> Vec<Map> {
    splits.map(|map| Map::from_lines(map.lines())).collect()
}

pub fn part_1(input: &str) -> usize {
    let mut splits = input.split("\n\n");
    let seeds: Vec<usize> = parse_seeds(splits.next().unwrap());
    let maps: Vec<Map> = parse_maps(splits);

    *maps
        .iter()
        .fold(seeds, |seeds, m| seeds.iter().map(|x| m.map(x)).collect())
        .iter()
        .min()
        .unwrap()
}

pub fn part_2(input: &str) -> usize {
    let mut splits = input.split("\n\n");
    let seeds: Vec<usize> = parse_seeds(splits.next().unwrap());
    let seed_ranges: Vec<Range<usize>> = seeds
        .chunks_exact(2)
        .map(|seeds| (seeds[0]..seeds[0] + seeds[1] - 1))
        .collect();
    let maps: Vec<Map> = parse_maps(splits);

    maps.iter()
        .fold(seed_ranges, |seed_ranges, m| {
            seed_ranges
                .into_iter()
                .flat_map(|range| m.map_range(range))
                .collect()
        })
        .iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::util::load_input;

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

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
        assert_eq!(part_1(EXAMPLE_INPUT), 35);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2023/day_5")), 424490994);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT), 46);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2023/day_5")), 15290096);
    }
}
