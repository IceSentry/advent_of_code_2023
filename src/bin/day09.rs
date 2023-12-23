type Data = Vec<i64>;

fn main() {
    let input = std::fs::read_to_string("inputs/09.txt").unwrap();
    let input = aoc_helper::run_parser(parse, &input);
    aoc_helper::run_solution_vec!(part_1, &input);
    aoc_helper::run_solution_vec!(part_2, &input);
}

fn parse(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn extrapolate(history: &[i64]) -> i64 {
    let differences = history
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<_>>();
    if differences.iter().all(|x| *x == 0) {
        *history.last().unwrap()
    } else {
        history.last().unwrap() + extrapolate(&differences)
    }
}

fn part_1(input: &[Data]) -> i64 {
    input.iter().map(|history| extrapolate(history)).sum()
}

fn part_2(input: &[Data]) -> i64 {
    input
        .iter()
        .map(|history| history.iter().rev().copied().collect::<Vec<_>>())
        .map(|history| extrapolate(&history))
        .sum()
}

#[cfg(test)]
mod tests {
    const INPUTS: &str = indoc::indoc! {"
        0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 114);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 2);
    }
}
