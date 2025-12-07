// How many times does the beam get split?
pub fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let s = lines
        .next()
        .unwrap()
        .find('S')
        .expect("Could not find starting point");
    let mut beams = vec![s];
    let mut splits = 0;

    // Split beams from top to bottom while iterating through the input
    for line in lines {
        let splitters: Vec<usize> = line
            .char_indices()
            .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
            .collect();

        let mut next_beams = vec![];
        for beam in beams {
            if splitters.contains(&beam) {
                let split_left = !next_beams.contains(&(beam - 1));
                let split_right = !next_beams.contains(&(beam + 1));
                if split_left {
                    next_beams.push(beam - 1);
                }
                if split_right {
                    next_beams.push(beam + 1);
                }
                if split_left || split_right {
                    splits += 1;
                }
            } else {
                next_beams.push(beam);
            }
        }
        beams = next_beams;

        // println!(
        //     "{}",
        //     (0..line.len())
        //         .map(|i| {
        //             if splitters.contains(&i) {
        //                 '^'
        //             } else if beams.contains(&i) {
        //                 '|'
        //             } else {
        //                 '.'
        //             }
        //         })
        //         .collect::<String>()
        // );
    }

    splits
}

//
pub fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

    use super::*;

    const EXAMPLE_INPUT_1: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 21);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_7")), 1619);
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(part_2(EXAMPLE_INPUT_1), 0);
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(part_2(&load_input("inputs/2025/day_7")), 0);
    // }
}
