type Data = String;

fn main() {
    let input = std::fs::read_to_string("inputs/01.txt").unwrap();
    let input = parse(&input);
    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn parse(input: &str) -> Vec<Data> {
    input.lines().map(|l| l.parse::<Data>().unwrap()).collect()
}

fn part_1(input: &[Data]) -> u32 {
    let mut total = 0;
    for line in input {
        let mut digits = vec![];
        for c in line.chars() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d);
            }
        }
        let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        total += number.parse::<u32>().unwrap();
    }
    total
}

fn part_2(input: &[Data]) -> u32 {
    let mut total = 0;
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in input {
        let mut digits = vec![];
        let chars = line.chars().collect::<Vec<_>>();
        for (i, c) in chars.iter().enumerate() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d);
                continue;
            }
            let s = String::from_iter(&chars[i..chars.len()]);
            for (i, n) in numbers.iter().enumerate() {
                if s.starts_with(n) {
                    digits.push(i as u32 + 1);
                }
            }
        }
        let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
        total += number.parse::<u32>().unwrap();
    }
    total
}

fn _part_2_slow(input: &[Data]) -> u32 {
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
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let input = super::parse(input);
        let result = super::part_2(&input);
        assert_eq!(result, 281);
    }
}
