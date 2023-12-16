use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::str::Lines;
use std::usize;

fn hash(s: &[u8]) -> usize {
    let mut current = 0;

    for c in s {
        current += *c as usize;
        current *= 17;
        current %= 256;
    }

    current
}

fn part_1(lines: Lines) -> usize {
    lines
        .map(|line| line.split(',').map(|s| hash(s.as_bytes())).sum::<usize>())
        .sum()
}

fn part_2(lines: Lines) -> usize {
    let mut boxes_of_lenses: HashMap<usize, Vec<(&[u8], u8)>> = HashMap::new();

    lines.for_each(|line| {
        line.split(',').for_each(|operation| {
            match operation.as_bytes() {
                // Add a lens
                [label @ .., b'=', focal_length] => {
                    // println!("Add {:?}", str::from_utf8(label).unwrap());
                    let lenses = boxes_of_lenses.entry(hash(label)).or_default();
                    match lenses
                        .iter()
                        .position(|(label_in_box, _)| label_in_box == &label)
                    {
                        // Replace focal length of existing lens
                        Some(position) => lenses.get_mut(position).unwrap().1 = *focal_length,
                        // Add lens to back of list
                        None => lenses.push((label, *focal_length)),
                    }
                }
                // Remove a lens
                [label @ .., b'-'] => {
                    // println!("Remove {:?}, {:?}", label, str::from_utf8(label).unwrap());
                    boxes_of_lenses.entry(hash(label)).and_modify(|lenses| {
                        if let Some(position) = lenses
                            .iter()
                            .position(|(label_in_box, _)| label_in_box == &label)
                        {
                            lenses.remove(position);
                        }
                    });
                }
                _ => panic!("Invalid line {:?}", line),
            }
        });
    });

    boxes_of_lenses
        .iter()
        .map(|(box_number, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(slot_number, (_, focal_length))| {
                    // Calculate focusing power of the resulting lens configuration
                    (box_number + 1) * (slot_number + 1) * (*focal_length as usize - 48)
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_15");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash("rn=1".as_bytes()), 30);
        assert_eq!(hash("cm-".as_bytes()), 253);
        assert_eq!(hash("pc=4".as_bytes()), 180);
        assert_eq!(hash("ot=9".as_bytes()), 9);
        assert_eq!(hash("ot=7".as_bytes()), 231);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines()), 1320);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_15").lines()), 513158);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 145);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_15").lines()), 200277);
    }
}
