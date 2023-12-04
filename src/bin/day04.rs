use std::collections::{BTreeMap, HashMap};

type Data = (i32, Vec<i32>, Vec<i32>);

fn main() {
    let input = std::fs::read_to_string("inputs/04.txt").unwrap();
    let input = aoc_helper::run_parser(parse, &input);
    aoc_helper::run_solution_vec!(part_1, &input);
    aoc_helper::run_solution_vec!(part_2, &input);
}

fn parse(input: &str) -> Vec<Data> {
    input
        .trim()
        .lines()
        .map(|l| {
            let (card, game) = l.split_once(':').unwrap();
            let card_id = card.replace("Card", "").trim().parse::<i32>().unwrap();
            let (winning_numbers, numbers) = game.split_once('|').unwrap();
            let winning_numbers = winning_numbers
                .split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let numbers = numbers
                .split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            (card_id, winning_numbers, numbers)
        })
        // .inspect(|line| println!("{line:?}"))
        .collect()
}

fn part_1(input: &[Data]) -> i32 {
    input
        .iter()
        .fold(0, |acc, (_card_id, winning_numbers, numbers)| {
            acc + winning_numbers
                .iter()
                .filter(|wn| numbers.contains(wn))
                .fold(0, |acc, _| match acc {
                    0 => 1,
                    _ => acc * 2,
                })
        })
}

fn part_2(input: &[Data]) -> usize {
    let mut copies = BTreeMap::new();
    for (card_id, winning_numbers, numbers) in input {
        let matches = winning_numbers
            .iter()
            .filter(|wn| numbers.contains(wn))
            .count() as i32;

        let copies_for_card: i32 = *copies.entry(*card_id).or_insert(1);
        for i in 0..matches {
            *copies.entry(card_id + i + 1).or_insert(1) += copies_for_card;
        }
    }
    copies.values().map(|x| *x as usize).sum::<usize>()
}

#[cfg(test)]
mod tests {
    const INPUTS: &str = indoc::indoc! {"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 13);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 30);
    }
}
