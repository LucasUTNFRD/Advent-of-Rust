// Day 05

use rustc_hash::FxHashMap;
use std::{collections::HashMap, ops::Div};

type Input = (usize, usize);

// type MapType = HashMap<u8, Vec<u8>>;
type MapType = FxHashMap<u8, Vec<u8>>;

pub fn parse(input: &str) -> Input {
    let mut parts = input.split("\n\n");
    let rules_str = parts.next().unwrap();
    let updates_str = parts.next().unwrap();

    let mut rules: MapType = HashMap::default();
    // FxHashMap<u8, Vec<u8>> = FxHashMap::default();

    rules_str.lines().for_each(|l| {
        let (l, r): (u8, u8) = {
            let mut nums = l.split('|').map(|n| n.parse().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        };

        rules.entry(l).or_default().push(r);
    });

    let updates = updates_str.lines().map(|l| {
        l.split(',')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u8>>()
    });

    let (mut part1, mut part2) = (0, 0);
    //
    // let (rules, updates) = input;
    //
    updates.for_each(|update| {
        if is_right_order(&update, &rules) {
            part1 += update[update.iter().len().div(2)] as usize
        } else {
            let fixed = fix_update(&update, &rules);
            part2 += fixed[fixed.iter().len().div(2)] as usize;
        }
    });
    (part1, part2)
}

pub fn part_1(input: &Input) -> usize {
    input.0
}

fn is_right_order(update: &[u8], page_ordering_map: &MapType) -> bool {
    for (&page, to_precede) in page_ordering_map {
        let index = match update.iter().position(|&x| x == page) {
            Some(i) => i,
            None => continue,
        };

        for &target in to_precede {
            if let Some(target_index) = update.iter().position(|&x| x == target)
                && target_index < index
            {
                return false;
            }
        }
    }

    true
}

fn fix_update(update: &[u8], rules: &MapType) -> Vec<u8> {
    let mut fixed = update.to_vec();
    fixed.sort_by(|a, b| {
        if rules.get(a).is_some_and(|succs| succs.contains(b)) {
            std::cmp::Ordering::Less
        } else if rules.get(b).is_some_and(|succs| succs.contains(a)) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    fixed
}

pub fn part_2(input: &Input) -> usize {
    input.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE);
        assert_eq!(part_1(&input), 143);
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE);
        assert_eq!(part_2(&input), 123);
    }
}
