use std::{fs::read_to_string, time::Instant};

use aoc_rs::year2024;
use z3::DatatypeAccessor;

fn main() {
    let data = read_to_string("inputs/year2024/day1.txt").unwrap();

    let data = year2024::day1::parse(&data);

    let start = Instant::now();
    let solution_part_1 = year2024::day1::part_1(&data);
    let duration_part_1 = start.elapsed();
    println!(
        "Day 10 - Part 1 solution {} - elapsed {:#?}",
        solution_part_1, duration_part_1
    );

    let start = Instant::now();
    let solution_part_2 = year2024::day1::part_2(&data);
    let duration_part_2 = start.elapsed();
    println!(
        "Day 10 - Part 2 solution {} - elapsed {:#?}",
        solution_part_2, duration_part_2
    );
}
