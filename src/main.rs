use anyhow::Context;
use std::process::Command;

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env");
    let mut args = std::env::args();
    _ = args.next();
    let day_id = args.next().unwrap();
    let day = format!("day{:0>2}", day_id);
    let test = args.next().is_some_and(|x| x == "test");

    let mut bin_dir = std::path::PathBuf::new();
    bin_dir.push("./src/bin/");

    let input_path = bin_dir.join(format!("inputs/{day_id:0>2}.txt"));
    if !input_path.exists() {
        let session_token = std::env::var("COOKIE_SESSION")?;
        let input = download_input(&session_token, 2023, 1)?;
        std::fs::create_dir_all(input_path.parent().unwrap()).context("creating dir")?;
        std::fs::write(&input_path, input).context("writing input file")?;
        println!("Input downloaded to {}", input_path.as_path().display());
    }

    let file_path = bin_dir.join(format!("day{day_id:0>2}.rs"));
    if !file_path.exists() {
        let template = TEMPLATE.replace("{{DAY_ID}}", &format!("{day_id:0>2}"));
        std::fs::write(file_path, template).context("writing day file")?;
    }

    Command::new("cargo")
        .args([if test { "test" } else { "run" }, "--bin", &day])
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn download_input(session_token: &str, year: u16, day: u8) -> anyhow::Result<String> {
    println!("Downloading inputs...");

    let response = ureq::get(&format!(
        "https://adventofcode.com/{}/day/{}/input",
        year, day
    ))
    .set("COOKIE", &format!("session={}", session_token))
    .set("User-Agent", "https://github.com/IceSentry/aoc_helper")
    .call();

    match response {
        Ok(response) => Ok(response.into_string()?),
        Err(ureq::Error::Status(code, _response)) => {
            anyhow::bail!("Failed to download inputs. status_code={}", code)
        }
        Err(_) => anyhow::bail!("Unknown error while downloading input"),
    }
}

const TEMPLATE: &str = "
use serde_scan::scan;

type Data = i32;

fn main() {
    let input = parse(include_str!(\"inputs/{{DAY_ID}}.txt\"));
    println!(\"part_1: {}\", part_1(&input));
    println!(\"part_2: {}\", part_2(&input));
}

fn parse(input: &str) -> Vec<Data> {
    input.lines().map(|l| scan!(\"{}\" <- l).unwrap()).collect()
}

fn part_1(input: &[Data]) -> i32 {
    0
}

fn part_2(input: &[Data]) -> i32 {
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
";
