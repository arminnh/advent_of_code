fn parse_pairs(line: &str) -> Vec<i32> {
    line.split(|c| c == ',' || c == '-')
        .collect::<Vec<&str>>()
        .iter()
        .map(|v| v.parse::<i32>().unwrap())
        .collect()
}

// In how many assignment pairs does one range fully contain the other?
pub fn part_1(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        let pairs = parse_pairs(line);

        let left_contains_right: bool = pairs[0] <= pairs[2] && pairs[1] >= pairs[3];
        let right_contains_left: bool = pairs[0] >= pairs[2] && pairs[1] <= pairs[3];
        acc + if left_contains_right || right_contains_left {
            1
        } else {
            0
        }
    })
}

// In how many assignment pairs do the ranges overlap?
pub fn part_2(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        let pairs = parse_pairs(line);

        let overlap: bool = pairs[1] >= pairs[2] && pairs[0] <= pairs[3];
        acc + if overlap { 1 } else { 0 }
    })
}
