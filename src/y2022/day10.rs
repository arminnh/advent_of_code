use std::str::Lines;

// Figure out the signal being sent by the CPU. It has a single register, X, which starts
// with the value 1. It supports only two instructions:
// - addx V takes two cycles to complete. After two cycles, the X register is
//   increased by the value V. (V can be negative.)
// - noop takes one cycle to complete. It has no other effect.
pub fn part_1(input: &str) -> i64 {
    let mut lines = input.lines();
    let mut cycle = 0;
    let mut x = 1;
    let mut last_addx_value = 0;
    let mut strength = 0;

    loop {
        cycle += 1;

        if (cycle - 20) % 40 == 0 {
            strength += cycle * x;
        }

        if last_addx_value != 0 {
            x += last_addx_value;
            last_addx_value = 0;
            continue;
        }

        if let Some(line) = lines.next() {
            match line.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
                ["addx", num] => {
                    last_addx_value = num.parse::<i64>().unwrap();
                }
                _ => (),
            }
        } else {
            break;
        }
    }

    println!("\n-- END: Cycle '{}', x '{}' -> {}\n", cycle, x, strength);
    strength
}

// CRT: 40 wide and 6 high. Draws pixels left-to-right from position 0 to position 39, row per row.
// The CRT draws a single pixel during each cycle. If the sprite is positioned such that one of its
// three pixels is the pixel currently being drawn, the screen produces a lit pixel (#);
// otherwise, the screen leaves the pixel dark (.). The X register sets the horizontal position of
// the middle of the sprite, which is 3 pixels wide.
pub fn part_2(input: &str) -> String {
    let mut lines = input.lines();
    let mut cycle = 0;
    let mut x = 1;
    let mut last_addx_value = 0;
    let mut out = String::new();

    loop {
        let pixel_col = cycle % 40;
        if pixel_col == 0 {
            out += "\n";
        }
        if pixel_col == x - 1 || pixel_col == x || pixel_col == x + 1 {
            out += "#";
        } else {
            out += " ";
        }
        cycle += 1;

        if last_addx_value != 0 {
            x += last_addx_value;
            last_addx_value = 0;
            continue;
        }

        if let Some(line) = lines.next() {
            match line.split_ascii_whitespace().collect::<Vec<&str>>()[..] {
                ["addx", num] => {
                    last_addx_value = num.parse::<i64>().unwrap();
                }
                _ => (),
            }
        } else {
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

    use super::*;

    #[test]
    fn test_part_1_example_1() {
        assert_eq!(part_1(&load_input("inputs/2022/day_10_example_1")), 0)
    }

    #[test]
    fn test_part_1_example_2() {
        assert_eq!(part_1(&load_input("inputs/2022/day_10_example_2")), 13140)
    }

    #[test]
    fn test_part_2() {
        let expected = "
##  ##  ##  ##  ##  ##  ##  ##  ##  ##  \n\
###   ###   ###   ###   ###   ###   ### \n\
####    ####    ####    ####    ####    \n\
#####     #####     #####     #####     \n\
######      ######      ######      ####\n\
#######       #######       #######     \n ";
        assert_eq!(
            part_2(&load_input("inputs/2022/day_10_example_2")),
            expected
        )
    }
}
