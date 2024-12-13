use crate::util::util::load_input;
use crate::{Solution, SolutionPair};
use std::usize;

type Coord = (usize, usize);

struct Line {
    // y = m x + b
    m: f32,
    b: f32,
}

impl Line {
    fn from_point_slope_form(p: Coord, slope: (usize, usize)) -> Self {
        let slope: f32 = slope.1 as f32 / slope.0 as f32;
        Line {
            m: slope,
            b: p.1 as f32 - slope * p.0 as f32,
        }
    }

    fn intersection(&self, other: &Line) -> Option<(f32, f32)> {
        if self.m == other.m {
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

fn parse_button(s: &str) -> Coord {
    let (x, y) = s
        .split_once("X+")
        .expect("Could not find X in button")
        .1
        .split_once(", Y+")
        .expect("Could not find Y in button");

    (
        x.parse().expect("Could not parse X as number"),
        y.parse().expect("Could not parse Y as number"),
    )
}

fn parse_prize(s: &str) -> Coord {
    let (x, y) = s
        .split_once("X=")
        .expect("Could not find X in prize")
        .1
        .split_once(", Y=")
        .expect("Could not find Y in prize");

    (
        x.parse().expect("Could not parse X as number"),
        y.parse().expect("Could not parse Y as number"),
    )
}

fn parse_machine(input: &str) -> (Coord, Coord, Coord) {
    let mut lines = input.lines();
    let a = parse_button(lines.next().expect("No string left for button A"));
    let b = parse_button(lines.next().expect("No string left for button B"));
    let prize = parse_prize(lines.next().expect("No string left for Prize"));
    (a, b, prize)
}

// Smallest number of presses of buttons A and B to get to the Prize
fn min_tokens_to_win_price(a: Coord, b: Coord, prize: Coord) -> usize {
    // look for intersection between origin with slope A and prize with slope B
    let origin_and_a = Line::from_point_slope_form((0, 0), a);
    let prize_and_b = Line::from_point_slope_form(prize, b);

    if let Some(intersection) = origin_and_a.intersection(&prize_and_b) {
        let (inters_x, inters_y) = (
            intersection.0.round() as usize,
            intersection.1.round() as usize,
        );
        if inters_x > prize.0 || inters_y > prize.1 {
            // overshot the target
            return 0;
        }

        if inters_x % a.0 != 0 || (inters_x / a.0 != inters_y / a.1) {
            // nr of A presses cannot be determined
            return 0;
        }
        let a_steps = inters_x / a.0;

        if (prize.0 - inters_x) % b.0 != 0
            || ((prize.0 - inters_x) / b.0 != (prize.1 - inters_y) / b.1)
        {
            // nr of A presses cannot be determined
            return 0;
        }
        let b_steps = (prize.0 - inters_x) / b.0;

        return a_steps * 3 + b_steps;
    }
    0
}

fn part_1(input: String) -> usize {
    input
        .split("\n\n")
        .map(|s| parse_machine(s))
        .map(|(a, b, prize)| min_tokens_to_win_price(a, b, prize))
        .sum()
}

fn part_2(input: String) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/2024/day_13");
    (
        Solution::from(part_1(input.clone())),
        Solution::from(part_2(input)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.to_string()), 480);
    }

    #[test]
    fn test_min_tokens_to_win_price_example() {
        let (a, b, p) = parse_machine(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400",
        );
        assert_eq!(min_tokens_to_win_price(a, b, p), 280);

        let (a, b, p) = parse_machine(
            "Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176",
        );
        assert_eq!(min_tokens_to_win_price(a, b, p), 0);

        let (a, b, p) = parse_machine(
            "Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450",
        );
        assert_eq!(min_tokens_to_win_price(a, b, p), 200);

        let (a, b, p) = parse_machine(
            "Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        );
        assert_eq!(min_tokens_to_win_price(a, b, p), 0);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/2024/day_13")), 36571);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.to_string()), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/2024/day_13")), 0)
    }
}
