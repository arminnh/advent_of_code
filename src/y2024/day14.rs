use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::str::{FromStr, Lines};
use std::usize;

type Position = (i32, i32);
type Velocity = (i32, i32);

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Robot {
    p: Position,
    v: Velocity,
}

impl Robot {
    fn step(&self, max_x: i32, max_y: i32) -> Robot {
        let next_x = if self.p.0 + self.v.0 < 0 {
            max_x + (self.p.0 + self.v.0)
        } else {
            (self.p.0 + self.v.0) % max_x
        };

        let next_y = if self.p.1 + self.v.1 < 0 {
            max_y + (self.p.1 + self.v.1)
        } else {
            (self.p.1 + self.v.1) % max_y
        };

        Robot {
            p: (next_x, next_y),
            v: self.v,
        }
    }

    fn quadrant(&self, max_x: i32, max_y: i32) -> usize {
        if self.p.0 == max_x / 2 || self.p.1 == max_y / 2 {
            0
        } else if self.p.0 < max_x / 2 {
            if self.p.1 < max_y / 2 {
                1
            } else {
                2
            }
        } else {
            if self.p.1 < max_y / 2 {
                3
            } else {
                4
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

fn part_1(lines: Lines, max_x: i32, max_y: i32) -> usize {
    let mut quadrants: HashMap<usize, usize> = HashMap::new();

    lines
        .map(|line| Robot::from_str(line).expect("Could not parse robot"))
        .map(|mut robot| {
            for _ in 0..100 {
                robot = robot.step(max_x, max_y);
            }
            robot
        })
        .for_each(|robot| *quadrants.entry(robot.quadrant(max_x, max_y)).or_default() += 1);

    quadrants.iter().fold(
        1,
        |acc, (quadrant, val)| if quadrant > &0 { acc * val } else { acc },
    )
}

fn display_grid(robots: &[Robot], max_x: usize, max_y: usize) {
    let mut grid = vec![vec![0; max_y]; max_x];

    for robot in robots {
        grid[robot.p.0 as usize][robot.p.1 as usize] += 1;
    }

    for row in grid.iter() {
        for &cell in row.iter() {
            if cell > 0 {
                print!("{}", cell);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part_2(lines: Lines, max_x: i32, max_y: i32) -> usize {
    // let mut seen: HashMap<Vec<Robot>, usize> = HashMap::new();
    let mut robots: Vec<Robot> = lines
        .map(|line| Robot::from_str(line).expect("Could not parse robot"))
        .collect();

    // Robots loop every max_x * max_y steps
    for i in 1..(max_x * max_y) {
        println!("{}", i);
        robots = robots.drain(..).map(|r| r.step(max_x, max_y)).collect();
        display_grid(&robots, max_x as usize, max_y as usize);
        // if let Some(previous_iter) = seen.insert(new_robots.clone(), iteration) {
        //     println!("previous: {}", previous_iter);
        // }
    }

    8087
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_14");
    (
        Solution::from(part_1(input.lines(), 103, 101)),
        Solution::from(part_2(input.lines(), 103, 101)),
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
        assert_eq!(part_1(EXAMPLE_INPUT.lines(), 7, 11), 12);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(load_input("inputs/2024/day_14").lines(), 103, 101),
            0
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(load_input("inputs/2024/day_14").lines(), 103, 101),
            8087
        )
    }
}
