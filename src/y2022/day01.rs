// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
pub fn part_1(input: &str) -> u32 {
    let elves: std::str::Split<&str> = input.split("\n\n");
    elves
        .map(|elf: &str| {
            elf.split_ascii_whitespace().fold(0, |acc, calories_str| {
                acc + calories_str.parse::<u32>().unwrap()
            })
        })
        .max()
        .unwrap()
}

// How many Calories are the top 3 Elves carrying in total?
pub fn part_2(input: &str) -> u32 {
    let elves: std::str::Split<&str> = input.split("\n\n");
    let mut calories_per_elf: Vec<u32> = elves
        .map(|elf: &str| {
            elf.split_ascii_whitespace().fold(0, |acc, calories_str| {
                acc + calories_str.parse::<u32>().unwrap()
            })
        })
        .collect::<Vec<u32>>();
    calories_per_elf.sort_unstable();

    calories_per_elf
        .get(calories_per_elf.len() - 3..)
        .unwrap()
        .iter()
        .sum::<u32>()
}
