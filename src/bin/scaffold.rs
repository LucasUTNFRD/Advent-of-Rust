use std::{env, fs, path::Path, process};

const TEMPLATE: &str = r#"
// Day {DAY}

type Input = ();

pub fn parse(input: &str) -> Input {
    todo!()
}

pub fn part_1(input: &Input) -> usize {
    todo!()
}

pub fn part_2(input: &Input) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_part_1() {
        let input = parse(EXAMPLE);
        assert_eq!(part_1(&input), 0);
    }

    #[test]
    fn test_part_2() {
        let input = parse(EXAMPLE);
        assert_eq!(part_2(&input), 0);
    }
}
"#;

fn main() {
    let [year, day]: [String; 2] = env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .try_into()
        .expect("usage: scaffold <year> <day>");

    let year: u16 = year.parse().expect("year must be a number");
    let day: u8 = day.parse().expect("day must be a number");

    assert!((1..=25).contains(&day), "day must be between 1 and 25");

    let day_str = format!("{day:02}");

    let src_dir = format!("src/year{year}");
    fs::create_dir_all(&src_dir).expect("failed to create source directory");

    let source_file = format!("{src_dir}/day{day_str}.rs");

    if Path::new(&source_file).exists() {
        eprintln!("{source_file} already exists");
        process::exit(1);
    }

    let source = TEMPLATE.replace("{DAY}", &day_str);

    fs::write(&source_file, source).expect("failed to write source file");

    let mod_file = format!("{src_dir}/mod.rs");

    let mut modules = fs::read_to_string(&mod_file).unwrap_or_default();

    let module_line = format!("pub mod day{day_str};");

    if !modules.lines().any(|l| l == module_line) {
        modules.push_str(&module_line);
        modules.push('\n');

        let mut lines: Vec<_> = modules.lines().collect();
        lines.sort_unstable();

        let modules = lines.join("\n") + "\n";

        fs::write(&mod_file, modules).expect("failed to update mod.rs");
    }

    let input_dir = format!("inputs/year{year}");
    fs::create_dir_all(&input_dir).expect("failed to create input directory");

    let input_file = format!("{input_dir}/day{day_str}.txt");

    if !Path::new(&input_file).exists() {
        fs::write(&input_file, "").expect("failed to create input file");
    }

    println!("Created:");
    println!("  {source_file}");
    println!("  {mod_file}");
    println!("  {input_file}");
}
