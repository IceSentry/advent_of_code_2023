
use serde_scan::scan;

type Data = i32;

fn main() {
    let input = parse(include_str!("inputs/01.txt"));
    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn parse(input: &str) -> Vec<Data> {
    input.lines().map(|l| scan!("{}" <- l).unwrap()).collect()
}

fn part_1(input: &[Data]) -> i32 {
    0
}

fn part_2(input: &[Data]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    const INPUTS: &str = indoc::indoc! {"

    "};

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
