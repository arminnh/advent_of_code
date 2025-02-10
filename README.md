# Advent of Code in Rust

[![Tests & Static Checks](https://github.com/arminnh/advent_of_code/actions/workflows/ci.yml/badge.svg)](https://github.com/arminnh/advent_of_code/actions/workflows/ci.yml)

Practicing Rust with [Advent Of Code](https://www.adventofcode.com) using as few dependencies as possible to solve puzzles.

Each day has functions `part_1` and `part_2` which take a `&str` and return a primitive type.

## Usage

```sh
$ cargo run -- --help
Solve Advent of Code puzzles with optional year and day filtering.

Usage: advent_of_code [YEAR] [DAYS]...

Arguments:
  [YEAR]     Year to solve (default: all years [2022-2024])
  [DAYS]...  Specific days to solve (default: all days [1-25])
```
