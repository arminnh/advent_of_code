use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use image::{ImageBuffer, Rgb, RgbImage};
use std::collections::HashSet;
use std::str::Lines;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn translate(&self, direction: Direction, distance: i32) -> Point {
        let (x_diff, y_diff) = direction.to_coords();
        Point {
            x: self.x + x_diff * distance,
            y: self.y + y_diff * distance,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn from_str(s: &str) -> Self {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction {:?}", s),
        }
    }

    fn to_coords(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn angle(&self, other: Direction) -> i32 {
        match self {
            Direction::Up => match other {
                Direction::Up => 0,
                Direction::Down => 180,
                Direction::Left => -90,
                Direction::Right => 90,
            },
            Direction::Down => match other {
                Direction::Up => 180,
                Direction::Down => 0,
                Direction::Left => 90,
                Direction::Right => -90,
            },
            Direction::Left => match other {
                Direction::Up => 90,
                Direction::Down => -90,
                Direction::Left => 0,
                Direction::Right => 180,
            },
            Direction::Right => match other {
                Direction::Up => -90,
                Direction::Down => 90,
                Direction::Left => 180,
                Direction::Right => 0,
            },
        }
    }
}

#[allow(dead_code)]
fn in_bounds(p: &Point, max_x: i32, max_y: i32) -> bool {
    p.x > 0 && p.x < max_x && p.y > 0 && p.y < max_y
}

#[allow(dead_code)]
fn parse_lagoon(lines: Lines<'_>) -> HashSet<Point> {
    let mut lagoon: HashSet<Point> = HashSet::new();

    lines.fold(Point::from(0, 0), |position, line| {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [direction, count, _] => (0..count.parse::<i32>().unwrap()).fold(position, |p, _| {
                let (x_diff, y_diff) = Direction::from_str(direction).to_coords();
                let next = Point::from(p.x + x_diff, p.y + y_diff);
                lagoon.insert(next);
                next
            }),
            _ => panic!("Invalid line {:?}", line),
        }
    });

    // shift all coordinates to be >= 0
    let min_x = lagoon.iter().min_by_key(|pos| pos.x).unwrap().x;
    let min_y = lagoon.iter().min_by_key(|pos| pos.y).unwrap().y;
    lagoon
        .iter()
        .map(|p| Point::from(p.x - min_x, p.y - min_y))
        .collect()
}

#[allow(dead_code)]
fn neighbors(p: &Point, max_x: i32, max_y: i32) -> Vec<Point> {
    vec![
        Point::from(p.x + 1, p.y),
        Point::from(p.x - 1, p.y),
        Point::from(p.x, p.y + 1),
        Point::from(p.x, p.y - 1),
    ]
    .into_iter()
    .filter(|p| in_bounds(p, max_x, max_y))
    .collect()
}

