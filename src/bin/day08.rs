use std::collections::BTreeMap;

type Data = (Vec<char>, BTreeMap<String, (String, String)>);

fn main() {
    let input = std::fs::read_to_string("inputs/08.txt").unwrap();
    let input = aoc_helper::run_parser(parse, &input);
    aoc_helper::run_solution!(part_1, &input);
    aoc_helper::run_solution!(part_2, &input);
}

fn parse(input: &str) -> Data {
    let (directions, map) = input.split_once("\n\n").unwrap();

    let directions = directions.chars().collect::<Vec<_>>();

    let mut nodes = BTreeMap::new();

    for line in map.lines() {
        let (node, elements) = line.split_once(" = ").unwrap();
        let elements = elements.replace(['(', ')'], "");
        let (left, right) = elements.split_once(", ").unwrap();
        nodes.insert(node.to_string(), (left.to_string(), right.to_string()));
    }

    (directions, nodes)
}

fn count_steps(
    start_node: &str,
    directions: &[char],
    nodes: &BTreeMap<String, (String, String)>,
    cond: fn(&str) -> bool,
) -> u64 {
    let mut node = start_node;
    let mut steps = 0;
    while cond(node) {
        for dir in directions {
            let (left, right) = &nodes[node];
            match dir {
                'L' => node = left,
                'R' => node = right,
                _ => unimplemented!("{dir}"),
            }
            steps += 1;
        }
    }
    steps
}

fn part_1((directions, nodes): &Data) -> u64 {
    count_steps("AAA", directions, nodes, |node| node != "ZZZ")
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

// from <https://gist.github.com/victor-iyi/8a84185c1d52419b0d4915a648d5e3e1>
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn part_2((directions, nodes): &Data) -> u64 {
    let steps = nodes
        .keys()
        .filter(|node| node.ends_with('A'))
        .map(|start_node| count_steps(start_node, directions, nodes, |node| !node.ends_with('Z')))
        .collect::<Vec<_>>();
    steps.iter().fold(steps[0], |acc, &x| lcm(acc, x))
}

#[cfg(test)]
mod tests {
    const INPUTS_1: &str = indoc::indoc! {"
        RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)
    "};

    const INPUTS_2: &str = indoc::indoc! {"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    "};

    const INPUTS_3: &str = indoc::indoc! {"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS_1);
        let result = super::part_1(&input);
        assert_eq!(result, 2);

        let input = super::parse(INPUTS_2);
        let result = super::part_1(&input);
        assert_eq!(result, 6);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS_3);
        let result = super::part_2(&input);
        assert_eq!(result, 6);
    }
}
