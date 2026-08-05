// Day 08

use crate::util::{grid::Grid, point::Point};
use itertools::Itertools;
use rustc_hash::FxHashMap;

type Input = (usize, usize);

pub fn parse(input: &str) -> Input {
    let grid = Grid::parse(input);

    let antennas: FxHashMap<u8, Vec<Point>> = grid
        .data
        .iter()
        .enumerate()
        .filter(|&(_, e)| *e != b'.')
        .fold(FxHashMap::default(), |mut acc, (idx, e)| {
            let x = (idx as i32) % grid.width;
            let y = (idx as i32) / grid.width;
            acc.entry(*e).or_default().push(Point::new(x, y));
            acc
        });

    let mut antinodes_1 = grid.copy_maze_with(0);
    let mut antinodes_2 = grid.copy_maze_with(0);
    antennas.values().for_each(|points| {
        points.iter().tuple_combinations().for_each(|(&p1, &p2)| {
            let delta = Point::new(p2.x - p1.x, p2.y - p1.y);

            let antinode1 = p1 - delta;
            if let Some(b) = antinodes_1.get_mut(antinode1) {
                *b = 1;
            }

            let mut current1 = p1;
            while let Some(b) = antinodes_2.get_mut(current1) {
                *b = 1;
                current1 -= delta;
            }

            let antinode2 = p2 + delta;
            if let Some(b) = antinodes_1.get_mut(antinode2) {
                *b = 1;
            }

            let mut current2 = p2;
            while let Some(b) = antinodes_2.get_mut(current1) {
                *b = 1;
                current2 += delta;
            }
        });
    });

    let part_1 = antinodes_1.data.iter().filter(|b| **b == 1).count();
    let part_2 = antinodes_2.data.iter().filter(|b| **b == 1).count();
    (part_1, part_2)
}

pub fn part_1(grid: &Input) -> usize {
    grid.0
}

pub fn part_2(grid: &Input) -> usize {
    grid.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE);
        assert_eq!(part_1(&input), 14);
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE);
        assert_eq!(part_2(&input), 34);
    }
}
