type Data = (u32, Vec<Round>);

fn main() {
    let input = std::fs::read_to_string("inputs/02.txt").unwrap();
    let input = aoc_helper::run_parser(parse, &input);
    aoc_helper::run_solution!(part_1, &input);
    aoc_helper::run_solution!(part_2, &input);
}

#[derive(Default, Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|l| {
            let (game_id, game) = sscanf::scanf!(l, "Game {}: {}", u32, String).unwrap();
            let mut rounds = vec![];
            for round_str in game.split(';') {
                let mut round = Round::default();
                for entry in round_str.split(',') {
                    let entry = entry.trim();
                    let (count, color) = entry.split_once(' ').unwrap();
                    let count = count.parse::<u32>().unwrap();
                    match color {
                        "red" => round.red = count,
                        "green" => round.green = count,
                        "blue" => round.blue = count,
                        _ => unreachable!(),
                    }
                }
                rounds.push(round);
            }
            (game_id, rounds)
        })
        .collect()
}

fn part_1(input: &[Data]) -> u32 {
    let mut result = 0;
    for (id, rounds) in input {
        let mut game = Round::default();
        for round in rounds {
            game.red = round.red.max(game.red);
            game.green = round.green.max(game.green);
            game.blue = round.blue.max(game.blue);
        }
        if game.red <= 12 && game.green <= 13 && game.blue <= 14 {
            result += id;
        }
    }
    result
}

fn part_2(input: &[Data]) -> u32 {
    let mut result = 0;
    for (_id, rounds) in input {
        let mut game = Round::default();
        for round in rounds {
            game.red = round.red.max(game.red);
            game.green = round.green.max(game.green);
            game.blue = round.blue.max(game.blue);
        }
        result += game.red * game.green * game.blue;
    }
    result
}

#[cfg(test)]
mod tests {
    const INPUTS: &str = indoc::indoc! {"
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 8);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 2286);
    }
}
