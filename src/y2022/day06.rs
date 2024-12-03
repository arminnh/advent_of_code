use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;
use std::{collections::HashSet, hash::Hash};

fn no_duplicates<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut unique = HashSet::new();
    // HashSet.insert returns whether the value was newly inserted.
    iter.into_iter().all(move |x| unique.insert(x))
}

// How many characters need to be processed before the first start-of-packet marker is detected?
fn find_start_of_packet_marker(lines: Lines, nr_of_distinct_chars: usize) -> usize {
    for line in lines {
        if line.len() > 0 {
            for chars in line
                .char_indices()
                .into_iter()
                .collect::<Vec<(usize, char)>>()
                .windows(nr_of_distinct_chars)
            {
                if no_duplicates(chars.iter().map(|(_, c)| c)) {
                    println!("{:?}", chars);
                    return chars[nr_of_distinct_chars - 1].0 + 1;
                }
            }
        }
    }

    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2022/day_6");
    (
        Solution::from(find_start_of_packet_marker(input.lines(), 4)),
        Solution::from(find_start_of_packet_marker(input.lines(), 14)),
    )
}
