type Data = (String, String);

fn main() {
    let input = std::fs::read_to_string("inputs/06.txt").unwrap();
    let input = aoc_helper::run_parser(parse, &input);
    aoc_helper::run_solution!(part_1, &input);
    aoc_helper::run_solution!(part_2, &input);
}

fn parse(input: &str) -> Data {
    let mut lines = input.lines();

    let time = lines.next().unwrap();

    let distance = lines.next().unwrap();

    (time.to_string(), distance.to_string())
}

fn count_possible_wins(time: u64, record: u64) -> u64 {
    let mut count = 0;
    for i in 0..time {
        let speed = i;
        let time_travelled = time - i;
        let dist = speed * time_travelled;
        if dist > record {
            count += 1;
        }
    }
    count
}

fn part_1((time, distance): &Data) -> u64 {
    let time = time
        .replace("Time:", "")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let distance = distance
        .replace("Distance:", "")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    time.iter()
        .zip(distance)
        .map(|(time, record)| count_possible_wins(*time, record))
        .product()
}

fn part_2((time, distance): &Data) -> u64 {
    let real_time = time
        .replace("Time:", "")
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let real_distance = distance
        .replace("Distance:", "")
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    count_possible_wins(real_time, real_distance)
}

#[cfg(test)]
mod tests {
    const INPUTS: &str = indoc::indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 288);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 71503);
    }
}
