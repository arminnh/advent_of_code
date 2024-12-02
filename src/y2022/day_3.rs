use std::collections::HashSet;
use std::fs;
use std::hash::Hash;
use std::str::Lines;

fn get_duplicates<T>(left: T, right: T) -> Vec<T::Item>
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut set = HashSet::new();
    left.into_iter().for_each(|x| {
        set.insert(x);
    });

    // let mut out: Vec<T::Item> = right.into_iter().filter(|x| set.contains(x)).collect();
    // out.dedup();
    // out
    right.into_iter().fold(Vec::new(), |mut acc, c| {
        if set.contains(&c) && !acc.contains(&c) {
            acc.push(c);
        }
        acc
    })
}

/// Lowercase item types a through z have priorities 1 through 26.
/// Uppercase item types A through Z have priorities 27 through 52.
fn get_priority(item: &char) -> u8 {
    if item.is_lowercase() {
        (*item as u8) - 'a' as u8 + 1
    } else {
        (*item as u8) - 'A' as u8 + 1 + 26
    }
}

/// The Elves have made a list of all of the items currently in each rucksack
/// (your puzzle input), but they need your help finding the errors. Every item
/// type is identified by a single lowercase or uppercase letter (that is,
/// a and A refer to different types of items).
/// Part 1: Find the item type that appears in both compartments of each rucksack.
/// What is the sum of the priorities of those item types?
fn part_1_sum_of_priorities_in_rucksacks(lines: Lines) -> u32 {
    let mut total: u32 = 0;
    lines.for_each(|rucksack: &str| {
        if rucksack.len() > 0 {
            let (left, right): (&str, &str) = rucksack.split_at(rucksack.len() / 2);
            let items: Vec<char> = get_duplicates(left.chars(), right.chars());
            let priority: u8 = items.iter().fold(0, |acc, item| acc + get_priority(item));
            println!(
                "{:?}, {:?}, {:?}, {:?}, {:?}",
                rucksack, left, right, items, priority
            );
            total += priority as u32;
        }
    });
    println!("Sum of priorities in rucksacks: {}\n\n", total);
    total
}

/// Part 2: Find the item type that corresponds to the badges of each three-Elf group.
/// What is the sum of the priorities of those item types?
fn part_2_sum_of_priorities_in_groups(lines: &mut Lines) -> u32 {
    let mut total: u32 = 0;
    while let (Some(l1), Some(l2), Some(l3)) = (lines.next(), lines.next(), lines.next()) {
        let duplicates_l1_l2: String = get_duplicates(l1.chars(), l2.chars())
            .into_iter()
            .collect::<String>();
        let badges: Vec<char> = get_duplicates(duplicates_l1_l2.chars(), l3.chars());
        println!("{:?}, {:?}, {:?}, {:?}", l1, l2, l3, badges);
        total += badges.iter().fold(0, |acc, c| acc + get_priority(c)) as u32;
    }
    println!("Sum of priorities in groups: {}", total);
    total
}

fn main() {
    if let Ok(contents) = fs::read_to_string("inputs/day_3") {
        part_1_sum_of_priorities_in_rucksacks(contents.lines());
        part_2_sum_of_priorities_in_groups(&mut contents.lines());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_single_line() {
        assert_eq!(
            part_1_sum_of_priorities_in_rucksacks("vJrwpWtwJgWrhcsFMMfFFhFp".lines()),
            16
        );
        assert_eq!(
            part_1_sum_of_priorities_in_rucksacks("PmmdzqPrVvPwwTWBwg".lines()),
            42
        );
    }

    #[test]
    fn test_part_1_multi_line() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(part_1_sum_of_priorities_in_rucksacks(input.lines()), 157)
    }

    #[test]
    fn test_part_2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(part_2_sum_of_priorities_in_groups(&mut input.lines()), 70)
    }
}
