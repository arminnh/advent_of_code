use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;
use std::usize;

// Expand compact disk map representation into blocks view of file blocks and free space blocks.
// Each block contains the id of (a part of) the file on the block. Free space is represented by -1.
fn parse_disk_map(input: &str) -> Vec<i32> {
    let mut file_id = -1;

    input
        .chars()
        .map(|c| c.to_digit(10).expect("NaN in disk map"))
        .enumerate()
        .flat_map(|(i, val)| {
            // val represents file length or amount of free space in alternating order
            let block_value = if i % 2 == 0 {
                file_id += 1;
                file_id
            } else {
                -1
            };

            [block_value].repeat(val as usize)
        })
        .collect()
}

fn fragment_files(mut disk_blocks: Vec<i32>) -> Vec<i32> {
    let mut left = 0;
    let mut right = disk_blocks.len() - 1;
    while left < right {
        if disk_blocks[left] != -1 {
            left += 1;
        } else if disk_blocks[right] == -1 {
            right -= 1;
        } else {
            disk_blocks[left] = disk_blocks[right];
            disk_blocks[right] = -1;
            left += 1;
            right -= 1;
        }
    }

    disk_blocks
}

fn checksum(disk_blocks: Vec<i32>) -> usize {
    let mut result = 0;
    for (i, &id) in disk_blocks.iter().enumerate() {
        if id == -1 {
            return result;
        }
        result += i * id as usize;
    }
    result
}

// Compact the amphipod's hard drive. What is the resulting filesystem checksum?
fn part_1(lines: Lines) -> usize {
    lines
        .map(parse_disk_map)
        .map(fragment_files)
        .map(checksum)
        .sum()
}

fn part_2(lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_9");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "12345";
    const EXAMPLE_INPUT_2: &str = "2333133121414131402";

    #[test]
    fn test_fragment() {
        assert_eq!(
            fragment_files(parse_disk_map(EXAMPLE_INPUT_1)),
            vec![0, 2, 2, 1, 1, 1, 2, 2, 2, -1, -1, -1, -1, -1, -1]
        );
        assert_eq!(
            fragment_files(parse_disk_map(EXAMPLE_INPUT_2)),
            vec![
                0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6,
                -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
            ]
        );
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_2.lines()), 1928);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_9").lines()), 6330095022244);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_9").lines()), 0)
    }
}
