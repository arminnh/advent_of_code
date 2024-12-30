type HeightMap = Vec<Vec<usize>>;

fn parse_input(input: &str) -> (HeightMap, (usize, usize), (usize, usize)) {
    let mut map: Vec<Vec<usize>> = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    input.lines().enumerate().for_each(|(i, line)| {
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

pub fn part_1(input: &str) -> usize {
    let (height_map, start, end) = parse_input(input);
    println!("{height_map:#?}");
    println!("{start:?}");
    println!("{end:?}");
    todo!()
}

pub fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(&load_input("inputs/2022/day_12_example")), 0)
    }

    // #[test]
    // fn test_part_1() {
    //     assert_eq!(part_1(&load_input("inputs/2022/day_12").lines()), 0)
    // }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(&load_input("inputs/2022/day_12_example").lines()), 0)
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(&load_input("inputs/2022/day_12").lines()), 0)
    // }
}
