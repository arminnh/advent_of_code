use std::fs;
// use std::fs::File;
// use std::io::{BufRead, BufReader};

/// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
fn day_1_part_1() {
    // let file: File = File::open("inputs/day_1").unwrap();
    // BufReader::new(file)
    //     .split(b'\n')
    //     .for_each(|elf| println!("elf: {:?}", elf.unwrap()));
    // .for_each(|line: Result<String, io::Error>| println!("{}", line.unwrap()));

    let contents = fs::read_to_string("inputs/day_1");
    match contents {
        Ok(c) => {
            let elves: std::str::Split<&str> = c.split("\n\n");
            let calories_per_elf = elves.map(|elf: &str| {
                elf.split_ascii_whitespace().fold(0, |acc, calories_str| {
                    acc + calories_str.parse::<u32>().unwrap()
                })
            });
            println!("{:?}", calories_per_elf);
            println!("{:?}", calories_per_elf.max());
        }
        Err(err) => println!("Error reading file: {}", err),
    }
}

/// By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually run out of snacks. To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the top three Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups. In the example above, the top three Elves are the fourth Elf (with 24000 Calories), then the third Elf (with 11000 Calories), then the fifth Elf (with 10000 Calories). The sum of the Calories carried by these three elves is 45000.
///
/// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
fn day_1_part_2() {
    let contents = fs::read_to_string("inputs/day_1");
    match contents {
        Ok(c) => {
            let elves: std::str::Split<&str> = c.split("\n\n");
            let mut calories_per_elf: Vec<u32> = elves
                .map(|elf: &str| {
                    elf.split_ascii_whitespace().fold(0, |acc, calories_str| {
                        acc + calories_str.parse::<u32>().unwrap()
                    })
                })
                .collect::<Vec<u32>>();
            calories_per_elf.sort_unstable();
            println!("{:?}", calories_per_elf);
            println!("{:?}", calories_per_elf.len());
            println!(
                "{:?}",
                calories_per_elf
                    .get(calories_per_elf.len() - 3..)
                    .unwrap()
                    .iter()
                    .sum::<u32>()
            );
        }
        Err(err) => println!("Error reading file: {}", err),
    }
}

fn main() {
    // day_1_part_1()
    day_1_part_2()
}
