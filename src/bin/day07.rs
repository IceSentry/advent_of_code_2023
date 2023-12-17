use std::{cmp::Ordering, collections::BTreeMap};

type Data = ([char; 5], u32, Kind, Kind);

fn main() {
    let input = std::fs::read_to_string("inputs/07.txt").unwrap();
    let input = aoc_helper::run_parser(parse, &input);
    aoc_helper::run_solution_vec!(part_1, &input);
    aoc_helper::run_solution_vec!(part_2, &input);
}

fn parse(input: &str) -> Vec<Data> {
    input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();
            let hand: [char; 5] = hand.chars().collect::<Vec<_>>().try_into().unwrap();
            (
                hand,
                bid.parse::<u32>().unwrap(),
                hand_kind(&hand, false), // part_1
                hand_kind(&hand, true),  // part_2
            )
        })
        .collect()
}

fn card_strength(card: char, joker_rule: bool) -> u32 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if joker_rule {
                0
            } else {
                11
            }
        }
        'T' => 10,
        card if card.is_ascii_digit() => card.to_digit(10).unwrap(),
        _ => unimplemented!("card_strength: {card}"),
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Kind {
    /// where all five cards have the same label: AAAAA
    FiveOfAKind = 7,
    /// where four cards have the same label and one card has a different label: AA8AA
    FourOfAKind = 6,
    /// where three cards have the same label, and the remaining two cards share a different label: 23332
    FullHouse = 5,
    /// where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    ThreeOfAKind = 4,
    /// where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    TwoPair = 3,
    /// where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    OnePair = 2,
    /// where all cards' labels are distinct: 23456
    HighCard = 1,
}

fn hand_kind(hand: &[char], joker_rule: bool) -> Kind {
    let mut hand_set = BTreeMap::new();
    let mut jokers = 0;
    for card in hand {
        if *card == 'J' && joker_rule {
            jokers += 1;
        } else {
            *hand_set.entry(card).or_insert(0) += 1;
        }
    }
    let mut v = hand_set.values().collect::<Vec<_>>();
    v.sort();
    v.reverse();

    match v.as_slice() {
        [5] => Kind::FiveOfAKind,
        [c] if jokers + *c == 5 => Kind::FiveOfAKind,
        [] if jokers == 5 => Kind::FiveOfAKind,

        [4, 1] => Kind::FourOfAKind,
        [c, 1] if jokers + *c == 4 => Kind::FourOfAKind,

        [3, 2] => Kind::FullHouse,
        [2, 2] if jokers == 1 => Kind::FullHouse,

        [3, 1, 1] => Kind::ThreeOfAKind,
        [c, 1, 1] if jokers + *c == 3 => Kind::ThreeOfAKind,

        [2, 2, 1] => Kind::TwoPair,

        [2, 1, 1, 1] => Kind::OnePair,
        [1, 1, 1, 1] if jokers == 1 => Kind::OnePair,

        [1, 1, 1, 1, 1] => Kind::HighCard,

        _ => unimplemented!("{v:?} jokers: {jokers}"),
    }
}

fn compute_total_winnings(input: &[Data], joker_rule: bool) -> u32 {
    let mut input = input.to_vec();
    input.sort_by(
        |(hand_a, _, kind_a, kind_a_joker), (hand_b, _, kind_b, kind_b_joker)| {
            let kind_a = if joker_rule { kind_a_joker } else { kind_a };
            let kind_b = if joker_rule { kind_b_joker } else { kind_b };
            match kind_a.cmp(kind_b) {
                Ordering::Less => Ordering::Less,
                Ordering::Equal => {
                    for i in 0..5 {
                        let card_a = card_strength(hand_a[i], joker_rule);
                        let card_b = card_strength(hand_b[i], joker_rule);
                        match card_a.cmp(&card_b) {
                            Ordering::Less => return Ordering::Less,
                            Ordering::Equal => continue,
                            Ordering::Greater => return Ordering::Greater,
                        }
                    }
                    unreachable!()
                }
                Ordering::Greater => Ordering::Greater,
            }
        },
    );
    input
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid, _, _))| {
            let rank = i as u32 + 1;
            acc + (bid * rank)
        })
}

fn part_1(input: &[Data]) -> u32 {
    compute_total_winnings(input, false)
}

fn part_2(input: &[Data]) -> u32 {
    compute_total_winnings(input, true)
}

#[cfg(test)]
mod tests {
    const INPUTS: &str = indoc::indoc! {"
        32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 6440);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 5905);
    }
}
