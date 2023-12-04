use anyhow::Context;
use std::process::Command;

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env");
    let mut args = std::env::args();
    _ = args.next();
    let day_id = args.next().unwrap();
    let day_str = format!("day{:0>2}", day_id);
    let test = args.next().is_some_and(|x| x == "test");

    let mut base_dir = std::path::PathBuf::new();
    base_dir.push(".");

    let inputs_dir = base_dir.join("inputs");
    if !inputs_dir.exists() {
        std::fs::create_dir_all(&inputs_dir).context("creating input dir")?;
    }

    let input_path = inputs_dir.join(format!("{day_id:0>2}.txt"));
    if !input_path.exists() {
        println!("Downloading inputs...");
        let session_token = std::env::var("COOKIE_SESSION")?;
        let input = aoc_helper::download_input(&session_token, 2023, day_id.parse().unwrap())?;
        std::fs::write(&input_path, input).context("writing input file")?;
        println!("Input downloaded to {}", input_path.as_path().display());
    }

    let file_path = base_dir.join(format!("src/bin/{day_str}.rs"));
    if !file_path.exists() {
        let template = TEMPLATE.replace("{{DAY_ID}}", &format!("{day_id:0>2}"));
        std::fs::write(file_path, template).context("writing day file")?;
    }
    let mut args = vec![
        if test { "test" } else { "run" },
        "--release",
        "--bin",
        &day_str,
    ];
    if test {
        args.push("--");
        args.push("--show-output");
    }
    println!("cargo {}", args.join(" "));

    Command::new("cargo").args(args).spawn()?.wait()?;

    Ok(())
}

const TEMPLATE: &str = indoc::indoc! {"
type Data = i32;

fn main() {
    let input = std::fs::read_to_string(\"inputs/{{DAY_ID}}.txt\").unwrap();
    let input = aoc_helper::run_parser(parse, &input);
    aoc_helper::run_solution_vec!(part_1, &input);
    aoc_helper::run_solution_vec!(part_2, &input);
}

fn parse(input: &str) -> Vec<Data> {
    input.lines().map(|l| l.parse::<Data>().unwrap()).collect()
}

fn part_1(_input: &[Data]) -> i32 {
    0
}

fn part_2(_input: &[Data]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    const INPUTS: &str = indoc::indoc! {\"

    \"};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 0);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 0);
    }
}
"};
