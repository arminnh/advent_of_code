use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

type HeightMap = Vec<Vec<usize>>;

fn parse_input(lines: Lines<BufReader<File>>) -> (HeightMap, (usize, usize), (usize, usize)) {
    let mut map: Vec<Vec<usize>> = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    lines.enumerate().for_each(|(i, line)| {
        println!("{i}, {line:?}");
        if let Ok(line) = line {
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
        }
    });

    (map, start, end)
}

fn part_1(lines: Lines<BufReader<File>>) -> usize {
    let (height_map, start, end) = parse_input(lines);
    println!("{height_map:#?}");
    println!("{start:?}");
    println!("{end:?}");
    todo!()
}

// fn part_2(lines: Lines<BufReader<File>>) -> usize {
//     0
// }

fn get_lines(path: &str) -> Lines<BufReader<File>> {
    BufReader::new(File::open(path).expect("Could not open file.")).lines()
}

fn main() {
    part_1(get_lines("inputs/day_12_example"));
    // part_2(get_lines("inputs/day_12"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(get_lines("inputs/day_12_example")), 0)
    }

    // #[test]
    // fn test_part_1() {
    //     assert_eq!(part_1(get_lines("inputs/day_12")), 0)
    // }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(get_lines("inputs/day_12_example")), 0)
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(get_lines("inputs/day_12")), 0)
    // }
}
