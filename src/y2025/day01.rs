// How many times does the dial point at 0 after rotating?
pub fn part_1(input: &str) -> usize {
    let mut dial: usize = 50;
    let max = 100;

    input.lines().fold(0, |mut result, line| {
        // Can turn left or right
        let direction = line.chars().next().unwrap();
        // Dial wraps around at 100
        let num = line[1..].parse::<usize>().expect("could not parse number") % max;

        match direction {
            'L' => {
                if num > dial {
                    dial = max - (num - dial);
                } else {
                    dial -= num;
                }
            }
            _ => {
                dial += num;
                if dial >= max {
                    dial %= max;
                }
            }
        }

        if dial == 0 {
            result += 1;
        }

        result
    })
}

// Now count how many times it points to 0 during rotations as well
pub fn part_2(input: &str) -> usize {
    let mut dial = 50;
    let max = 100;

    input.lines().fold(0, |mut result, line| {
        let direction = line.chars().next().unwrap();
        let mut num = line[1..].parse::<usize>().expect("could not parse number");
        // Count the extra rotations
        result += num / max;
        num %= max;

        match direction {
            'L' => {
                if num > dial {
                    // Don't count rollover if starting from zero
                    if dial != 0 {
                        result += 1;
                    }
                    dial = max - (num - dial);
                } else {
                    dial -= num;
                    if dial == 0 {
                        result += 1;
                    }
                }
            }
            _ => {
                dial += num;
                if dial >= max {
                    dial %= max;
                    result += 1;
                }
            }
        }

        result
    })
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

    use super::*;

    const EXAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    const EXAMPLE_INPUT_2: &str = "L68
L30
R48
L100
L5
R60
L55
R200
L1
L99
R1000
R14
L82
L100
R168
L50
R550";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT), 3);
        assert_eq!(part_1(EXAMPLE_INPUT_2), 8);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_1")), 1147);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT), 6);
        assert_eq!(part_2(EXAMPLE_INPUT_2), 29);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2025/day_1")), 6789);
    }
}
