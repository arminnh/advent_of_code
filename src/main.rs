mod util;
mod y2022;
mod y2023;
mod y2024;

use std::env;
use std::time::Instant;
use util::solution::Solution;
use util::util::load_input;

fn main() {
    let mut args: env::Args = env::args();
    args.next();
    let years: Vec<usize> = if let Some(y) = args.next() {
        Vec::from([y.parse().expect("Could not parse first argument to year.")])
    } else {
        (2022..=2024).collect()
    };
    let mut days: Vec<u8> = args
        .map(|x| {
            x.parse()
                .unwrap_or_else(|v| panic!("Not a valid day: {}", v))
        })
        .collect();
    if days.is_empty() {
        days = (1..=25).collect();
    };

    let mut runtime = 0.0;
    let mut times: Vec<(f64, usize, u8, usize)> = Vec::new();

    for year in years {
        println!("====== Year {} ======", year);
        for day in &days {
            println!("=== Day {:02} ===", day);
            let (part_1, part_2) = get_day_solvers(year, day);
            let input = load_input(&format!("inputs/{}/day_{}", year, day));

            let time = Instant::now();
            let p1 = part_1(&input);
            let elapsed_ms_part1 = time.elapsed().as_nanos() as f64 / 1_000_000.0;
            println!("  · Part 1 ({:>5.4} ms): {}", elapsed_ms_part1, p1);
            runtime += elapsed_ms_part1;

            let time = Instant::now();
            let p2 = part_2(&input);
            let elapsed_ms_part2 = time.elapsed().as_nanos() as f64 / 1_000_000.0;
            println!("  · Part 2 ({:>5.4} ms): {}\n", elapsed_ms_part2, p2);
            runtime += elapsed_ms_part2;

            times.push((elapsed_ms_part1, year, *day, 1));
            times.push((elapsed_ms_part2, year, *day, 2));
        }
    }

    println!("Total runtime ({} parts): {:.4} ms", times.len(), runtime);
    if times.len() > 10 {
        times.sort_by(|a, b| b.0.total_cmp(&a.0));
        println!("Slowest 5: {:?}", &times[..5]);
    }
}

