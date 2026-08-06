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
    let mut disk_map = input.clone();
    let n = disk_map.len();

    let mut pos = 0;

    let mut free_list: Vec<BinaryHeap<Reverse<usize>>> = (1..=9)
        .map(|free_size| {
            let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
            for (i, (_, free)) in disk_map.iter().enumerate() {
                if *free == free_size {
                    heap.push(Reverse(i));
                }
            }
            heap
        })
        .collect();

    let (start, end) = (0usize, disk_map.len() - 1);

    for idx in (0..n).rev() {
        let (file_len, _) = disk_map[idx];
        // where i can find enough blocks left most
        if file_len == 0 {
            continue;
        }

        let Some((size, span_pos)) = (file_len..=9)
            .filter_map(|size| free_list[size as usize].peek().map(|&Reverse(p)| (size, p)))
            .min_by_key(|&(_, p)| p)
        else {
            continue;
        };

        free_list[size as usize - 1].pop();
        let left_over = size - file_len;
        if left_over > 0 {
            free_list[left_over as usize - 1].push(Reverse(span_pos + file_len as usize));
        }
    }

    todo!()
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
        assert_eq!(part_2(&input), 0);
    }
}
