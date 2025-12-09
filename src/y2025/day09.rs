// What is the largest area of any rectangle you can make?
pub fn part_1(input: &str) -> usize {
    let corners: Vec<(u32, u32)> = input
        .lines()
        .map(|line| line.split_once(",").expect("Could not split line"))
        .map(|(a, b)| {
            (
                a.parse::<u32>().expect("Could not parse first number"),
                b.parse::<u32>().expect("Could not parse second number"),
            )
        })
        .collect();

    let mut max = 0;

    for i in 0..(corners.len() - 1) {
        for j in i + i..corners.len() {
            max = max.max(area(&corners[i], &corners[j]));
        }
    }

    max
}

fn area(i: &(u32, u32), j: &(u32, u32)) -> usize {
    let dx = (i.0.abs_diff(j.0) + 1) as usize;
    let dy = (i.1.abs_diff(j.1) + 1) as usize;
    dx * dy
}

//
pub fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

    use super::*;

    const EXAMPLE_INPUT_1: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 50);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_9")), 4_765_757_080);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT_1), 0);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(&load_input("inputs/2025/day_9")), 0);
    // }
}