fn get_day_solvers(year: usize, day: &u8) -> (fn(&str) -> Solution, fn(&str) -> Solution) {
    match year {
        2022 => match day {
            1 => (
                |input: &str| Solution::from(y2022::day01::part_1(input)),
                |input: &str| Solution::from(y2022::day01::part_2(input)),
            ),
            2 => (
                |input: &str| Solution::from(y2022::day02::part_1(input)),
                |input: &str| Solution::from(y2022::day02::part_2(input)),
            ),
            3 => (
                |input: &str| Solution::from(y2022::day03::part_1(input)),
                |input: &str| Solution::from(y2022::day03::part_2(input)),
            ),
            4 => (
                |input: &str| Solution::from(y2022::day04::part_1(input)),
                |input: &str| Solution::from(y2022::day04::part_2(input)),
            ),
            5 => (
                |input: &str| Solution::from(y2022::day05::part_1(input)),
                |input: &str| Solution::from(y2022::day05::part_2(input)),
            ),
            6 => (
                |input: &str| Solution::from(y2022::day06::part_1(input)),
                |input: &str| Solution::from(y2022::day06::part_2(input)),
            ),
            7 => (
                |input: &str| Solution::from(y2022::day07::part_1(input)),
                |input: &str| Solution::from(y2022::day07::part_2(input)),
            ),
            8 => (
                |input: &str| Solution::from(y2022::day08::part_1(input)),
                |input: &str| Solution::from(y2022::day08::part_2(input)),
            ),
            9 => (
                |input: &str| Solution::from(y2022::day09::part_1(input)),
                |input: &str| Solution::from(y2022::day09::part_2(input)),
            ),
            10 => (
                |input: &str| Solution::from(y2022::day10::part_1(input)),
                |input: &str| Solution::from(y2022::day10::part_2(input)),
            ),
            11 => (
                |input: &str| Solution::from(y2022::day11::part_1(input)),
                |input: &str| Solution::from(y2022::day11::part_2(input)),
            ),
            12 => (
                |input: &str| Solution::from(y2022::day12::part_1(input)),
                |input: &str| Solution::from(y2022::day12::part_2(input)),
            ),
            _ => unimplemented!(),
        },
        2023 => match day {
            1 => (
                |input: &str| Solution::from(y2023::day01::part_1(input)),
                |input: &str| Solution::from(y2023::day01::part_2(input)),
            ),
            2 => (
                |input: &str| Solution::from(y2023::day02::part_1(input)),
                |input: &str| Solution::from(y2023::day02::part_2(input)),
            ),
            3 => (
                |input: &str| Solution::from(y2023::day03::part_1(input)),
                |input: &str| Solution::from(y2023::day03::part_2(input)),
            ),
            4 => (
                |input: &str| Solution::from(y2023::day04::part_1(input)),
                |input: &str| Solution::from(y2023::day04::part_2(input)),
            ),
            5 => (
                |input: &str| Solution::from(y2023::day05::part_1(input)),
                |input: &str| Solution::from(y2023::day05::part_2(input)),
            ),
            6 => (
                |input: &str| Solution::from(y2023::day06::part_1(input)),
                |input: &str| Solution::from(y2023::day06::part_2(input)),
            ),
            7 => (
                |input: &str| Solution::from(y2023::day07::part_1(input)),
                |input: &str| Solution::from(y2023::day07::part_2(input)),
            ),
            8 => (
                |input: &str| Solution::from(y2023::day08::part_1(input)),
                |input: &str| Solution::from(y2023::day08::part_2(input)),
            ),
            9 => (
                |input: &str| Solution::from(y2023::day09::part_1(input)),
                |input: &str| Solution::from(y2023::day09::part_2(input)),
            ),
            10 => (
                |input: &str| Solution::from(y2023::day10::part_1(input)),
                |input: &str| Solution::from(y2023::day10::part_2(input)),
            ),
            11 => (
                |input: &str| Solution::from(y2023::day11::part_1(input)),
                |input: &str| Solution::from(y2023::day11::part_2(input)),
            ),
            12 => (
                |input: &str| Solution::from(y2023::day12::part_1(input)),
                |input: &str| Solution::from(y2023::day12::part_2(input)),
            ),
            13 => (
                |input: &str| Solution::from(y2023::day13::part_1(input)),
                |input: &str| Solution::from(y2023::day13::part_2(input)),
            ),
            14 => (
                |input: &str| Solution::from(y2023::day14::part_1(input)),
                |input: &str| Solution::from(y2023::day14::part_2(input)),
            ),
            15 => (
                |input: &str| Solution::from(y2023::day15::part_1(input)),
                |input: &str| Solution::from(y2023::day15::part_2(input)),
            ),
            16 => (
                |input: &str| Solution::from(y2023::day16::part_1(input)),
                |input: &str| Solution::from(y2023::day16::part_2(input)),
            ),
            17 => (
                |input: &str| Solution::from(y2023::day17::part_1(input)),
                |input: &str| Solution::from(y2023::day17::part_2(input)),
            ),
            18 => (
                |input: &str| Solution::from(y2023::day18::part_1(input)),
                |input: &str| Solution::from(y2023::day18::part_2(input)),
            ),
            19 => (
                |input: &str| Solution::from(y2023::day19::part_1(input)),
                |input: &str| Solution::from(y2023::day19::part_2(input)),
            ),
            20 => (
                |input: &str| Solution::from(y2023::day20::part_1(input)),
                |input: &str| Solution::from(y2023::day20::part_2(input)),
            ),
            21 => (
                |input: &str| Solution::from(y2023::day21::part_1(input)),
                |input: &str| Solution::from(y2023::day21::part_2(input)),
            ),
            22 => (
                |input: &str| Solution::from(y2023::day22::part_1(input)),
                |input: &str| Solution::from(y2023::day22::part_2(input)),
            ),
            23 => (
                |input: &str| Solution::from(y2023::day23::part_1(input)),
                |input: &str| Solution::from(y2023::day23::part_2(input)),
            ),
            24 => (
                |input: &str| Solution::from(y2023::day24::part_1(input)),
                |input: &str| Solution::from(y2023::day24::part_2(input)),
            ),
            25 => (
                |input: &str| Solution::from(y2023::day25::part_1(input)),
                |input: &str| Solution::from(y2023::day25::part_2(input)),
            ),
            _ => unimplemented!(),
        },
        2024 => match day {
            1 => (
                |input: &str| Solution::from(y2024::day01::part_1(input)),
                |input: &str| Solution::from(y2024::day01::part_2(input)),
            ),
            2 => (
                |input: &str| Solution::from(y2024::day02::part_1(input)),
                |input: &str| Solution::from(y2024::day02::part_2(input)),
            ),
            3 => (
                |input: &str| Solution::from(y2024::day03::part_1(input)),
                |input: &str| Solution::from(y2024::day03::part_2(input)),
            ),
            4 => (
                |input: &str| Solution::from(y2024::day04::part_1(input)),
                |input: &str| Solution::from(y2024::day04::part_2(input)),
            ),
            5 => (
                |input: &str| Solution::from(y2024::day05::part_1(input)),
                |input: &str| Solution::from(y2024::day05::part_2(input)),
            ),
            6 => (
                |input: &str| Solution::from(y2024::day06::part_1(input)),
                |input: &str| Solution::from(y2024::day06::part_2(input)),
            ),
            7 => (
                |input: &str| Solution::from(y2024::day07::part_1(input)),
                |input: &str| Solution::from(y2024::day07::part_2(input)),
            ),
            8 => (
                |input: &str| Solution::from(y2024::day08::part_1(input)),
                |input: &str| Solution::from(y2024::day08::part_2(input)),
            ),
            9 => (
                |input: &str| Solution::from(y2024::day09::part_1(input)),
                |input: &str| Solution::from(y2024::day09::part_2(input)),
            ),
            10 => (
                |input: &str| Solution::from(y2024::day10::part_1(input)),
                |input: &str| Solution::from(y2024::day10::part_2(input)),
            ),
            11 => (
                |input: &str| Solution::from(y2024::day11::part_1(input)),
                |input: &str| Solution::from(y2024::day11::part_2(input)),
            ),
            12 => (
                |input: &str| Solution::from(y2024::day12::part_1(input)),
                |input: &str| Solution::from(y2024::day12::part_2(input)),
            ),
            13 => (
                |input: &str| Solution::from(y2024::day13::part_1(input)),
                |input: &str| Solution::from(y2024::day13::part_2(input)),
            ),
            14 => (
                |input: &str| Solution::from(y2024::day14::part_1(input)),
                |input: &str| Solution::from(y2024::day14::part_2(input)),
            ),
            15 => (
                |input: &str| Solution::from(y2024::day15::part_1(input)),
                |input: &str| Solution::from(y2024::day15::part_2(input)),
            ),
            16 => (
                |input: &str| Solution::from(y2024::day16::part_1(input)),
                |input: &str| Solution::from(y2024::day16::part_2(input)),
            ),
            17 => (
                |input: &str| Solution::from(y2024::day17::part_1(input)),
                |input: &str| Solution::from(y2024::day17::part_2(input)),
            ),
            18 => (
                |input: &str| Solution::from(y2024::day18::part_1(input)),
                |input: &str| Solution::from(y2024::day18::part_2(input)),
            ),
            19 => (
                |input: &str| Solution::from(y2024::day19::part_1(input)),
                |input: &str| Solution::from(y2024::day19::part_2(input)),
            ),
            20 => (
                |input: &str| Solution::from(y2024::day20::part_1(input)),
                |input: &str| Solution::from(y2024::day20::part_2(input)),
            ),
            21 => (
                |input: &str| Solution::from(y2024::day21::part_1(input)),
                |input: &str| Solution::from(y2024::day21::part_2(input)),
            ),
            22 => (
                |input: &str| Solution::from(y2024::day22::part_1(input)),
                |input: &str| Solution::from(y2024::day22::part_2(input)),
            ),
            23 => (
                |input: &str| Solution::from(y2024::day23::part_1(input)),
                |input: &str| Solution::from(y2024::day23::part_2(input)),
            ),
            24 => (
                |input: &str| Solution::from(y2024::day24::part_1(input)),
                |input: &str| Solution::from(y2024::day24::part_2(input)),
            ),
            25 => (
                |input: &str| Solution::from(y2024::day25::part_1(input)),
                |input: &str| Solution::from(y2024::day25::part_2(input)),
            ),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
