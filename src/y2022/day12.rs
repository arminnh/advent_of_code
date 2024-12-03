use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::str::Lines;

type HeightMap = Vec<Vec<usize>>;

fn parse_input(lines: Lines) -> (HeightMap, (usize, usize), (usize, usize)) {
    let mut map: Vec<Vec<usize>> = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    lines.enumerate().for_each(|(i, line)| {
        println!("{i}, {line:?}");
        let mut row: Vec<usize> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                'a'..='z' => row.push(c as usize - 'a' as usize + 1),
                'S' => {
                    row.push(0);
                    start = (i, j);
                }
                'E' => {
                    row.push(26);
                    end = (i, j);
                }
                _ => todo!(),
            }
        }
        map.push(row);
    });

    (map, start, end)
}

fn part_1(lines: Lines) -> usize {
    let (height_map, start, end) = parse_input(lines);
    println!("{height_map:#?}");
    println!("{start:?}");
    println!("{end:?}");
    todo!()
}

fn part_2(lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2022/day_12");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(load_input("inputs/2022/day_12_example").lines()), 0)
    }

    // #[test]
    // fn test_part_1() {
    //     assert_eq!(part_1(load_input("inputs/2022/day_12").lines()), 0)
    // }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(load_input("inputs/2022/day_12_example").lines()), 0)
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(load_input("inputs/2022/day_12").lines()), 0)
    // }
}
