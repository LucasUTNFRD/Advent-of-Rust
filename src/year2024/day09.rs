// Day 09

use std::{cmp::Reverse, collections::BinaryHeap};

type Input = Vec<(u8, u8)>;

pub fn parse(input: &str) -> Input {
    let n = input.len();
    let mut i = 0;
    let mut disk_map = Vec::with_capacity(n / 2);
    let bytes = input.as_bytes();
    while i < n {
        let files = bytes[i] - b'0';
        let free_space = bytes
            .get(i + 1)
            .copied()
            .map(|b| b - b'0')
            .unwrap_or_default();

        disk_map.push((files, free_space));
        i += 2
    }

    disk_map
}

pub fn part_1(input: &Input) -> usize {
    let mut disk_map = input.clone();
    let (mut start, mut end) = (0usize, disk_map.len() - 1);

    let mut pos = 0;
    let mut checksum = 0;
    while start < end {
        let (start_blocks, start_free) = disk_map[start];
        for _ in 0..start_blocks {
            checksum += pos * start;
            pos += 1;
        }

        let mut free_space = start_free;
        while free_space > 0 && start < end {
            let (end_blocks, _) = disk_map[end];
            let can_move = u8::min(free_space, end_blocks);

            for _ in 0..can_move {
                checksum += pos * end;
                pos += 1;
            }

            disk_map[end].0 -= can_move;
            free_space -= can_move;

            if disk_map[end].0 == 0 {
                end -= 1;
            }
        }

        start += 1
    }

    if start == end {
        let (last_file, _) = disk_map[start];
        for _ in 0..last_file {
            checksum += pos * start;
            pos += 1;
        }
    }

    checksum
}

pub fn part_2(input: &Input) -> usize {
    struct File {
        file_id: usize,
        pos: usize,
        size: usize,
    }

    let mut file = Vec::with_capacity(input.len());
    let mut free_heaps: [BinaryHeap<Reverse<usize>>; 10] = Default::default();
    let mut curr_pos = 0;
    for (id, &(block_size, free_size)) in input.iter().enumerate() {
        file.push(File {
            file_id: id,
            pos: curr_pos,
            size: block_size as usize,
        });

        curr_pos += block_size as usize;

        if free_size > 0 && free_size < 10 {
            free_heaps[free_size as usize].push(Reverse(curr_pos));
            curr_pos += free_size as usize;
        }
    }

    file.iter_mut().rev().for_each(|f| {
        let target = (f.size..=9)
            .filter_map(|size| free_heaps[size].peek().map(|&Reverse(pos)| (size, pos)))
            .filter(|&(_, pos)| pos < f.pos)
            .min_by_key(|&(_, pos)| pos);

        if let Some((best_size, best_pos)) = target {
            free_heaps[best_size].pop();
            f.pos = best_pos;

            let remaining_size = best_size - f.size;
            if remaining_size > 0 {
                free_heaps[remaining_size].push(Reverse(best_pos + f.size));
            }
        }
    });

    file.iter().fold(0usize, |checksum, f| {
        let s = f.size;
        let p = f.pos;
        let i = f.file_id;
        checksum + i * (s * p + (s * (s - 1)) / 2)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE);

        assert_eq!(part_1(&input), 1928);
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE);
        assert_eq!(part_2(&input), 2858);
    }
}
