use std::collections::HashSet;
use std::hash::Hash;

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

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
fn get_priority(item: &char) -> u8 {
    if item.is_lowercase() {
        (*item as u8) - 'a' as u8 + 1
    } else {
        (*item as u8) - 'A' as u8 + 1 + 26
    }
}

// What is the sum of the priorities of item types that appear in both compartments of each rucksack?
pub fn part_1(input: &str) -> u32 {
    let mut total: u32 = 0;
    input.lines().for_each(|rucksack: &str| {
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

// What is the sum of the priorities of those item types that correspond to the badges of each three-Elf group?
pub fn part_2(input: &str) -> u32 {
    let mut total: u32 = 0;
    let mut lines = input.lines();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_single_line() {
        assert_eq!(part_1("vJrwpWtwJgWrhcsFMMfFFhFp"), 16);
        assert_eq!(part_1("PmmdzqPrVvPwwTWBwg"), 42);
    }

    #[test]
    fn test_part_1_multi_line() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(part_1(input), 157)
    }

    #[test]
    fn test_part_2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(part_2(input), 70)
    }
}
