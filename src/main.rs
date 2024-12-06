mod util;
mod y2022;
mod y2023;
mod y2024;

use std::env;
use std::time::Instant;
use util::solution::Solution;

pub type SolutionPair = (Solution, Solution);

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

    for year in years {
        println!("\n\n\n====== Year {} ======", year);
        for day in &days {
            let func = get_day_solver(year, day);

            let time = Instant::now();
            let (p1, p2) = func();
            let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;

            println!("\n=== Day {:02} ===", day);
            println!("  · Part 1: {}", p1);
            println!("  · Part 2: {}", p2);
            println!("  · Elapsed: {:.4} ms", elapsed_ms);

            runtime += elapsed_ms;
        }
    }

    println!("Total runtime: {:.4} ms", runtime);
}

fn get_day_solver(year: usize, day: &u8) -> fn() -> SolutionPair {
    match year {
        2022 => match day {
            1 => y2022::day01::solve,
            2 => y2022::day02::solve,
            3 => y2022::day03::solve,
            4 => y2022::day04::solve,
            5 => y2022::day05::solve,
            6 => y2022::day06::solve,
            7 => y2022::day07::solve,
            8 => y2022::day08::solve,
            9 => y2022::day09::solve,
            10 => y2022::day10::solve,
            11 => y2022::day11::solve,
            12 => y2022::day12::solve,
            _ => unimplemented!(),
        },
        2023 => match day {
            1 => y2023::day01::solve,
            2 => y2023::day02::solve,
            3 => y2023::day03::solve,
            4 => y2023::day04::solve,
            5 => y2023::day05::solve,
            6 => y2023::day06::solve,
            7 => y2023::day07::solve,
            8 => y2023::day08::solve,
            9 => y2023::day09::solve,
            10 => y2023::day10::solve,
            11 => y2023::day11::solve,
            12 => y2023::day12::solve,
            13 => y2023::day13::solve,
            14 => y2023::day14::solve,
            15 => y2023::day15::solve,
            16 => y2023::day16::solve,
            17 => y2023::day17::solve,
            18 => y2023::day18::solve,
            19 => y2023::day19::solve,
            20 => y2023::day20::solve,
            21 => y2023::day21::solve,
            22 => y2023::day22::solve,
            23 => y2023::day23::solve,
            24 => y2023::day24::solve,
            25 => y2023::day25::solve,
            _ => unimplemented!(),
        },
        2024 => match day {
            1 => y2024::day01::solve,
            2 => y2024::day02::solve,
            3 => y2024::day03::solve,
            4 => y2024::day04::solve,
            5 => y2024::day05::solve,
            6 => y2024::day06::solve,
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
