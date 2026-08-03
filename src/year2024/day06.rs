use crate::util::{
    grid::Grid,
    point::{EAST, NORTH, Point, SOUTH, WEST},
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustc_hash::{FxBuildHasher, FxHashSet};

type Input = Grid<u8>;

pub fn parse(input: &str) -> Input {
    Grid::parse(input)
}

const OBSTACLE: u8 = b'#';

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_90_degree(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn part_1(input: &Input) -> usize {
    let mut curr_pos = input.find(b'^').expect("start position is guaranteed");
    let mut curr_dir = Direction::Up;
    // let mut visited = FxHashSet::<Point>::with_capacity_and_hasher(5000, FxBuildHasher);

    let mut visited = input.copy_maze_with(0);
    let mut count = 1;

    visited[curr_pos] = 1;

    loop {
        let dir = match curr_dir {
            Direction::Up => NORTH,
            Direction::Down => SOUTH,
            Direction::Left => WEST,
            Direction::Right => EAST,
        };
        let next_pos = curr_pos + dir;

        if let Some(&OBSTACLE) = input.get(next_pos) {
            curr_dir = curr_dir.rotate_90_degree();
        } else if let Some(&b'.') | Some(&b'^') = input.get(next_pos) {
            curr_pos = next_pos;
        } else {
            return count;
        }

        if visited[curr_pos] == 0 {
            visited[curr_pos] = 1;
            count += 1
        }
    }
}

fn is_loop(
    maze: &Grid<u8>,
    obstacle_pos: Point,
    start_pos: Point,
    visited: &mut FxHashSet<(Point, Direction)>,
) -> bool {
    visited.clear();

    let mut loop_flag = false;
    let mut curr_pos = start_pos;
    let mut curr_dir = Direction::Up;

    loop {
        if !visited.insert((curr_pos, curr_dir)) {
            loop_flag = true;
            break;
        }

        let dir = match curr_dir {
            Direction::Up => NORTH,
            Direction::Down => SOUTH,
            Direction::Left => WEST,
            Direction::Right => EAST,
        };

        let next_pos = curr_pos + dir;

        if next_pos == obstacle_pos {
            curr_dir = curr_dir.rotate_90_degree();
            continue;
        }
        match maze.get(next_pos) {
            Some(&OBSTACLE) => curr_dir = curr_dir.rotate_90_degree(),
            Some(_) => curr_pos = next_pos,
            None => break,
        }
    }

    loop_flag
}

pub fn part_2(input: &Input) -> usize {
    let start_pos = input.find(b'^').expect("start position is guaranteed");

    let visited = {
        let mut curr_pos = start_pos;
        let mut curr_dir = Direction::Up;
        let mut visited = FxHashSet::<Point>::with_capacity_and_hasher(5350, FxBuildHasher);

        visited.insert(curr_pos);

        loop {
            let dir = match curr_dir {
                Direction::Up => NORTH,
                Direction::Down => SOUTH,
                Direction::Left => WEST,
                Direction::Right => EAST,
            };
            let next_pos = curr_pos + dir;

            if let Some(&OBSTACLE) = input.get(next_pos) {
                curr_dir = curr_dir.rotate_90_degree();
            } else if let Some(&b'.') | Some(&b'^') = input.get(next_pos) {
                curr_pos = next_pos;
            } else {
                break;
            }

            visited.insert(curr_pos);
        }
        visited
    };

    visited
        .par_iter()
        .map_init(
            || FxHashSet::<(Point, Direction)>::with_capacity_and_hasher(5500, FxBuildHasher),
            |set, &obstacle_candidate| is_loop(input, obstacle_candidate, start_pos, set),
        )
        .filter(|&is_loop| is_loop)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE);
        assert_eq!(part_1(&input), 41);
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE);
        assert_eq!(part_2(&input), 6);
    }
}
