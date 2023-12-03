use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Data = String;

fn main() {
    let input = std::fs::read_to_string("inputs/01.txt").unwrap();
    let input = aoc_helper::run_parser(parse, &input);
    aoc_helper::run_solution_vec!(part_1, &input);
    aoc_helper::run_solution_vec!(part_1_two_pass, &input);
    aoc_helper::run_solution_vec!(part_2_starts_with, &input);
    aoc_helper::run_solution_vec!(part_2_two_pass, &input);
    aoc_helper::run_solution_vec!(part_2_rayon, &input);
    aoc_helper::run_solution_vec!(part_2_find, &input);
}

fn parse(input: &str) -> Vec<Data> {
    input.lines().map(|l| l.parse::<Data>().unwrap()).collect()
}

fn part_1(input: &[Data]) -> u32 {
    let mut total = 0;
    for line in input {
        let digits = line
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<Vec<_>>();
        let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        total += number.parse::<u32>().unwrap();
    }
    total
}

fn part_1_two_pass(input: &[Data]) -> u32 {
    let mut total = 0;
    for line in input {
        let first = line.chars().find(|c| c.is_ascii_digit()).unwrap();
        let last = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
        let number = format!("{}{}", first, last);
        total += number.parse::<u32>().unwrap();
    }
    total
}

fn part_2_starts_with(input: &[Data]) -> u32 {
    let mut total = 0;
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in input {
        let mut digits = vec![];
        for (ci, c) in line.chars().enumerate() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d);
                continue;
            }
            let s = &line[ci..];
            for (i, n) in numbers.iter().enumerate() {
                if s.starts_with(n) {
                    digits.push(i as u32 + 1);
                    break;
                }
            }
        }
        let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        total += number.parse::<u32>().unwrap();
    }
    total
}

fn part_2_rayon(input: &[Data]) -> u32 {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let total = input
        .par_iter()
        .map(|line| {
            let mut digits = vec![];
            for (ci, c) in line.chars().enumerate() {
                if let Some(d) = c.to_digit(10) {
                    digits.push(d);
                    continue;
                }
                let s = &line[ci..];
                for (i, n) in numbers.iter().enumerate() {
                    if s.starts_with(n) {
                        digits.push(i as u32 + 1);
                        break;
                    }
                }
            }
            let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            number.parse::<u32>().unwrap()
        })
        .sum::<u32>();
    total
}

fn part_2_two_pass(input: &[Data]) -> u32 {
    let mut total = 0;
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in input {
        let mut digits = vec![];
        'first: for (ci, c) in line.chars().enumerate() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d);
                break 'first;
            }
            let s = &line[ci..];
            for (i, n) in numbers.iter().enumerate() {
                if s.starts_with(n) {
                    digits.push(i as u32 + 1);
                    break 'first;
                }
            }
        }
        'last: for (ci, c) in line.chars().rev().enumerate() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d);
                break 'last;
            }
            let s = &line[..line.chars().count() - ci];
            for (i, n) in numbers.iter().enumerate() {
                if s.ends_with(n) {
                    digits.push(i as u32 + 1);
                    break 'last;
                }
            }
        }
        let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        total += number.parse::<u32>().unwrap();
    }
    total
}

fn part_2_find(input: &[Data]) -> u32 {
    let mut total = 0;
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in input {
        let mut digits = vec![];
        for (i, n) in numbers.iter().enumerate() {
            if let Some(idx) = line.find(n) {
                digits.push((idx, i as u32 + 1));
            }
            if let Some(idx) = line.rfind(n) {
                digits.push((idx, i as u32 + 1));
            }
        }
        for (idx, c) in line.chars().enumerate() {
            if let Some(d) = c.to_digit(10) {
                digits.push((idx, d));
            }
        }
        digits.sort_by_key(|x| x.0);

        let number = format!("{}{}", digits.first().unwrap().1, digits.last().unwrap().1);
        total += number.parse::<u32>().unwrap();
    }
    total
}

#[cfg(test)]
mod tests {
    const INPUTS: &str = indoc::indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 142);
    }

    #[test]
    pub fn part_2() {
        let input = indoc::indoc! {"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "};
        let input = super::parse(input);
        let result = super::part_2_starts_with(&input);
        assert_eq!(result, 281);
        let result = super::part_2_find(&input);
        assert_eq!(result, 281);
    }
}
