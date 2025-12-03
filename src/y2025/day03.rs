// The sum of the maximum joltage from each battery bank
pub fn part_1(input: &str) -> u32 {
    input.lines().map(|line| maximum_joltage(line)).sum()
}

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

// ...
pub fn part_2(input: &str) -> usize {
    0
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
    fn tst_maximum_joltage() {
        assert_eq!(maximum_joltage("987654321111111"), 98);
        assert_eq!(maximum_joltage("811111111111119"), 89);
        assert_eq!(maximum_joltage("234234234234278"), 78);
        assert_eq!(maximum_joltage("818181911112111"), 92);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_3")), 17244);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT_1), 0);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(&load_input("inputs/2025/day_3")), 0);
    // }
}
