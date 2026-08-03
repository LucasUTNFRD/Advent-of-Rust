use std::{fs::read_to_string, time::Instant};

use aoc_rs::year2024;

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(std::thread::available_parallelism().unwrap().get())
        .build_global()
        .unwrap();

    let data = read_to_string("inputs/year2024/day06.txt").unwrap();

    let data = year2024::day06::parse(&data);

    let start = Instant::now();
    let solution_part_1 = year2024::day06::part_1(&data);
    let duration = start.elapsed();

    println!(
        "Day 2 - Part 1 solution {}  elapsed {:#?}",
        solution_part_1, duration
    );

    let start = Instant::now();
    let solution_part_2 = year2024::day06::part_2(&data);
    let duration = start.elapsed();

    println!(
        "Day 2 - Part 2 solution {}  elapsed {:#?}",
        solution_part_2, duration
    );
}
