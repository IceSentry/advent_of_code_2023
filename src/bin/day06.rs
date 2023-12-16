type Data = (Vec<i32>, Vec<i32>);

fn main() {
    let input = std::fs::read_to_string("inputs/06.txt").unwrap();
    let input = aoc_helper::run_parser(parse, &input);
    aoc_helper::run_solution!(part_1, &input);
    aoc_helper::run_solution!(part_2, &input);
}

fn parse(input: &str) -> Data {
    let mut lines = input.lines();

    let time = lines.next().unwrap();
    let time = time
        .replace("Time:", "")
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let distance = lines.next().unwrap();
    let distance = distance
        .replace("Distance:", "")
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    (time, distance)
}

fn part_1((time, distance): &Data) -> i32 {
    let mut total = vec![];
    for (time, record) in time.iter().zip(distance) {
        let mut count = 0;
        for i in 0..*time {
            let speed = i;
            let time_travelled = time - i;
            let dist = speed * time_travelled;
            if dist > *record {
                count += 1;
            }
        }
        total.push(count);
    }
    total.iter().product()
}

fn part_2((time, distance): &Data) -> usize {
    let real_time = time
        .iter()
        .fold(String::new(), |acc, t| format!("{acc}{t}"))
        .parse::<usize>()
        .unwrap();
    let real_distance = distance
        .iter()
        .fold(String::new(), |acc, d| format!("{acc}{d}"))
        .parse::<usize>()
        .unwrap();

    println!("{real_time}\n{real_distance}");
    let mut count = 0;
    for i in 0..real_time {
        let speed = i;
        let time_travelled = real_time - i;
        let dist = speed * time_travelled;
        if dist > real_distance {
            count += 1;
        }
    }

    count
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
