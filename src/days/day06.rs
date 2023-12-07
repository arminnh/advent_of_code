use crate::{Solution, SolutionPair};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::usize;

fn get_lines(path: &str) -> Lines<BufReader<File>> {
    BufReader::new(File::open(path).expect("Could not open file.")).lines()
}

fn parse_line_part_1(line: String) -> Vec<usize> {
    line.split_once(":")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn parse_line_part_2(line: String) -> usize {
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

fn part_1(mut lines: Lines<BufReader<File>>) -> usize {
    let times = parse_line_part_1(lines.next().unwrap().unwrap());
    let record_distances = parse_line_part_1(lines.next().unwrap().unwrap());

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

fn part_2(mut lines: Lines<BufReader<File>>) -> usize {
    let time = parse_line_part_2(lines.next().unwrap().unwrap());
    let record_distance = parse_line_part_2(lines.next().unwrap().unwrap());

    calculate_nr_of_wins(time, record_distance)
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_1(get_lines("inputs/day_6"))),
        Solution::from(part_2(get_lines("inputs/day_6"))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(get_lines("inputs/day_6_example")), 288)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(get_lines("inputs/day_6")), 840336)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(get_lines("inputs/day_6")), 41382569)
    }
}
