use std::{fs::read_to_string, time::Instant};

use aoc_rs::year2024;

fn main() {
    let data = read_to_string("inputs/year2024/day09.txt").unwrap();

    let data = year2024::day09::parse(&data);

    let start = Instant::now();
    let solution_part_1 = year2024::day09::part_1(&data);
    let duration = start.elapsed();

    println!(
        "Day 2 - Part 1 solution {}    - elapsed {:#?}",
        solution_part_1, duration
    );
}
