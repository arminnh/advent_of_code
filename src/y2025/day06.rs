// Sum of all the math problems
pub fn part_1(input: &str) -> usize {
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let numbers: Vec<Vec<usize>> = lines[0..lines.len() - 1]
        .iter()
        .map(|line| {
            line.iter()
                .map(|n| n.parse::<usize>().expect("Could not parse number"))
                .collect()
        })
        .collect();
    let operators = &lines[lines.len() - 1];

    operators
        .into_iter()
        .enumerate()
        .map(|(i, op)| {
            numbers.iter().fold(0, |acc, numbers| {
                let num = numbers[i];
                if acc == 0 {
                    num
                } else {
                    match *op {
                        "*" => acc * num,
                        _ => acc + num,
                    }
                }
            })
        })
        .sum()
}

// Now the numbers are read in colums with the most significant digit at the top
pub fn part_2(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    let grid: Vec<Vec<Option<u32>>> = lines[0..lines.len() - 1]
        .iter()
        .map(|line| line.chars().map(|c| c.to_digit(10)).collect())
        .collect();
    let width = grid.iter().map(|row| row.len()).max().unwrap();
    let mut operators = lines[lines.len() - 1].split_whitespace();

    let mut total = 0;
    let mut current_nums: Vec<usize> = vec![];
    for i in 0..width {
        // Try parsing next number
        let digits = grid.iter().map(|row| row[i]);
        let next_number: usize = digits.fold(0, |acc, digit| {
            if let Some(d) = digit {
                acc * 10 + (d as usize)
            } else {
                acc
            }
        });

        if next_number != 0 {
            current_nums.push(next_number);
        }

        // Perform current calculation and reset
        if next_number == 0 || i == width - 1 {
            let current_operator = operators.next().unwrap();
            let result =
                current_nums.iter().skip(1).fold(
                    current_nums[0],
                    |acc, n| match current_operator {
                        "*" => acc * n,
                        _ => acc + n,
                    },
                );
            total += result;
            current_nums.clear();
            continue;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::util::util::load_input;

    use super::*;

    const EXAMPLE_INPUT_1: &str =
        "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_part_1_example() {
        assert_eq!(part_1(EXAMPLE_INPUT_1), 4277556);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&load_input("inputs/2025/day_6")), 4_405_895_212_738);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(part_2(EXAMPLE_INPUT_1), 3263827);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&load_input("inputs/2025/day_6")), 7450962_489_289);
    }
}
