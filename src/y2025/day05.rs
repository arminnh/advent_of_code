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

// ...
pub fn part_2(input: &str) -> usize {
    0
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

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 3);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_5")), 0);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT_1), 0);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(&load_input("inputs/2025/day_5")), 0);
    // }
}
