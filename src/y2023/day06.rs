use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;
use std::usize;

fn parse_line_part_1(line: &str) -> Vec<usize> {
    line.split_once(":")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn parse_line_part_2(line: &str) -> usize {
    line.split_once(":")
        .unwrap()
        .1
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn calculate_distance(total_time: usize, time_charged: usize) -> usize {
    let speed_gained = time_charged;
    let remaining_time = total_time - time_charged;

    speed_gained * remaining_time
}

fn calculate_nr_of_wins(time: usize, record_distance: usize) -> usize {
    (1..time).fold(0, |nr_of_wins, time_charged| {
        if calculate_distance(time, time_charged) > record_distance {
            nr_of_wins + 1
        } else {
            nr_of_wins
        }
    })
}

fn part_1(mut lines: Lines) -> usize {
    let times = parse_line_part_1(lines.next().unwrap());
    let record_distances = parse_line_part_1(lines.next().unwrap());

    times
        .iter()
        .zip(record_distances.iter())
        .map(|(time, record_distance)| calculate_nr_of_wins(*time, *record_distance))
        .fold(1, |result, nr_of_wins| {
            if nr_of_wins > 0 {
                result * nr_of_wins
            } else {
                result
            }
        })
}

fn part_2(mut lines: Lines) -> usize {
    let time = parse_line_part_2(lines.next().unwrap());
    let record_distance = parse_line_part_2(lines.next().unwrap());

    calculate_nr_of_wins(time, record_distance)
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_6");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 288)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_6").lines()), 840336)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_6").lines()), 41382569)
    }
}
