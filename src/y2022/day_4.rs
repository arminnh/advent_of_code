use std::fs;

/// To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a big list
/// of the section assignments for each pair (your puzzle input). Some of the pairs have noticed that one
/// of their assignments fully contains the other. For example, 2-8 fully contains 3-7, and 6-6 is fully
/// contained by 4-6. In pairs where one assignment fully contains the other, one Elf in the pair would
/// be exclusively cleaning sections their partner will already be cleaning, so these seem like the most
/// in need of reconsideration. In this example, there are 2 such pairs.
///
/// In how many assignment pairs does one range fully contain the other?
/// Part 2: In how many assignment pairs do the ranges overlap?
fn main() {
    let Ok(contents) = fs::read_to_string("inputs/day_4") else { return };

    let result: i32 = contents.lines().fold(0, |acc, line| {
        let values: Vec<i32> = line
            .split(|c| c == ',' || c == '-')
            .collect::<Vec<&str>>()
            .iter()
            .map(|v| v.parse::<i32>().unwrap())
            .collect();
        println!("{:?}", values);

        // PART 1
        // let left_contains_right: bool = values[0] <= values[2] && values[1] >= values[3];
        // let right_contains_left: bool = values[0] >= values[2] && values[1] <= values[3];
        // acc + if left_contains_right || right_contains_left {
        //     1
        // } else {
        //     0
        // }

        // PART 2
        let overlap: bool = values[1] >= values[2] && values[0] <= values[3];
        acc + if overlap { 1 } else { 0 }
    });
    println!("Number of overlaps: {}", result);
}
