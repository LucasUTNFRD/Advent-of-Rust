use rustc_hash::FxHashMap;

pub type Input = (Vec<u32>, Vec<u32>);

fn parse(input: &str) -> Input {
    let mut left = Vec::with_capacity(input.lines().count());
    let mut right = Vec::with_capacity(left.capacity());

    for l in input.lines() {
        let mut it = l.split_whitespace();
        let l_str = unsafe { it.next().unwrap_unchecked() };
        let r_str = unsafe { it.next().unwrap_unchecked() };
        left.push(unsafe { l_str.parse::<u32>().unwrap_unchecked() });
        right.push(unsafe { r_str.parse::<u32>().unwrap_unchecked() });
    }
    (left, right)
}

pub fn part_1(input: &Input) -> u32 {
    let (mut left_list, mut right_list) = input.clone();
    left_list.sort_unstable();
    right_list.sort_unstable();
    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| u32::abs_diff(*l, *r))
        .sum()
}
pub fn part_2(input: &Input) -> u32 {
    let (left, right) = input;
    let freq = {
        let mut freq = FxHashMap::default();
        right.iter().for_each(|r| *freq.entry(r).or_insert(0) += 1);
        freq
    };

    left.iter()
        .map(|l| {
            let f = freq.get(l).copied().unwrap_or(0);
            l * f
        })
        .sum()
}
