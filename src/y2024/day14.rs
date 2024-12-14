use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::str::{FromStr, Lines};
use std::usize;

const MAX_X: i32 = 103;
const MAX_Y: i32 = 101;

type Position = (i32, i32);
type Velocity = (i32, i32);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Robot {
    p: Position,
    v: Velocity,
}

impl Robot {
    fn step(&self, n: i32, max_x: i32, max_y: i32) -> Robot {
        // let next_x = if self.p.0 + self.v.0 < 0 {
        //     max_x + (self.p.0 + self.v.0)
        // } else {
        //     (self.p.0 + self.v.0) % max_x
        // };

        // Can wrap around in both directions of axis using Euclidean div-mod
        let next_x = (self.p.0 + self.v.0 * n).rem_euclid(max_x);
        let next_y = (self.p.1 + self.v.1 * n).rem_euclid(max_y);

        Robot {
            p: (next_x, next_y),
            v: self.v,
        }
    }

    fn quadrant(&self, max_x: i32, max_y: i32) -> Option<usize> {
        if self.p.0 == max_x / 2 || self.p.1 == max_y / 2 {
            None
        } else if self.p.0 < max_x / 2 {
            if self.p.1 < max_y / 2 {
                Some(1)
            } else {
                Some(2)
            }
        } else {
            if self.p.1 < max_y / 2 {
                Some(3)
            } else {
                Some(4)
            }
        }
    }
}

impl std::str::FromStr for Robot {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut parts = input.split_whitespace();

        let p = parts
            .next()
            .unwrap_or("Missing first part")
            .strip_prefix("p=")
            .ok_or("Missing 'p=' prefix")?
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Invalid position coordinates")?;

        let v = parts
            .next()
            .unwrap_or("Missing second part")
            .strip_prefix("v=")
            .ok_or("Missing 'v=' prefix")?
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Invalid velocity coordinates")?;

        if p.len() == 2 && v.len() == 2 {
            Ok(Robot {
                p: (p[1], p[0]),
                v: (v[1], v[0]),
            })
        } else {
            Err("Incorrect number of coordinates".to_string())
        }
    }
}

fn parse_robots(lines: Lines) -> Vec<Robot> {
    lines
        .map(|line| Robot::from_str(line).expect("Could not parse robot"))
        .collect()
}

fn safety_score(robots: &Vec<Robot>, max_x: i32, max_y: i32) -> usize {
    let mut quadrants: HashMap<usize, usize> = HashMap::new();

    for robot in robots {
        if let Some(quadrant) = robot.quadrant(max_x, max_y) {
            *quadrants.entry(quadrant).or_default() += 1;
        }
    }

    quadrants.values().fold(1, |acc, val| acc * val)
}

fn part_1(lines: Lines) -> usize {
    let robots = parse_robots(lines)
        .iter()
        .map(|r| r.step(100, MAX_X, MAX_Y))
        .collect();
    safety_score(&robots, MAX_X, MAX_Y)
}

#[allow(dead_code)]
fn display_grid(robots: &[Robot], file: &mut File, max_x: usize, max_y: usize) {
    let mut grid = vec![vec![0; max_y]; max_x];

    for robot in robots {
        grid[robot.p.0 as usize][robot.p.1 as usize] += 1;
    }

    for row in grid.iter() {
        for &cell in row.iter() {
            if cell > 0 {
                file.write_fmt(format_args!("{}", cell)).unwrap();
            } else {
                file.write(b".").unwrap();
            }
        }
        file.write(b".\n").unwrap();
    }
    file.write(b"\n").unwrap();
}

fn part_2(lines: Lines) -> i32 {
    // let mut seen: HashMap<Vec<Robot>, i32> = HashMap::new();
    let mut robots = parse_robots(lines);
    // Most of the safety scores are close to the score of part 1 since they have a uniform distribution of robots
    // Robots that form a pattern should result in a significantly different safety score
    // Sorting by safety score gives us the most interesting patterns to check out first
    // let mut history: Vec<(i32, usize, Vec<Robot>)> = Vec::new();
    let mut scores: Vec<(i32, usize)> = Vec::new();

    // Robots loop every max_x * max_y steps
    for i in 1..(MAX_X * MAX_Y) {
        robots = robots.drain(..).map(|r| r.step(1, MAX_X, MAX_Y)).collect();
        // if let Some(previous_iter) = seen.insert(robots.clone(), i) { println!("previous: {}", previous_iter); }
        // history.push((i, safety_score(&robots, MAX_X, MAX_Y), robots.clone()));
        scores.push((i, safety_score(&robots, MAX_X, MAX_Y)));
    }

    // history.sort_by(|a, b| a.1.cmp(&b.1));
    // let mut file = File::create("outputs/2024/day14_part2.txt").unwrap();
    // for (iteration, score, robots) in &history[0..20] {
    //     file.write_fmt(format_args!("Iteration {}, score {}\n", iteration, score))
    //         .unwrap();
    //     display_grid(&robots, &mut file, MAX_X as usize, MAX_Y as usize);
    // }

    // The third lowest score contains the chrismas tree
    scores.sort_by(|a, b| a.1.cmp(&b.1));
    scores[2].0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_14");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_move() {
        let mut r = Robot {
            p: (4, 2),
            v: (-3, 2),
        };
        let (max_x, max_y) = (7, 11);
        r = r.step(max_x, max_y);
        assert_eq!(r.p, (1, 4));
        r = r.step(max_x, max_y);
        assert_eq!(r.p, (5, 6));
        r = r.step(max_x, max_y);
        assert_eq!(r.p, (2, 8));
        r = r.step(max_x, max_y);
        assert_eq!(r.p, (6, 10));
        r = r.step(max_x, max_y);
        assert_eq!(r.p, (3, 1));
    }

    #[test]
    fn test_part_1_example() {
        let (max_x, max_y) = (7, 11);
        let robots = step_robots_n_times(parse_robots(EXAMPLE_INPUT.lines()), 100, max_x, max_y);
        assert_eq!(safety_score(&robots, max_x, max_y), 12);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_14").lines()), 230172768);
    }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(
    //         part_2(load_input("inputs/2024/day_14").lines(), 103, 101),
    //         8087
    //     )
    // }
}
