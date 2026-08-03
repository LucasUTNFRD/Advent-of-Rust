pub type Input = Vec<Instruction>;

pub enum Instruction {
    Do(u32, u32),
    Dont(u32, u32),
}

pub fn part_1(data: &Input) -> u32 {
    data.iter()
        .map(|i| match &i {
            Instruction::Dont(a, b) | Instruction::Do(a, b) => a * b,
        })
        .sum()
}

pub fn parse(data: &str) -> Input {
    let bytes = data.as_bytes();
    let n = bytes.len();
    let mut results = Vec::new();
    let mut i = 0;
    let mut enabled = true;

    while i < n {
        if bytes[i..].starts_with(b"do()") {
            i += 4;
            enabled = true;
        }
        if bytes[i..].starts_with(b"don't()") {
            i += 7;
            enabled = false;
        }

        // look for literal "mul("
        if i + 4 <= n && bytes[i..].starts_with(b"mul(") {
            let mut j = i + 4;

            // parse first number (1-3 digits)
            let start1 = j;
            while j < n && bytes[j].is_ascii_digit() && j - start1 < 3 {
                j += 1;
            }
            if j == start1 {
                i += 1;
                continue;
            }
            let num1_end = j;

            // expect comma
            if j < n && bytes[j] == b',' {
                j += 1;
            } else {
                i += 1;
                continue;
            }

            // parse second number (1-3 digits)
            let start2 = j;
            while j < n && bytes[j].is_ascii_digit() && j - start2 < 3 {
                j += 1;
            }
            if j == start2 {
                i += 1;
                continue;
            }
            let num2_end = j;

            // expect close paren
            if j < n && bytes[j] == b')' {
                let a: u32 = unsafe { data[start1..num1_end].parse().unwrap_unchecked() };
                let b: u32 = unsafe { data[start2..num2_end].parse().unwrap_unchecked() };
                let instruction = if enabled {
                    Instruction::Do(a, b)
                } else {
                    Instruction::Dont(a, b)
                };
                results.push(instruction);
                i = j + 1;
                continue;
            } else {
                i += 1;
                continue;
            }
        }
        i += 1;
    }

    results
}

pub fn part_2(data: &Input) -> u32 {
    data.iter()
        .filter_map(|i| match i {
            Instruction::Do(a, b) => Some(a * b),
            Instruction::Dont(_, _) => None,
        })
        .sum()
}
