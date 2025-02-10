mod util;
mod y2022;
mod y2023;
mod y2024;

use clap::{command, Parser};
use std::time::Instant;
use util::solution::{Solution, SolverFn};
use util::util::load_input;

#[derive(Parser, Debug)]
#[command(
    name = "Advent of Code Solver",
    about = "Solve Advent of Code puzzles with optional year and day filtering."
)]
struct Cli {
    /// Year to solve (default: all years [2022-2024])
    // #[arg(short, long)]
    year: Option<usize>,

    /// Specific days to solve (default: all days [1-25])
    // #[arg(short, long)]
    #[arg(num_args=1..=25)]
    days: Option<Vec<u8>>,
}

fn main() {
    let cli = Cli::parse();

    let years = cli
        .year
        .map_or_else(|| (2022..=2024).collect(), |y| vec![y]);
    let days: Vec<u8> = cli.days.unwrap_or((1..=25).collect());

    solve_with_time_tracking(years, days);
}

fn solve_with_time_tracking(years: Vec<usize>, days: Vec<u8>) {
    let mut runtime = 0.0;
    let mut times: Vec<(f64, usize, u8, usize)> = Vec::new();

    for year in years {
        println!("====== Year {} ======", year);
        for day in &days {
            println!("=== Day {:02} ===", day);
            let (part_1, part_2) = get_day_solvers(year, day);
            let input = load_input(&format!("inputs/{}/day_{}", year, day));

            let mut do_part = |solver: fn(&str) -> Solution, part_nr| {
                let time = Instant::now();
                let result = solver(&input);
                let elapsed_ms = time.elapsed().as_nanos() as f64 / 1_000_000.0;
                println!("  · Part {} ({:>9.4} ms): {}", part_nr, elapsed_ms, result);
                runtime += elapsed_ms;
                times.push((elapsed_ms, year, *day, part_nr));
            };
            do_part(part_1, 1);
            do_part(part_2, 2);
            println!();
        }
    }

    println!("Total runtime ({} parts): {:.4} ms", times.len(), runtime);
    if times.len() > 10 {
        times.sort_by(|a, b| b.0.total_cmp(&a.0));
        println!("Slowest 5: {:?}", &times[..5]);
    }
}

fn get_day_solvers(year: usize, day: &u8) -> (SolverFn, SolverFn) {
    match year {
        2022 => match day {
            1 => make_solver!(y2022, day01),
            2 => make_solver!(y2022, day02),
            3 => make_solver!(y2022, day03),
            4 => make_solver!(y2022, day04),
            5 => make_solver!(y2022, day05),
            6 => make_solver!(y2022, day06),
            7 => make_solver!(y2022, day07),
            8 => make_solver!(y2022, day08),
            9 => make_solver!(y2022, day09),
            10 => make_solver!(y2022, day10),
            11 => make_solver!(y2022, day11),
            12 => make_solver!(y2022, day12),
            _ => unimplemented!(),
        },
        2023 => match day {
            1 => make_solver!(y2023, day01),
            2 => make_solver!(y2023, day02),
            3 => make_solver!(y2023, day03),
            4 => make_solver!(y2023, day04),
            5 => make_solver!(y2023, day05),
            6 => make_solver!(y2023, day06),
            7 => make_solver!(y2023, day07),
            8 => make_solver!(y2023, day08),
            9 => make_solver!(y2023, day09),
            10 => make_solver!(y2023, day10),
            11 => make_solver!(y2023, day11),
            12 => make_solver!(y2023, day12),
            13 => make_solver!(y2023, day13),
            14 => make_solver!(y2023, day14),
            15 => make_solver!(y2023, day15),
            16 => make_solver!(y2023, day16),
            17 => make_solver!(y2023, day17),
            18 => make_solver!(y2023, day18),
            19 => make_solver!(y2023, day19),
            20 => make_solver!(y2023, day20),
            21 => make_solver!(y2023, day21),
            22 => make_solver!(y2023, day22),
            23 => make_solver!(y2023, day23),
            24 => make_solver!(y2023, day24),
            25 => make_solver!(y2023, day25),
            _ => unimplemented!(),
        },
        2024 => match day {
            1 => make_solver!(y2024, day01),
            2 => make_solver!(y2024, day02),
            3 => make_solver!(y2024, day03),
            4 => make_solver!(y2024, day04),
            5 => make_solver!(y2024, day05),
            6 => make_solver!(y2024, day06),
            7 => make_solver!(y2024, day07),
            8 => make_solver!(y2024, day08),
            9 => make_solver!(y2024, day09),
            10 => make_solver!(y2024, day10),
            11 => make_solver!(y2024, day11),
            12 => make_solver!(y2024, day12),
            13 => make_solver!(y2024, day13),
            14 => make_solver!(y2024, day14),
            15 => make_solver!(y2024, day15),
            16 => make_solver!(y2024, day16),
            17 => make_solver!(y2024, day17),
            18 => make_solver!(y2024, day18),
            19 => make_solver!(y2024, day19),
            20 => make_solver!(y2024, day20),
            21 => make_solver!(y2024, day21),
            22 => make_solver!(y2024, day22),
            23 => make_solver!(y2024, day23),
            24 => make_solver!(y2024, day24),
            25 => make_solver!(y2024, day25),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
