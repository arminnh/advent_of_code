use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::fmt::Display;
use std::num::ParseFloatError;
use std::str::{FromStr, Lines};
use std::usize;

#[derive(Debug, Clone, Copy)]
struct Vec3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Display for Vec3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl FromStr for Vec3D {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(",").map(|s| s.trim()).collect::<Vec<&str>>()[..] {
            [x, y, z] => Ok(Vec3D {
                x: x.parse()?,
                y: y.parse()?,
                z: z.parse()?,
            }),
            _ => panic!("Invalid Vec3D input: {:?}", s),
        }
    }
}

struct Hailstone {
    position: Vec3D,
    velocity: Vec3D,
}

impl Hailstone {
    fn is_in_future(&self, x: f32, y: f32) -> bool {
        let dot_product =
            (x - self.position.x) * self.velocity.x + (y - self.position.y) * self.velocity.y;

        dot_product > 0.0
    }
}

fn parse_hailstones(input_lines: Lines<'_>) -> Vec<Hailstone> {
    input_lines
        .map(
            |l| match l.split("@").map(|l| l.trim()).collect::<Vec<&str>>()[..] {
                [position, velocity] => Hailstone {
                    position: Vec3D::from_str(position).unwrap(),
                    velocity: Vec3D::from_str(velocity).unwrap(),
                },
                _ => panic!("Invalid input line {:?}", l),
            },
        )
        .collect()
}

struct Line {
    // y = m x + b
    m: f32,
    b: f32,
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "y = {} x + {}", self.m, self.b)
    }
}

impl Line {
    fn from_point_slope_form(p: Vec3D, velocity: Vec3D) -> Self {
        let slope = velocity.y / velocity.x;
        Line {
            m: slope,
            b: p.y - slope * p.x,
        }
    }

    fn intersection(&self, other: &Line) -> Option<(f32, f32)> {
        if self.m == other.m || self.b == other.b {
            return None;
        }
        // For two lines   y = m1* x + b1   and   y = m2 * x + b2
        // Can substitute:
        // x = (y - b1) / m1
        // => y = (b2 - b1 * m2 / m1) * (m1 / (m1 - m2))
        let y = (other.b - self.b * other.m / self.m) * self.m / (self.m - other.m);
        let x = (y - self.b) / self.m;
        Some((x, y))
    }
}

fn part_1(input_lines: Lines, min: f32, max: f32) -> usize {
    let hailstones: Vec<Hailstone> = parse_hailstones(input_lines);
    let lines: Vec<Line> = hailstones
        .iter()
        .map(|h| Line::from_point_slope_form(h.position, h.velocity))
        .collect();
    let mut intersections: Vec<(usize, f32, f32)> = Vec::new();
    let in_range = |x, y| -> bool { x >= min && x <= max && y >= min && y <= max };

    for (i, line_1) in lines.iter().enumerate() {
        for j in i + 1..lines.len() {
            if let Some((x, y)) = line_1.intersection(&lines[j]) {
                if in_range(x, y)
                    && hailstones[i].is_in_future(x, y)
                    && hailstones[j].is_in_future(x, y)
                {
                    intersections.push((i, x, y));
                }
            }
        }
    }

    intersections.len()
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_24");
    (
        Solution::from(part_1(
            input.lines(),
            200_000_000_000_000.0,
            400_000_000_000_000.0,
        )),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1.lines(), 7.0, 27.0), 2);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                load_input("inputs/day_24").lines(),
                200_000_000_000_000.0,
                400_000_000_000_000.0
            ),
            18651
        );
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_24").lines()), 0);
    }
}
