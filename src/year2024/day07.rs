// Day 07

use itertools::Itertools;
use std::ops::Add;

pub type Input = Vec<Calibration>;

pub struct Calibration {
    result: u64,
    test: Vec<u64>,
}

impl Calibration {
    fn pass_test(&self) -> bool {
        let num_operators = self.test.len() - 1;

        (0..2_u64.pow(num_operators as u32))
            // .into_par_iter()
            .any(|mask| {
                let mut acc = self.test[0];
                for i in 0..num_operators {
                    let op = (mask >> i) & 1;
                    if op == 0 {
                        acc += self.test[i + 1];
                    } else {
                        acc *= self.test[i + 1];
                    }
                }
                acc == self.result
            })

        // self.test.iter().reduce() == self.test
    }

    fn pass_test_part_2(&self) -> bool {
        let num_operators = self.test.len() - 1;

        let ops = vec![BinOp::Concat, BinOp::Add, BinOp::Mull];
        std::iter::repeat_n(ops, num_operators)
            .multi_cartesian_product()
            .into_iter()
            .any(|ops| {
                let mut acc = self.test[0];
                ops.iter().enumerate().for_each(|(i, &op)| {
                    let op = get_op(op);
                    acc = op(acc, self.test[i + 1]);
                });
                acc == self.result
            })
    }
}
#[derive(Clone, Copy, Debug)]
enum BinOp {
    Concat,
    Add,
    Mull,
}

fn get_op(op: BinOp) -> fn(u64, u64) -> u64 {
    match op {
        BinOp::Concat => |a, b| {
            let mut mult = 1;
            let mut temp = b;
            while temp > 0 {
                mult *= 10;
                temp /= 10;
            }
            if mult == 1 { a * 10 + b } else { a * mult + b }
        },
        BinOp::Add => |a, b| a + b,
        BinOp::Mull => |a, b| a * b,
    }
}

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let mut items_iter = l.split(':');
            let result = items_iter.next().unwrap().parse::<u64>().unwrap();
            let test: Vec<u64> = items_iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect();

            Calibration { result, test }
        })
        .collect()
}

pub fn part_1(input: &[Calibration]) -> u64 {
    input
        .iter()
        .filter(|calibration| calibration.pass_test())
        .map(|calibration| calibration.result)
        .sum()
}

pub fn part_2(input: &Input) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE);
        assert_eq!(part_1(&input), 3749);
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE);
        assert_eq!(part_2(&input), 0);
    }
}
