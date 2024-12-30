use std::collections::HashMap;
use std::usize;

type Stones = Vec<u64>;

fn parse_stones(line: &str) -> Stones {
    line.split_whitespace()
        .map(|s| s.parse().expect("Could not parse stone number"))
        .collect()
}

// Old blink for part 1
// fn blink(stones: Stones) -> Stones {
//     let mut new_stones = Vec::with_capacity(stones.len() * 2);
//     for stone in stones {
//         if stone == 0 {
//             new_stones.push(1);
//         } else {
//             let nr_of_digits = stone.ilog10() + 1;
//             if nr_of_digits % 2 == 0 {
//                 let divisor = 10_u64.pow(nr_of_digits / 2);
//                 let (left, right) = (stone / divisor, stone % divisor);
//                 new_stones.push(left);
//                 new_stones.push(right);
//             } else {
//                 new_stones.push(stone * 2024)
//             }
//         }
//     }
//     new_stones
// }

fn blink(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        (1, None)
    } else {
        let nr_of_digits = stone.ilog10() + 1;
        let (half_of_digits, remainder) = (nr_of_digits / 2, nr_of_digits % 2);

        if remainder == 0 {
            let divisor = 10_u64.pow(half_of_digits);
            let (left, right) = (stone / divisor, stone % divisor);
            (left, Some(right))
        } else {
            (stone * 2024, None)
        }
    }
}

fn blink_stone_n_times_recursive(
    stone: u64,
    n: usize,
    seen: &mut HashMap<(u64, usize), usize>,
) -> usize {
    fn recurse(seen: &mut HashMap<(u64, usize), usize>, stone: u64, n: usize) -> usize {
        if let Some(count) = seen.get(&(stone, n)) {
            *count
        } else {
            blink_stone_n_times_recursive(stone, n, seen)
        }
    }

    if n == 0 {
        seen.insert((stone, 0), 1);
        1
    } else {
        let (left, right) = blink(stone);

        let left_count = recurse(seen, left, n - 1);
        let right_count = if let Some(right) = right {
            recurse(seen, right, n - 1)
        } else {
            0
        };

        let result = left_count + right_count;
        seen.insert((stone, n), result);
        result
    }
}

#[allow(dead_code)]
fn blink_stones_n_times_recursive(stones: Stones, n: usize) -> usize {
    let mut seen: HashMap<(u64, usize), usize> = HashMap::new();
    let mut result = 0;

    for &stone in &stones {
        result += blink_stone_n_times_recursive(stone, n, &mut seen);
    }

    result
}

fn blink_stones_n_times(stones: Stones, n: usize) -> usize {
    let mut stone_counts: HashMap<u64, usize> = HashMap::new();
    for s in stones {
        *stone_counts.entry(s).or_default() += 1;
    }

    for _ in 0..n {
        for (stone, count) in stone_counts.drain().collect::<Vec<_>>() {
            let (left, right) = blink(stone);
            *stone_counts.entry(left).or_default() += count;
            if let Some(right) = right {
                *stone_counts.entry(right).or_default() += count;
            }
        }
    }

    stone_counts.values().sum()
}

// Consider the arrangement of stones in front of you. How many stones will you have after blinking 25 times?
pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(parse_stones)
        .map(|stones| blink_stones_n_times(stones, 25))
        .sum()
}

// After 75 blinks
pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(parse_stones)
        .map(|stones| blink_stones_n_times(stones, 75))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::util::load_input;

    const EXAMPLE_INPUT: &str = "125 17";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT), 55312);
    }

    #[test]
    fn test_blink() {
        assert_eq!(blink(125), (253000, None));
        assert_eq!(blink(17), (1, Some(7)));
        assert_eq!(blink(253000), (253, Some(0)));
        assert_eq!(blink(1), (2024, None));
        assert_eq!(blink(7), (14168, None));
        assert_eq!(blink(253), (512072, None));
        assert_eq!(blink(0), (1, None));
        assert_eq!(blink(2024), (20, Some(24)));
        assert_eq!(blink(14168), (28676032, None));
    }

    #[test]
    fn test_blink_stones_n_times() {
        assert_eq!(blink_stones_n_times_recursive(parse_stones("125 17"), 1), 3);
        assert_eq!(blink_stones_n_times(parse_stones("125 17"), 1), 3);
        assert_eq!(
            blink_stones_n_times_recursive(parse_stones("253000 1 7"), 1),
            4
        );
        assert_eq!(blink_stones_n_times(parse_stones("253000 1 7"), 1), 4);
        assert_eq!(
            blink_stones_n_times_recursive(parse_stones("253 0 2024 14168"), 1),
            5
        );
        assert_eq!(blink_stones_n_times(parse_stones("253 0 2024 14168"), 1), 5);
        assert_eq!(
            blink_stones_n_times_recursive(parse_stones("512072 1 20 24 28676032"), 1),
            9
        );
        assert_eq!(
            blink_stones_n_times(parse_stones("512072 1 20 24 28676032"), 1),
            9
        );
        assert_eq!(
            blink_stones_n_times_recursive(parse_stones("512 72 2024 2 0 2 4 2867 6032"), 1),
            13
        );
        assert_eq!(
            blink_stones_n_times(parse_stones("512 72 2024 2 0 2 4 2867 6032"), 1),
            13
        );
        assert_eq!(
            blink_stones_n_times_recursive(
                parse_stones("1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32"),
                1
            ),
            22
        );
        assert_eq!(
            blink_stones_n_times(
                parse_stones("1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32"),
                1
            ),
            22
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2024/day_11")), 190865);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2024/day_11")), 225404711855335);
    }
}
