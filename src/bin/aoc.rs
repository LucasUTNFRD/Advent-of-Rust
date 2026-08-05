use std::{fs::read_to_string, time::Instant};

use aoc_rs::year2024;

fn main() {
    let data = read_to_string("inputs/year2024/day08.txt").unwrap();

    let start = Instant::now();
    let data = year2024::day08::parse(&data);
    let (solution_part_1, solution_part_2) = data;
    let duration = start.elapsed();

    println!(
        "Day 2 - Part 1 solution {}   - Part 2 Solution {} - elapsed {:#?}",
        solution_part_1, solution_part_2, duration
    );
}
