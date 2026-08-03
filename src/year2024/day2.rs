pub type Input = Vec<Vec<u32>>;

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_1(input: &Input) -> u32 {
    input.iter().filter(|&levels| is_safe(levels)).count() as u32
}

fn is_safe(levels: &[u32]) -> bool {
    let first_diff = levels[1] as i32 - levels[0] as i32;
    if first_diff == 0 || first_diff.abs() > 3 {
        return false;
    }

    let increasing = first_diff > 0;

    for i in 0..levels.iter().len() - 1 {
        let diff = levels[i + 1] as i32 - levels[i] as i32;
        let right_dir = if increasing { diff > 0 } else { diff < 0 };
        if !right_dir || diff.abs() > 3 {
            return false;
        }
    }
    true
}

fn is_safe_with_dampener(levels: &[u32]) -> bool {
    if is_safe(levels) {
        return true;
    }

    // para cada indice en levels, check si al menos removiendo uno pasa a dar is_safe positivo
    (0..levels.len()).any(|i| {
        let mut v = levels.to_vec();
        v.remove(i);
        is_safe(&v)
    })
}

pub fn part_2(input: &Input) -> u32 {
    input
        .iter()
        .filter(|&levels| is_safe_with_dampener(levels))
        .count() as u32
}
