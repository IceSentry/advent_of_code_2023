type Data = (u32, (u32, u32, u32));

fn main() {
    let input = std::fs::read_to_string("inputs/02.txt").unwrap();
    let input = aoc_helper::run_parser(parse, &input);
    aoc_helper::run_solution!(part_1, &input);
    aoc_helper::run_solution!(part_2, &input);
}

fn parse(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|l| {
            // Game {game_id}: {game_str}
            let (game_id, game_str) = l.split_once(':').unwrap();
            let game_id = game_id.replace("Game ", "").parse::<u32>().unwrap();
            let (mut max_r, mut max_g, mut max_b) = (0, 0, 0);
            for cubes_str in game_str.split(';') {
                // {count} {color},
                for cube in cubes_str.split(',') {
                    let (count, color) = cube.trim().split_once(' ').unwrap();
                    let count = count.parse::<u32>().unwrap();
                    match color {
                        "red" => max_r = count.max(max_r),
                        "green" => max_g = count.max(max_g),
                        "blue" => max_b = count.max(max_b),
                        _ => unreachable!(),
                    };
                }
            }
            (game_id, (max_r, max_g, max_b))
        })
        .collect()
}

fn part_1(input: &[Data]) -> u32 {
    input.iter().fold(0, |acc, &(id, (max_r, max_g, max_b))| {
        if max_r <= 12 && max_g <= 13 && max_b <= 14 {
            acc + id
        } else {
            acc
        }
    })
}

fn part_2(input: &[Data]) -> u32 {
    input.iter().fold(0, |acc, (_, (max_r, max_g, max_b))| {
        acc + (max_r * max_g * max_b)
    })
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
