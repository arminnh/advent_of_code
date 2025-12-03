use std::collections::HashSet;

// The sum of the maximum joltage of each battery bank
pub fn part_1(input: &str) -> u32 {
    input.lines().map(|line| maximum_joltage(line)).sum()
}

// Find the largest number formed by two digits
fn maximum_joltage(battery_bank: &str) -> u32 {
    let mut left: char = '0';
    let mut right: char = '0';
    let len = battery_bank.len();

    for (i, c) in battery_bank.char_indices() {
        if i == len - 1 {
            if c > right {
                right = c;
            }
        } else {
            if c > left {
                left = c;
                right = '0';
            } else if c > right {
                right = c;
            }
        }
    }

    left.to_digit(10).unwrap() * 10 + right.to_digit(10).unwrap()
}

// Create the largest possible joltage by turning on exactly 12 batteries
pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            max_number_of_n_digits(
                &line
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>(),
                12,
            )
        })
        .sum()
}

// Form the largest possible number by taking N digits in the collection from left to right
fn max_number_of_n_digits(battery_bank: &[u32], digits: usize) -> usize {
    let len = battery_bank.len();
    // Find all positions for each digit. Reverse order so that first vec is for 9's.
    let digit_positions: Vec<Vec<usize>> = (1..10)
        .rev()
        .map(|digit| {
            battery_bank
                .iter()
                .enumerate()
                .filter(|(_, d)| digit == **d)
                .map(|(i, _)| i)
                .collect()
        })
        .collect();

    // Keep candidates as (number so far, index of latest digit)
    let mut candidates: HashSet<(usize, usize)> = HashSet::new();

    // Build starting set of candidates
    for (d, positions) in digit_positions.iter().enumerate() {
        let valid_positions: Vec<&usize> =
            positions.iter().filter(|&p| len - p >= digits).collect();
        if !valid_positions.is_empty() {
            let number = 9 - d;
            candidates.extend(valid_positions.into_iter().map(|&p| (number, p)));
            break;
        }
    }
    // println!("first candidates in {:?}: {:?}", battery_bank, candidates);

    let mut remaining_digits = digits - 1;
    while remaining_digits > 0 {
        // println!("candidates at {}: {:?}", remaining_digits, candidates);
        let mut next_candidates: HashSet<(usize, usize)> = HashSet::new();
        // Each iteration, only keep candidates equal to max number found so far
        let mut max = 0;

        // Build next candidates by selecting the next largest digit that still has valid positions left
        for (candidate, index_last_digit) in candidates {
            for (d, positions) in digit_positions.iter().enumerate() {
                let valid_positions: Vec<&usize> = positions
                    .iter()
                    .filter(|&p| *p > index_last_digit && len - p >= remaining_digits)
                    .collect();

                if !valid_positions.is_empty() {
                    let num = candidate * 10 + 9 - d;
                    if num > max {
                        max = num;
                        next_candidates.clear();
                    }
                    if num == max {
                        next_candidates.extend(valid_positions.into_iter().map(|p| (num, *p)))
                    }
                    break;
                }
            }
        }

        candidates = next_candidates;
        remaining_digits -= 1;
    }

    candidates.into_iter().map(|(n, _)| n).max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

    use super::*;

    const EXAMPLE_INPUT_1: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 357);
    }

    #[test]
    fn test_maximum_joltage() {
        assert_eq!(maximum_joltage("987654321111111"), 98);
        assert_eq!(maximum_joltage("811111111111119"), 89);
        assert_eq!(maximum_joltage("234234234234278"), 78);
        assert_eq!(maximum_joltage("818181911112111"), 92);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_3")), 17244);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1), 3121910778619);
    }

    #[test]
    fn test_max_number_of_n_digits() {
        assert_eq!(max_number_of_n_digits(&[9, 9, 9], 3), 999);
        assert_eq!(max_number_of_n_digits(&[1, 9, 9, 9, 1], 3), 999);
        assert_eq!(max_number_of_n_digits(&[9, 9, 1, 9, 9, 2], 3), 999);
        assert_eq!(max_number_of_n_digits(&[9, 9, 8, 9, 9, 9, 1], 3), 999);
        assert_eq!(max_number_of_n_digits(&[8, 9, 9, 7, 9, 9, 9, 6], 3), 999);
        assert_eq!(
            max_number_of_n_digits(&[9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1], 12),
            987654321111
        );
        assert_eq!(
            max_number_of_n_digits(&[8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9], 12),
            811111111119
        );
        assert_eq!(
            max_number_of_n_digits(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8], 12),
            434234234278
        );
        assert_eq!(
            max_number_of_n_digits(&[8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1], 12),
            888911112111
        );

        let num = "4453322423234323362634238645943333332463321659433346534324232461344544333233244323632243313334262243"
            .chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
        assert_eq!(max_number_of_n_digits(&num, 1), 9);
        assert_eq!(max_number_of_n_digits(&num, 2), 99);
        assert_eq!(max_number_of_n_digits(&num, 3), 996);
        assert_eq!(max_number_of_n_digits(&num, 4), 9966);
        assert_eq!(max_number_of_n_digits(&num, 5), 99666);
        assert_eq!(max_number_of_n_digits(&num, 6), 996666);
        assert_eq!(max_number_of_n_digits(&num, 12), 996664462243);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2025/day_3")), 171435596092638);
    }
}
