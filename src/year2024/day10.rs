// Day 10

use crate::util::{
    grid::Grid,
    point::{EAST, NORTH, Point, SOUTH, WEST},
};

type Input = Grid<u8>;

pub fn parse(input: &str) -> Input {
    Grid::parse(input)
}

pub fn part_1(input: &Input) -> usize {
    let trails = input.find_all(b'0');
    trails
        .iter()
        .map(|trail| get_trail_score(input, *trail))
        .sum()
}

fn get_trail_score(grid: &Input, curr_point: Point) -> usize {
    let curr = grid[curr_point];

    if curr == b'9' {
        return 1;
    }

    let mut total_paths = 0;

    for dir in [NORTH, SOUTH, WEST, EAST] {
        let new_dir = dir + curr_point;

        if let Some(val) = grid.get(new_dir)
            && *val == curr + 1
        {
            total_paths += get_trail_score(grid, new_dir)
        }
    }

    dbg!(total_paths);
    total_paths
}

pub fn part_2(input: &Input) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE);
        assert_eq!(part_1(&input), 5);
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE);
        assert_eq!(part_2(&input), 0);
    }
}
