use crate::days::util::load_input;
use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::Lines;
use std::usize;

type Grid = Vec<Vec<Tile>>;

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
enum Tile {
    Sphere,
    Cube,
    Empty,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Tile::Sphere => 'O',
            Tile::Cube => '#',
            Tile::Empty => '.',
        };
        write!(f, "{}", out)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn parse_grid(lines: Lines) -> Grid {
    let grid: Grid = lines
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'O' => Tile::Sphere,
                    '#' => Tile::Cube,
                    _ => Tile::Empty,
                })
                .collect()
        })
        .collect();
    grid
}

fn print_grid(grid: &Grid) {
    grid.iter().for_each(|row| {
        row.iter().for_each(|c| print!("{c}"));
        println!("");
    });
    println!("");
}

fn tilt(grid: &Grid, direction: Direction) -> Grid {
    match direction {
        Direction::North | Direction::South => tilt_vertical(grid, direction),
        Direction::East | Direction::West => tilt_horizontal(grid, direction),
    }
}

fn tilt_vertical(grid: &Grid, direction: Direction) -> Grid {
    let mut grid = grid.clone();
    for j in 0..grid[0].len() {
        let i_iter: Vec<usize> = match direction {
            Direction::North => (0..grid.len()).collect(),
            _ => (0..grid.len()).rev().collect(),
        };

        let mut open_spot: Option<usize> = None;
        for i in i_iter {
            match grid[i][j] {
                Tile::Sphere => match open_spot {
                    Some(prev_index) => {
                        grid[prev_index][j] = Tile::Sphere;
                        grid[i][j] = Tile::Empty;
                        open_spot = Some(match direction {
                            Direction::North => prev_index + 1,
                            _ => prev_index - 1,
                        });
                    }
                    None => (),
                },
                Tile::Cube => open_spot = None,
                Tile::Empty => match open_spot {
                    Some(_) => (),
                    None => open_spot = Some(i),
                },
            }
        }
    }
    grid
}

fn tilt_horizontal(grid: &Grid, direction: Direction) -> Grid {
    let mut grid = grid.clone();
    for i in 0..grid.len() {
        let j_iter: Vec<usize> = match direction {
            Direction::West => (0..grid[0].len()).collect(),
            _ => (0..grid[0].len()).rev().collect(),
        };

        let mut open_spot: Option<usize> = None;
        for j in j_iter {
            match grid[i][j] {
                Tile::Sphere => match open_spot {
                    Some(prev_index) => {
                        grid[i][prev_index] = Tile::Sphere;
                        grid[i][j] = Tile::Empty;
                        open_spot = Some(match direction {
                            Direction::West => prev_index + 1,
                            _ => prev_index - 1,
                        });
                    }
                    None => (),
                },
                Tile::Cube => open_spot = None,
                Tile::Empty => match open_spot {
                    Some(_) => (),
                    None => open_spot = Some(j),
                },
            }
        }
    }
    grid
}

fn total_load(grid: &Grid) -> usize {
    grid.iter()
        .enumerate()
        .map(|(i, row)| (grid.len() - i) * row.iter().filter(|&c| c == &Tile::Sphere).count())
        .sum()
}

fn part_1(lines: Lines) -> usize {
    let mut grid = parse_grid(lines);
    grid = tilt(&grid, Direction::North);
    total_load(&grid)
}

fn part_2(lines: Lines, nr_of_cycles: usize) -> usize {
    let mut grid = parse_grid(lines);
    // Keep track of when a repeating grid was last seen
    let mut cache: HashMap<Grid, (Grid, usize)> = HashMap::new();
    let directions_cycle = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    for i in 0..nr_of_cycles {
        if let Some((final_grid, seen_at)) = cache.get(&grid) {
            if (nr_of_cycles - i - 1) % (i - seen_at) == 0 {
                // The remaining number of cycles is a multiple of the cycle length
                // -> can skip the rest of the iterations and return the seen grid
                return total_load(final_grid);
            }
        }

        let old_grid = grid.clone();
        for &direction in directions_cycle.iter() {
            grid = tilt(&grid, direction);
        }

        cache.insert(old_grid, (grid.clone(), i));
    }

    total_load(&grid)
}

