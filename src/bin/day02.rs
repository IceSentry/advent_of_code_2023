type Data = (u32, Vec<Cube>);

fn main() {
    let input = std::fs::read_to_string("inputs/02.txt").unwrap();
    let input = aoc_helper::run_parser(parse, &input);
    aoc_helper::run_solution!(part_1, &input);
    aoc_helper::run_solution!(part_2, &input);
}

enum Cube {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn parse(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|l| {
            let (game_id, game_str) = sscanf::scanf!(l, "Game {}: {}", u32, String).unwrap();
            let mut game = vec![];
            for cubes_str in game_str.split(';') {
                for entry in cubes_str.split(',') {
                    let (count, color) = entry.trim().split_once(' ').unwrap();
                    let count = count.parse::<u32>().unwrap();
                    game.push(match color {
                        "red" => Cube::Red(count),
                        "green" => Cube::Green(count),
                        "blue" => Cube::Blue(count),
                        _ => unreachable!(),
                    });
                }
            }
            (game_id, game)
        })
        .collect()
}

fn part_1(input: &[Data]) -> u32 {
    let mut result = 0;
    for (id, game) in input {
        let possible = game.iter().all(|c| match *c {
            Cube::Red(red) => red <= 12,
            Cube::Green(green) => green <= 13,
            Cube::Blue(blue) => blue <= 14,
        });
        if possible {
            result += id;
        }
    }
    result
}

fn part_2(input: &[Data]) -> u32 {
    let mut result = 0;
    for (_id, game) in input {
        let (mut max_r, mut max_g, mut max_b) = (0, 0, 0);
        for cube in game.iter() {
            match *cube {
                Cube::Red(r) => max_r = r.max(max_r),
                Cube::Green(g) => max_g = g.max(max_g),
                Cube::Blue(b) => max_b = b.max(max_b),
            }
        }
        result += max_r * max_g * max_b;
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