#[allow(dead_code)]
fn print_lagoon(lagoon: &HashSet<Point>, highlight: Option<&HashSet<Point>>) {
    let max_x = lagoon.iter().max_by_key(|pos| pos.x).unwrap().x;
    let max_y = lagoon.iter().max_by_key(|pos| pos.y).unwrap().y;

    for i in 0..=max_x {
        for j in 0..=max_y {
            if highlight.is_some() && highlight.unwrap().contains(&Point::from(i, j)) {
                print!("*");
            } else if lagoon.contains(&Point::from(i, j)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    println!("");
}

// Flood fill from the edges of the bounding box around the lagoon
#[allow(dead_code)]
fn flood_fill(lagoon: &HashSet<Point>) -> usize {
    let mut frontier: Vec<Point> = Vec::new();
    let mut visited: HashSet<Point> = HashSet::new();

    // fill frontier with edges of the map
    let max_x = lagoon.iter().max_by_key(|pos| pos.x).unwrap().x;
    let max_y = lagoon.iter().max_by_key(|pos| pos.y).unwrap().y;
    let left_and_right_edges = (0..=max_x).into_iter().flat_map(|i| [(i, 0), (i, max_y)]);
    let top_and_bottom_edges = (0..=max_y).into_iter().flat_map(|j| [(0, j), (max_x, j)]);
    left_and_right_edges
        .chain(top_and_bottom_edges)
        .map(|(x, y)| Point::from(x, y))
        .filter(|pos| !lagoon.contains(pos))
        .for_each(|pos| frontier.push(pos));

    while let Some(current) = frontier.pop() {
        if !visited.contains(&current) {
            visited.insert(current);

            for next in neighbors(&current, max_x, max_y) {
                if !visited.contains(&next) && !lagoon.contains(&next) {
                    frontier.push(next);
                }
            }
        }
    }

    // print_lagoon(&lagoon, Some(&visited));
    (max_x as usize + 1) * (max_y as usize + 1) - visited.len()
}

// Calculate the volume of the lagoon formed by the perimeter. Each position is a 1 meter cube.
fn part_1_flood_fill(lines: Lines) -> usize {
    let lagoon = parse_lagoon(lines);
    // print_lagoon(&lagoon, None);
    flood_fill(&lagoon)
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct Edge {
    from: Point,
    to: Point,
    direction: Direction,
    distance: i32,
    color: Rgb<u8>,
}

impl Edge {
    fn from(from: Point, to: Point, direction: Direction, distance: i32, color: Rgb<u8>) -> Self {
        Edge {
            from,
            to,
            direction,
            distance,
            color,
        }
    }

    // fn extend(&mut self, distance: i32) -> &Self {
    //     self.to = next_position(self.to, self.direction, distance);
    //     self.distance += distance;
    //     self
    // }

    // fn shrink(&mut self, distance: i32) -> &Self {
    //     self.to = next_position(self.to, self.direction.opposite(), distance);
    //     self.distance -= distance;
    //     self
    // }

    // fn translate(&mut self, direction: Direction, distance: i32) -> &Self {
    //     self.from = next_position(self.from, direction, distance);
    //     self.to = next_position(self.to, direction, distance);
    //     self
    // }

    fn extend(&self, distance: i32) -> Self {
        Edge {
            from: self.from,
            to: self.to.translate(self.direction, distance),
            direction: self.direction,
            distance: self.distance + distance,
            color: self.color,
        }
    }

    fn shrink(&self, distance: i32) -> Self {
        Edge {
            from: self.from,
            to: self.to.translate(self.direction.opposite(), distance),
            direction: self.direction,
            distance: self.distance - distance,
            color: self.color,
        }
    }

    fn translate(&self, direction: Direction, distance: i32) -> Self {
        Edge {
            from: self.from.translate(direction, distance),
            to: self.to.translate(direction, distance),
            direction: self.direction,
            distance: self.distance,
            color: self.color,
        }
    }
}

fn edges_to_lagoon(edges: &[Edge]) -> HashSet<Point> {
    let mut out = HashSet::new();
    edges.iter().fold(Point::from(0, 0), |position, edge| {
        let (x_diff, y_diff) = edge.direction.to_coords();
        (0..edge.distance).fold(position, |p, _| {
            let next = Point::from(p.x + x_diff, p.y + y_diff);
            out.insert(next);
            next
        })
    });
    out
}

fn edge_to_positions(direction: Direction, distance: i32, position: Point) -> HashSet<Point> {
    let mut edge_positions = HashSet::new();
    let (x_diff, y_diff) = direction.to_coords();
    (0..distance).fold(position, |p, _| {
        let next = Point::from(p.x + x_diff, p.y + y_diff);
        edge_positions.insert(next);
        next
    });
    edge_positions
}

fn parse_edges(lines: Lines) -> Vec<Edge> {
    let mut edges = Vec::new();
    lines.fold(Point::from(0, 0), |from, line| -> Point {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [direction, distance, color] => {
                let distance = distance.parse::<i32>().unwrap();
                let direction = Direction::from_str(direction);
                let to = from.translate(direction, distance);
                let color = Rgb([
                    u8::from_str_radix(&color[2..4], 16).unwrap(),
                    u8::from_str_radix(&color[4..6], 16).unwrap(),
                    u8::from_str_radix(&color[6..8], 16).unwrap(),
                ]);
                edges.push(Edge::from(from, to, direction, distance, color));
                to
            }
            _ => panic!("Invalid line {:?}", line),
        }
    });
    edges
}

fn point_in_rectangle(point: Point, rectangle_from: Point, rectangle_to: Point) -> bool {
    let x_from = rectangle_from.x.min(rectangle_to.x);
    let x_to = rectangle_from.x.max(rectangle_to.x);
    let y_from = rectangle_from.y.min(rectangle_to.y);
    let y_to = rectangle_from.y.max(rectangle_to.y);

    point.x >= x_from && point.x <= x_to && point.y >= y_from && point.y <= y_to
}

// Rectangle can be sliced if no other edge lies in it
fn can_slice(
    edges: &Vec<Edge>,
    grandparent_index: usize,
    grandparent: &Edge,
    current: &Edge,
    angle: i32,
) -> bool {
    if grandparent.direction != current.direction.opposite() || angle < 0 {
        return false;
    }

    let (rectangle_from, rectangle_to) = if current.distance == grandparent.distance {
        (grandparent.from, current.from)
    } else if current.distance < grandparent.distance {
        (grandparent.to, current.to)
    } else {
        (grandparent.from, current.from)
    };

    edges.iter().enumerate().all(|(j, e)| {
        !point_in_rectangle(e.from, rectangle_from, rectangle_to)
            || (grandparent_index <= j && j <= grandparent_index + 4)
            || (grandparent_index >= edges.len() - 2 && j < 1)
    })
}

// Calculate area by slicing rectangles off of the polygon until only one rectangle remains
// This whole thing would have been much easier by searching rectangles based on the corner points,
// but I wanted to make it work using the directions in the given input.
fn area_of_rectilinear_polygon(mut edges: Vec<Edge>) -> i32 {
    let mut area = 0;
    let mut rectangles: Vec<(Edge, Edge)> = Vec::new();

    while edges.len() > 4 {
        let mut next_edges: Vec<Edge> = Vec::with_capacity(edges.len());
        let mut i = 0;
        let mut angle = edges[i].direction.angle(edges[i + 1].direction);
        while i < edges.len() {
            let grandparent = edges[i];
            let parent = edges[(i + 1) % edges.len()];
            let current = edges[(i + 2) % edges.len()];
            angle += parent.direction.angle(current.direction);

            if !can_slice(&edges, i, &grandparent, &current, angle)
                || next_edges.len() + edges.len() - i == 4
            {
                next_edges.push(grandparent);
                println!("{i}, cannot slice");
                i += 1;
                continue;
            }

            angle = 0;
            if current.distance == grandparent.distance {
                println!("{} -> Clean cut", i);
                // Clean cut -> connect before & after rectangle by extending the last pushed edge
                area += current.distance * (parent.distance + 1);
                rectangles.push((parent, current));

                let next = &edges[i + 3];
                // extend most recent new edge by the length of the sliced rectangle
                // and by the next edge after the rectangle
                let most_recent = next_edges.last_mut().unwrap();
                if next.direction == parent.direction {
                    *most_recent = most_recent.extend(parent.distance + next.distance);
                } else {
                    area += next.distance; // add area of sliced edge which will not be part of a future rectangle
                    *most_recent = most_recent.extend(parent.distance - next.distance);
                }

                i += 4;
            } else if current.distance < grandparent.distance {
                println!("{} -> Current edge is shorter", i);
                // Current edge is shorter -> shorten grandparent, drop parent and current, move and extend next
                area += current.distance * (parent.distance + 1);
                rectangles.push((parent, current));

                // shorten grandparent
                let new_grandparent = grandparent.shrink(current.distance);
                next_edges.push(new_grandparent);

                // move and extend next
                let next = if i < edges.len() - 2 {
                    edges[i + 3]
                } else {
                    if i == edges.len() - 1 {
                        // pop parent
                        next_edges.remove(0);
                    }
                    // pop current
                    next_edges.remove(0);
                    // pop and return next
                    next_edges.remove(0)
                };
                if next.direction == parent.direction {
                    next_edges.push(
                        next.translate(next.direction.opposite(), parent.distance)
                            .extend(parent.distance),
                    );
                } else {
                    area += next.distance; // add area of sliced edge which will not be part of a future rectangle
                    next_edges.push(Edge {
                        from: new_grandparent.to,
                        to: next.to,
                        direction: next.direction.opposite(),
                        distance: parent.distance - next.distance,
                        color: next.color,
                    });
                }

                i += 4;
            } else {
                println!("{} -> Current edge is longer", i);
                // Current edge is longer -> drop grandparent & parent, extend great grandparent, move and shrink current
                area += grandparent.distance * (parent.distance + 1);
                rectangles.push((parent, grandparent));

                // Extend great grandparent
                if i == 0 {
                    // great grandparent is actually last element of the list of edges currently iterating over
                    let xxxx = edges.len() - 1;
                    let great_grandparent = edges[xxxx];
                    edges[xxxx] = great_grandparent.extend(parent.distance);
                } else {
                    let great_grandparent = next_edges.last_mut().unwrap();
                    *great_grandparent = great_grandparent.extend(parent.distance);
                }

                // move and shrink current
                if i < edges.len() - 2 {
                    println!("{i:?} / {} -> aaaaaaaaaaaaaaaaaaaa", edges.len());
                    next_edges.push(
                        current
                            .translate(current.direction, grandparent.distance)
                            .shrink(grandparent.distance),
                    );
                } else {
                    if i == edges.len() - 1 {
                        next_edges.remove(0);
                    }
                    println!("{i:?} / {} -> bbbbbbbbbbbbbbbbbbbbb", edges.len());
                    // next edge is actually first edge of the next_edges vec, so update that one
                    next_edges[0] = next_edges[0]
                        .translate(current.direction, grandparent.distance)
                        .shrink(grandparent.distance);
                }

                i += 3;
            }

            // next_edges.iter().for_each(|e| println!("{e:?}"));
            // draw_lagoon(&edges, &rectangles);
            // next_edges.append(&mut edges[i..].to_vec());
            // break;
        }
        edges = next_edges;
        edges.iter().for_each(|e| println!("{e:?}"));
        draw_lagoon(&edges, &rectangles);
    }

    rectangles.push((edges[0], edges[1]));
    draw_lagoon(&Vec::new(), &rectangles);

    area + (edges[0].distance + 1) * (edges[1].distance + 1)
}

// Calculate the volume of the lagoon formed by the perimeter. Each position is a 1 meter cube.
fn part_1(lines: Lines) -> i32 {
    let edges = parse_edges(lines);
    draw_lagoon(&edges, &Vec::new());
    area_of_rectilinear_polygon(edges)
}

fn draw_lagoon(edges: &Vec<Edge>, rectangles: &Vec<(Edge, Edge)>) {
    let (width, height): (u32, u32) = (800, 800);
    let (x_min, x_max, y_min, y_max) = edges
        .iter()
        .chain(rectangles.iter().flat_map(|(e1, e2)| vec![e1, e2]))
        .fold(
            (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
            |(x_min, x_max, y_min, y_max), edge| {
                (
                    x_min.min(edge.from.y),
                    x_max.max(edge.from.y),
                    y_min.min(edge.from.x),
                    y_max.max(edge.from.x),
                )
            },
        );
    let x_range = x_max - x_min;
    let y_range = y_max - y_min;
    let x_scaled = |x| ((x - x_min) as f64 * (width - 1) as f64 / x_range as f64).round() as u32;
    let y_scaled = |y| ((y - y_min) as f64 * (height - 1) as f64 / y_range as f64).round() as u32;

    let mut img = ImageBuffer::new(width, height);
    for (e1, e2) in rectangles {
        draw_line(
            &mut img,
            x_scaled(
                [e1.from.y, e1.to.y, e2.from.y, e2.to.y]
                    .into_iter()
                    .min()
                    .unwrap(),
            ),
            y_scaled(
                [e1.from.x, e1.to.x, e2.from.x, e2.to.x]
                    .into_iter()
                    .min()
                    .unwrap(),
            ),
            x_scaled(
                [e1.from.y, e1.to.y, e2.from.y, e2.to.y]
                    .into_iter()
                    .max()
                    .unwrap(),
            ),
            y_scaled(
                [e1.from.x, e1.to.x, e2.from.x, e2.to.x]
                    .into_iter()
                    .max()
                    .unwrap(),
            ),
            e1.color,
        );
    }
    for (iiii, edge) in edges.iter().enumerate() {
        let (x0, y0) = (x_scaled(edge.from.y), y_scaled(edge.from.x));
        let (x1, y1) = (x_scaled(edge.to.y), y_scaled(edge.to.x));
        if iiii == edges.len() - 1 {
            draw_line(&mut img, x0, y0, x1, y1, Rgb([255, 255, 255]));
        } else {
            draw_line(&mut img, x0, y0, x1, y1, edge.color);
        }
    }
    // img.save(format!("outputs/day_18_{}.png", rectangles.len()))
    img.save("outputs/day_18.png")
        .expect("Failed to save image");

    // let mut input = String::new();
    // std::io::stdin()
    //     .read_line(&mut input)
    //     .expect("can not read user input");
}

fn draw_line(img: &mut RgbImage, x0: u32, y0: u32, x1: u32, y1: u32, color: Rgb<u8>) {
    for x in x0.min(x1)..=x1.max(x0) {
        for y in y0.min(y1)..=y1.max(y0) {
            img.put_pixel(x, y, color);
        }
    }
}

fn part_2(_lines: Lines) -> usize {
    0
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_18");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines())),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part_1_flood_fill() {
        assert_eq!(part_1_flood_fill(EXAMPLE_INPUT_1.lines()), 62);
        assert_eq!(
            part_1_flood_fill(load_input("inputs/day_18_shortened").lines()),
            5532
        );
        assert_eq!(
            part_1_flood_fill(load_input("inputs/day_18_shortened_2").lines()),
            17891
        );
        assert_eq!(
            part_1_flood_fill(load_input("inputs/day_18_shortened_3").lines()),
            1704
        );
        assert_eq!(
            part_1_flood_fill(load_input("inputs/day_18").lines()),
            33491
        );
    }

    #[test]
    fn test_part_1_rectangles_example() {
        let mut lines = EXAMPLE_INPUT_1.split("\n").collect::<Vec<&str>>();
        assert_eq!(part_1(lines.join("\n").lines()), 62);
        lines.rotate_left(1);
        assert_eq!(part_1(lines.join("\n").lines()), 62);
        lines.rotate_left(5);
        assert_eq!(part_1(lines.join("\n").lines()), 62);
        lines.rotate_left(1);
        assert_eq!(part_1(lines.join("\n").lines()), 62);
        lines.rotate_left(1);
        assert_eq!(part_1(lines.join("\n").lines()), 62);
        lines.rotate_left(1);
        assert_eq!(part_1(lines.join("\n").lines()), 62);
    }

    #[test]
    fn test_part_1_rectangles() {
        assert_eq!(part_1(load_input("inputs/day_18_shortened").lines()), 5532);
        assert_eq!(
            part_1(load_input("inputs/day_18_shortened_3").lines()),
            1704
        );
        assert_eq!(
            part_1(load_input("inputs/day_18_shortened_2").lines()),
            17891
        );
        assert_eq!(part_1(load_input("inputs/day_18").lines()), 33491);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1.lines()), 952408144115);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(load_input("inputs/day_18").lines()), 0);
    }
}
