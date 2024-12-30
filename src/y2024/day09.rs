use std::usize;

// Expand compact disk map representation into blocks view of file blocks and free space blocks.
// Each block contains the id of (a part of) the file on the block.
fn parse_disk_map(input: &str) -> Vec<Option<usize>> {
    let mut file_id: i32 = -1;

    input
        .chars()
        .map(|c| c.to_digit(10).expect("NaN in disk map"))
        .enumerate()
        .flat_map(|(i, val)| {
            // val represents file length or amount of free space in alternating order
            let block_value = if i % 2 == 0 {
                file_id += 1;
                Some(file_id as usize)
            } else {
                None
            };

            [block_value].repeat(val as usize)
        })
        .collect()
}

#[allow(dead_code)]
fn print_disk_blocks(disk_blocks: &Vec<Option<usize>>) {
    for block in disk_blocks {
        if let Some(value) = block {
            print!("{}", value);
        } else {
            print!(".");
        }
    }
    println!()
}

// Move all file parts to the front of the disk, starting from the back
fn fragment_files(mut disk_blocks: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut left = 0;
    let mut right = disk_blocks.len() - 1;
    while left < right {
        if disk_blocks[left].is_some() {
            left += 1;
        } else if disk_blocks[right].is_none() {
            right -= 1;
        } else {
            disk_blocks[left] = disk_blocks[right];
            disk_blocks[right] = None;
            left += 1;
            right -= 1;
        }
    }

    disk_blocks
}

fn checksum(disk_blocks: Vec<Option<usize>>) -> usize {
    disk_blocks.iter().enumerate().fold(0, |acc, (i, &id)| {
        if let Some(val) = id {
            acc + i * val as usize
        } else {
            acc
        }
    })
}

// Get the blocks of free space as (index, length) pairs
fn get_free_space(disk_blocks: &Vec<Option<usize>>) -> Vec<(usize, usize)> {
    let mut free_space: Vec<(usize, usize)> = Vec::new();
    let mut left = 0;
    while left < disk_blocks.len() {
        if disk_blocks[left].is_none() {
            let mut right = left + 1;
            while right < disk_blocks.len() && disk_blocks[right].is_none() {
                right += 1;
            }
            let length = right - left;
            free_space.push((left, length));
            left += length;
        } else {
            left += 1;
        }
    }
    free_space
}

// Move whole files instead of parts
fn defragment_files(mut disk_blocks: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut free_space: Vec<(usize, usize)> = get_free_space(&disk_blocks);
    // print_disk_blocks(&disk_blocks);

    let mut right = disk_blocks.len() - 1;
    while right > 0 {
        if disk_blocks[right].is_none() {
            right -= 1;
        } else {
            let current_file = disk_blocks[right];
            let mut left = right - 1;
            while left > 0 && disk_blocks[left] == current_file {
                left -= 1;
            }
            let length = right - left;

            if let Some(free_space_index) = free_space
                .iter()
                .position(|(i, space)| i < &left && space >= &length)
            {
                let (block_index, free_space_left) = free_space[free_space_index];
                for k in 0..length {
                    disk_blocks[block_index + k] = disk_blocks[left + k + 1];
                    disk_blocks[left + k + 1] = None;
                    // print_disk_blocks(&disk_blocks);
                }

                if free_space_left == length {
                    free_space.remove(free_space_index);
                } else {
                    free_space[free_space_index] = (block_index + length, free_space_left - length);
                }
            }
            right -= length;
        }
    }

    disk_blocks
}

// Compact the amphipod's hard drive. What is the resulting filesystem checksum?
pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(parse_disk_map)
        .map(fragment_files)
        .map(checksum)
        .sum()
}

// Filesystem checksum if we defragment instead of fragment
pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(parse_disk_map)
        .map(defragment_files)
        .map(checksum)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::util::load_input;

    const EXAMPLE_INPUT_1: &str = "12345";
    const EXAMPLE_INPUT_2: &str = "2333133121414131402";

    #[test]
    fn test_fragment() {
        assert_eq!(
            fragment_files(parse_disk_map(EXAMPLE_INPUT_1)),
            vec![
                Some(0),
                Some(2),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(2),
                Some(2),
                Some(2),
                None,
                None,
                None,
                None,
                None,
                None
            ]
        );
        assert_eq!(
            fragment_files(parse_disk_map(EXAMPLE_INPUT_2)),
            vec![
                Some(0),
                Some(0),
                Some(9),
                Some(9),
                Some(8),
                Some(1),
                Some(1),
                Some(1),
                Some(8),
                Some(8),
                Some(8),
                Some(2),
                Some(7),
                Some(7),
                Some(7),
                Some(3),
                Some(3),
                Some(3),
                Some(6),
                Some(4),
                Some(4),
                Some(6),
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                Some(6),
                Some(6),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None
            ]
        );
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_2), 1928);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2024/day_9")), 6330095022244);
    }

    #[test]
    fn test_defragment() {
        assert_eq!(
            defragment_files(parse_disk_map(EXAMPLE_INPUT_1)),
            vec![
                Some(0),
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                Some(2),
                Some(2),
                Some(2),
                Some(2),
                Some(2),
            ]
        );
        assert_eq!(
            defragment_files(parse_disk_map(EXAMPLE_INPUT_2)),
            vec![
                Some(0),
                Some(0),
                Some(9),
                Some(9),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(7),
                Some(7),
                Some(7),
                None,
                Some(4),
                Some(4),
                None,
                Some(3),
                Some(3),
                Some(3),
                None,
                None,
                None,
                None,
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                None,
                Some(6),
                Some(6),
                Some(6),
                Some(6),
                None,
                None,
                None,
                None,
                None,
                Some(8),
                Some(8),
                Some(8),
                Some(8),
                None,
                None,
            ]
        );
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_2), 2858);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2024/day_9")), 6359491814941)
    }
}
