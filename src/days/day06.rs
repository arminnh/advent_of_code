use crate::{Solution, SolutionPair};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::usize;

fn get_lines(path: &str) -> Lines<BufReader<File>> {
    BufReader::new(File::open(path).expect("Could not open file.")).lines()
}

fn parse_line(line: String) -> Vec<usize> {
    line.split_once(":")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
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
    let times = parse_line(lines.next().unwrap().unwrap());
    let record_distances = parse_line(lines.next().unwrap().unwrap());

    let result = times
        .iter()
        .zip(record_distances.iter())
        .map(|(time, record_distance)| {
            let nr_of_wins = calculate_nr_of_wins(*time, *record_distance);
            println!("{:?}, {:?}, {:?}", time, record_distance, nr_of_wins);
            nr_of_wins
        })
        .fold(1, |result, nr_of_wins| {
            if nr_of_wins > 0 {
                result * nr_of_wins
            } else {
                result
            }
        });

    println!("{:?}", result);

    result
}

pub fn solve() -> SolutionPair {
    (
        Solution::from(part_1(get_lines("inputs/day_6"))),
        Solution::from(0),
    )
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(get_lines("inputs/day_6_example")), 288)
    }
}
