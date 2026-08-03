use crate::util::{
    grid::Grid,
    point::{DIRECTIONS, Point},
};

pub type Input = Grid<u8>;

pub fn parse(data: &str) -> Input {
    Grid::parse(data)
}

pub fn part_1(data: &Input) -> usize {
    data.find_all_iter(b'X')
        .map(|start_pos| count_xmas_in_every_pos(data, start_pos))
        .sum()
}

fn count_xmas_in_every_pos(data: &Input, start_pos: Point) -> usize {
    let mut words_found = 0;
    for dir in DIRECTIONS {
        if data.get(start_pos + dir) == Some(&b'M')
            && data.get(start_pos + dir * 2) == Some(&b'A')
            && data.get(start_pos + dir * 3) == Some(&b'S')
        {
            words_found += 1;
        }
    }

    words_found
}

pub fn part_2(data: &Input) -> usize {
    data.find_all_iter(b'A')
        .filter(|&start_pos| {
            let nw = data.get(start_pos + Point::new(-1, -1));
            let se = data.get(start_pos + Point::new(1, 1));
            let ne = data.get(start_pos + Point::new(1, -1));
            let sw = data.get(start_pos + Point::new(-1, 1));

            // Check if Diagonal 1 is valid ("MAS" or "SAM")
            let diag1 = (nw == Some(&b'M') && se == Some(&b'S'))
                || (nw == Some(&b'S') && se == Some(&b'M'));
            // Check if Diagonal 1 is valid ("MAS" or "SAM")
            let diag2 = (ne == Some(&b'M') && sw == Some(&b'S'))
                || (ne == Some(&b'S') && sw == Some(&b'M'));

            diag1 && diag2
        })
        .count()
}

#[cfg(test)]
mod test {
    const SAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    use super::*;
    #[test]
    fn test_part_1() {
        let input = parse(SAMPLE_INPUT);
        assert_eq!(18, part_1(&input));
    }

    #[test]
    fn test_part_2() {
        let input = parse(SAMPLE_INPUT);
        assert_eq!(9, part_2(&input));
    }
}
