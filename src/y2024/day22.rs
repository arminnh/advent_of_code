use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;
use std::usize;

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(val: u64) -> u64 {
    val % 16777216
}

fn n_iterations(mut secret: u64, n: usize) -> u64 {
    for _ in 0..n {
        let mult = secret as u64 * 64;
        secret = mix(secret, mult);
        secret = prune(secret);
        let div = secret / 32;
        secret = mix(secret, div);
        secret = prune(secret);
        let mult2 = secret * 2048;
        secret = mix(secret, mult2);
        secret = prune(secret);
    }
    secret
}

fn part_1(lines: Lines) -> u64 {
    lines
        .map(|line| n_iterations(line.parse().expect("Could not parse secret number"), 2000))
        .sum()
}

fn part_2(lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_22");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1
10
100
2024";

    #[test]
    fn test_n_iterations() {
        assert_eq!(n_iterations(123, 1), 15887950);
        assert_eq!(n_iterations(123, 2), 16495136);
        assert_eq!(n_iterations(123, 3), 527345);
        assert_eq!(n_iterations(123, 4), 704524);
        assert_eq!(n_iterations(123, 5), 1553684);
        assert_eq!(n_iterations(123, 6), 12683156);
        assert_eq!(n_iterations(123, 7), 11100544);
        assert_eq!(n_iterations(123, 8), 12249484);
        assert_eq!(n_iterations(123, 9), 7753432);
        assert_eq!(n_iterations(123, 10), 5908254);
        assert_eq!(n_iterations(1, 2000), 8685429);
        assert_eq!(n_iterations(10, 2000), 4700978);
        assert_eq!(n_iterations(100, 2000), 15273692);
        assert_eq!(n_iterations(2024, 2000), 8667524);
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 37327623);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_22").lines()), 0);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_22").lines()), 0)
    }
}
