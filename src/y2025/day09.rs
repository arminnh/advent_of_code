use std::collections::HashSet;

type Point = (u32, u32);

// What is the largest area of any rectangle you can make?
pub fn part_1(input: &str) -> usize {
    let corners: Vec<Point> = parse_input(input);

    let mut max = 0;
    for i in 0..(corners.len() - 1) {
        for j in i + 1..corners.len() {
            max = max.max(area(&corners[i], &corners[j]));
        }
    }
    max
}

fn parse_input(input: &str) -> Vec<Point> {
    let corners: Vec<Point> = input
        .lines()
        .map(|line| line.split_once(",").expect("Could not split line"))
        .map(|(a, b)| {
            (
                a.parse::<u32>().expect("Could not parse first number"),
                b.parse::<u32>().expect("Could not parse second number"),
            )
        })
        .collect();
    corners
}

fn area(i: &Point, j: &Point) -> usize {
    let dx = (i.0.abs_diff(j.0) + 1) as usize;
    let dy = (i.1.abs_diff(j.1) + 1) as usize;
    dx * dy
}

// All the corners form a shape. What is the largest area of any rectangle that can be made
// by two of the given corners while fully fitting in this shape?
pub fn part_2(input: &str) -> usize {
    let corners: Vec<Point> = parse_input(input);
    // Map each position to its rank
    // This condenses the grid from 100_000 x 100_000 to about 500 x 500
    let mut ranks_x: Vec<u32> = corners.iter().copied().map(|(x, _)| x).collect();
    let mut ranks_y: Vec<u32> = corners.iter().copied().map(|(_, y)| y).collect();
    // println!("{}, {}", ranks_x.len(), ranks_y.len());
    ranks_x.sort();
    ranks_y.sort();
    let corners: Vec<Point> = corners
        .into_iter()
        .map(|(x, y)| {
            (
                // Add 1 to leave borders of the grid empty -- this does not alter the shape
                ranks_x.iter().position(|r| *r == x).unwrap() as u32 + 1,
                ranks_y.iter().position(|r| *r == y).unwrap() as u32 + 1,
            )
        })
        .collect();

    // Shape is irregular rectilinear polygon
    // Grid == true for positions that are inside the polygon
    let mut grid: Vec<Vec<bool>> = vec![vec![false; ranks_y.len() + 1]; ranks_x.len() + 1];
    // The edges are subsequent corners in the input
    for i in 0..corners.len() {
        let j = if i == corners.len() - 1 { 0 } else { i + 1 };
        let (from_x, from_y) = corners[i];
        let (to_x, to_y) = corners[j];
        for x in from_x.min(to_x)..=from_x.max(to_x) {
            for y in from_y.min(to_y)..=from_y.max(to_y) {
                grid[x as usize][y as usize] = true;
            }
        }
    }

    // Mark the inside of the polygon as true
    // print_grid(&grid);
    flood_fill(&mut grid);
    // print_grid(&grid);

    let mut max = 0;
    for i in 0..(corners.len() - 1) {
        for j in i + 1..corners.len() {
            if valid_rectangle(&corners[i], &corners[j], &grid) {
                let corner1 = (
                    ranks_x[corners[i].0 as usize],
                    ranks_y[corners[i].1 as usize],
                );
                let corner2 = (
                    ranks_x[corners[j].0 as usize],
                    ranks_y[corners[j].1 as usize],
                );
                max = max.max(area(&corner1, &corner2));
            }
        }
    }
    max
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid.clone() {
        println!(
            "{}",
            row.iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>()
        );
    }
    println!();
}

// Flood fill the inside of the polygon
fn flood_fill(grid: &mut [Vec<bool>]) {
    let max_x = grid.len() - 1;
    let max_y = grid[0].len() - 1;
    // Flood fill the outside of the shape
    // Then flip grid to true for positions not visited
    let mut frontier: Vec<(usize, usize)> =
        Vec::from([(0, 0), (max_x / 2, 0), (0, max_y / 2), (max_x, max_y)]);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some(current) = frontier.pop() {
        if visited.insert(current) {
            for neighbor in neighbors(&current, max_x, max_y) {
                if let Some(next_pos) = neighbor {
                    if !visited.contains(&next_pos) && !grid[next_pos.0][next_pos.1] {
                        frontier.push(next_pos);
                    }
                }
            }
        }
    }

    for x in 0..=max_x {
        for y in 0..=max_y {
            if !visited.contains(&(x, y)) {
                grid[x][y] = true;
            }
        }
    }
}

fn neighbors(p: &(usize, usize), max_x: usize, max_y: usize) -> [Option<(usize, usize)>; 4] {
    [
        if p.0 < max_x {
            Some((p.0 + 1, p.1))
        } else {
            None
        },
        if p.0 > 0 { Some((p.0 - 1, p.1)) } else { None },
        if p.1 < max_y {
            Some((p.0, p.1 + 1))
        } else {
            None
        },
        if p.1 > 0 { Some((p.0, p.1 - 1)) } else { None },
    ]
}

// Rectangle is valid if all points on the edges are inside the grid
fn valid_rectangle(i: &Point, j: &Point, grid: &[Vec<bool>]) -> bool {
    let x1 = i.0.min(j.0) as usize;
    let x2 = i.0.max(j.0) as usize;
    let y1 = i.1.min(j.1) as usize;
    let y2 = i.1.max(j.1) as usize;
    let edges = [
        (x1, y1, x1, y2),
        (x1, y1, x2, y1),
        (x1, y2, x2, y2),
        (x2, y1, x2, y2),
    ];

    for e in edges {
        for x in e.0..=e.2 {
            for y in e.1..=e.3 {
                if !grid[x][y] {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

    use super::*;

    const EXAMPLE_INPUT_1: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    const EXAMPLE_INPUT_2: &str = "4,2
13,2
13,4
8,4
8,6
11,6
11,10
4,10";

    const EXAMPLE_INPUT_3: &str = "3,2
13,2
13,4
8,4
8,6
11,6
11,11
7,11
7,8
5,8
5,10
3,10";

    const EXAMPLE_INPUT_4: &str = "3,2
17,2
17,13
13,13
13,11
15,11
15,8
11,8
11,15
18,15
18,17
4,17
4,12
6,12
6,5
3,5";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 50);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_9")), 4_765_757_080);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1), 24);
        assert_eq!(part_2(EXAMPLE_INPUT_2), 40);
        assert_eq!(part_2(EXAMPLE_INPUT_3), 35);
        assert_eq!(part_2(EXAMPLE_INPUT_4), 66);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2025/day_9")), 1498673376);
    }
}
