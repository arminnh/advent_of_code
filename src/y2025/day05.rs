// How many of the available ingredient IDs are fresh?
pub fn part_1(input: &str) -> usize {
    let (ranges, ids) = input
        .split_once("\n\n")
        .expect("Could not split input in 2 parts");
    let ranges = parse_ranges(ranges);

    ids.lines()
        .map(|line| {
            line.parse::<usize>()
                .expect("Could not parse ingredient ID")
        })
        .filter(|id| is_fresh(id, &ranges))
        .count()
}

fn parse_ranges(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").expect("Could not split range on '-'");
            (
                start.parse().expect("Could not parse start of range"),
                end.parse().expect("Could not parse end of range"),
            )
        })
        .collect()
}

fn is_fresh(id: &usize, ranges: &Vec<(usize, usize)>) -> bool {
    for (start, end) in ranges {
        if start <= id && id <= end {
            return true;
        }
    }
    false
}

// How many possible ingredient IDs could be considered fresh?
pub fn part_2(input: &str) -> usize {
    let (ranges, _) = input
        .split_once("\n\n")
        .expect("Could not split input in 2 parts");
    let ranges = parse_ranges(ranges);
    let ranges = merge_overlapping_ranges(ranges);
    ranges.iter().map(|(start, end)| end - start + 1).sum()
}

fn merge_overlapping_ranges(ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut new_ranges = Vec::new();

    for (start, end) in ranges {
        if let Some(p) = new_ranges
            .iter()
            .position(|(s, e)| overlaps((start, end), (*s, *e)))
        {
            let (prev_start, prev_end) = new_ranges[p];
            new_ranges[p] = merge_ranges(prev_start, prev_end, start, end);
            new_ranges = merge_overlapping_ranges(new_ranges);
        } else {
            new_ranges.push((start, end));
        }
    }

    new_ranges
}

fn merge_ranges(prev_start: usize, prev_end: usize, start: usize, end: usize) -> (usize, usize) {
    let new_start = if prev_start < start {
        prev_start
    } else {
        start
    };
    let new_end = if prev_end < end { end } else { prev_end };
    (new_start, new_end)
}

fn overlaps(a: (usize, usize), b: (usize, usize)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

    use super::*;

    const EXAMPLE_INPUT_1: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    const EXAMPLE_INPUT_2: &str = "3-5
10-14
16-20
12-18
3-5
3-6
2-6
3-5
8-9
9-10
7-8
6-8
0-25

";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 3);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_5")), 888);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1), 14);
        assert_eq!(part_2(EXAMPLE_INPUT_2), 26);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2025/day_5")), 344378119285354);
    }
}
