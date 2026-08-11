// Day 10

use crate::util::{
    grid::Grid,
    point::{EAST, NORTH, Point, SOUTH, WEST},
};
use rustc_hash::FxHashSet;

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

fn get_trail_score(grid: &Input, start: Point) -> usize {
    let mut stack = Vec::with_capacity(grid.data.len());
    stack.push(start);

    let mut visited = grid.copy_maze_with(false as u8);
    visited[start] = true as u8;

    let mut trailhead_visited = FxHashSet::default();

    while let Some(point) = stack.pop() {
        let curr_height = grid[point];

        if curr_height == b'9' {
            trailhead_visited.insert(point);
            continue;
        }

        for dir in [EAST, NORTH, SOUTH, WEST] {
            let next_point = point + dir;
            if let Some(&next_height) = grid.get(next_point)
                && next_height == curr_height + 1
                && visited[next_point] == false as u8
            {
                stack.push(next_point);
            }
        }
    }

    trailhead_visited.len()
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
        assert_eq!(part_1(&input), 36);
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE);
        assert_eq!(part_2(&input), 0);
    }
}