pub fn solve() -> SolutionPair {
    let input = load_input("inputs/day_14");
    (
        Solution::from(part_1(input.lines())),
        Solution::from(part_2(input.lines(), 1_000_000_000)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    const EXAMPLE_INPUT_NORTH: &str = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    const EXAMPLE_INPUT_WEST: &str = "OOOO.#O...
OO..#....#
OOO..##O..
O..#OO....
........#.
..#....#.#
O....#OO..
O.........
#....###..
#....#....";

    const EXAMPLE_INPUT_SOUTH: &str = ".....#....
....#.O..#
O..O.##...
O.O#......
O.O....O#.
O.#..O.#.#
O....#....
OO....OO..
#O...###..
#O..O#....";

    const EXAMPLE_INPUT_EAST: &str = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

    const AFTER_2_CYCLES: &str = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";

    const AFTER_3_CYCLES: &str = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";

    #[test]
    fn test_total_load() {
        assert_eq!(total_load(&parse_grid(EXAMPLE_INPUT.lines())), 104);
        assert_eq!(total_load(&parse_grid(EXAMPLE_INPUT_NORTH.lines())), 136);
        assert_eq!(total_load(&parse_grid(AFTER_3_CYCLES.lines())), 69);
    }

    #[test]
    fn test_tilt_north_simple() {
        let grid = tilt(
            &parse_grid("...O\n.#O.\n..##\nOO.O".lines()),
            Direction::North,
        );
        assert_eq!(grid, parse_grid("O.OO\n.#..\n.O##\n...O".lines()));
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT.lines()), 136);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(load_input("inputs/day_14").lines()), 109385);
    }

    #[test]
    fn test_tilt_north_example() {
        let grid = tilt(&parse_grid(EXAMPLE_INPUT.lines()), Direction::North);
        assert_eq!(grid, parse_grid(EXAMPLE_INPUT_NORTH.lines()));
    }

    #[test]
    fn test_tilt_west_example() {
        let grid = tilt(&parse_grid(EXAMPLE_INPUT_NORTH.lines()), Direction::West);
        assert_eq!(grid, parse_grid(EXAMPLE_INPUT_WEST.lines()));
    }

    #[test]
    fn test_tilt_south_example() {
        let grid = tilt(&parse_grid(EXAMPLE_INPUT_WEST.lines()), Direction::South);
        assert_eq!(grid, parse_grid(EXAMPLE_INPUT_SOUTH.lines()));
    }

    #[test]
    fn test_tilt_east_example() {
        let grid = tilt(&parse_grid(EXAMPLE_INPUT_SOUTH.lines()), Direction::East);
        assert_eq!(grid, parse_grid(EXAMPLE_INPUT_EAST.lines()));
    }

    #[test]
    fn test_tilt_cycles() {
        let mut grid = parse_grid(EXAMPLE_INPUT.lines());
        use Direction as D;
        let cycle = [D::North, D::West, D::South, D::East];

        cycle.iter().for_each(|d| grid = tilt(&grid, *d));
        assert_eq!(grid, parse_grid(EXAMPLE_INPUT_EAST.lines()));

        cycle.iter().for_each(|d| grid = tilt(&grid, *d));
        assert_eq!(grid, parse_grid(AFTER_2_CYCLES.lines()));

        cycle.iter().for_each(|d| grid = tilt(&grid, *d));
        assert_eq!(grid, parse_grid(AFTER_3_CYCLES.lines()));
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT.lines(), 1_000_000_000), 64);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(load_input("inputs/day_14").lines(), 1_000_000_000),
            93102
        );
    }
}
