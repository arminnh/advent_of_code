// The sum of all the invalid IDs in the given ranges
pub fn part_1(input: &str) -> usize {
    sum_of_invalid_ids(input, false)
}

fn sum_of_invalid_ids(input: &str, part_2: bool) -> usize {
    input
        .split(",")
        .filter_map(|range| range.trim().split_once('-'))
        .map(|(start, end)| {
            let start = start
                .parse::<usize>()
                .expect("Could not parse start of range");
            let end = end.parse::<usize>().expect("Could not parse end of range");

            (start..=end)
                .filter(|&i| {
                    if part_2 {
                        is_invalid_id_part_2(i)
                    } else {
                        is_invalid_id(i)
                    }
                })
                .collect::<Vec<usize>>()
        })
        .flatten()
        .sum()
}

// ID is invalid if it is made up of a sequence of digits repeated twice
fn is_invalid_id(id: usize) -> bool {
    if id < 11 {
        return false;
    }

    let nr_of_digits = id.ilog10() + 1;
    if nr_of_digits % 2 != 0 {
        return false;
    }

    // Split number in two parts and check if they're the same
    let divisor = 10_usize.pow(nr_of_digits / 2);
    let (left, right) = (id / divisor, id % divisor);
    left == right
}

pub fn part_2(input: &str) -> usize {
    sum_of_invalid_ids(input, true)
}

// ID is invalid if it is made up of a sequence of digits AT LEAST twice
fn is_invalid_id_part_2(id: usize) -> bool {
    if id < 11 {
        return false;
    }

    let nr_of_digits_total = id.ilog10() + 1;
    for pattern_digits in 1..=nr_of_digits_total / 2 {
        // Only check cases where the pattern fits multiple times in the ID
        if nr_of_digits_total % pattern_digits != 0 {
            continue;
        }

        // Check if ONLY this pattern repeats itself in the ID
        let pattern = id % 10_usize.pow(pattern_digits);

        // Split the ID in pieces and check that each piece matches the pattern
        let mut attempt = id;
        while attempt > 0 {
            let divisor = 10_usize.pow(pattern_digits);
            let (left, right) = (attempt / divisor, attempt % divisor);
            if right != pattern {
                break;
            } else {
                attempt = left;
            }
        }
        if attempt == 0 {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

    use super::*;

    const EXAMPLE_INPUT_1: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_is_invalid_id() {
        assert_eq!(is_invalid_id(0), false);
        assert_eq!(is_invalid_id(1), false);
        assert_eq!(is_invalid_id(10), false);
        assert_eq!(is_invalid_id(11), true);
        assert_eq!(is_invalid_id(12), false);
        assert_eq!(is_invalid_id(20), false);
        assert_eq!(is_invalid_id(22), true);
        assert_eq!(is_invalid_id(99), true);
        assert_eq!(is_invalid_id(100), false);
        assert_eq!(is_invalid_id(101), false);
        assert_eq!(is_invalid_id(1010), true);
        assert_eq!(is_invalid_id(1188511885), true);
        assert_eq!(is_invalid_id(222222), true);
        assert_eq!(is_invalid_id(446446), true);
        assert_eq!(is_invalid_id(38593859), true);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 1227775554); // too low
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_2")), 32976912643);
    }

    #[test]
    fn test_is_invalid_id_part_2() {
        assert_eq!(is_invalid_id_part_2(0), false);
        assert_eq!(is_invalid_id_part_2(1), false);
        assert_eq!(is_invalid_id_part_2(10), false);
        assert_eq!(is_invalid_id_part_2(11), true);
        assert_eq!(is_invalid_id_part_2(12), false);
        assert_eq!(is_invalid_id_part_2(20), false);
        assert_eq!(is_invalid_id_part_2(22), true);
        assert_eq!(is_invalid_id_part_2(99), true);
        assert_eq!(is_invalid_id_part_2(100), false);
        assert_eq!(is_invalid_id_part_2(101), false);
        assert_eq!(is_invalid_id_part_2(1010), true);
        assert_eq!(is_invalid_id_part_2(1188511885), true);
        assert_eq!(is_invalid_id_part_2(222222), true);
        assert_eq!(is_invalid_id_part_2(446446), true);
        assert_eq!(is_invalid_id_part_2(38593859), true);
        assert_eq!(is_invalid_id_part_2(111), true);
        assert_eq!(is_invalid_id_part_2(999), true);
        assert_eq!(is_invalid_id_part_2(565656), true);
        assert_eq!(is_invalid_id_part_2(824824824), true);
        assert_eq!(is_invalid_id_part_2(2121212121), true);
        assert_eq!(is_invalid_id_part_2(1111), true);
        assert_eq!(is_invalid_id_part_2(11111), true);
        assert_eq!(is_invalid_id_part_2(111111), true);
        assert_eq!(is_invalid_id_part_2(1111111), true);
        assert_eq!(is_invalid_id_part_2(11111111), true);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1), 4174379265);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2025/day_2")), 54446379122);
    }
}
