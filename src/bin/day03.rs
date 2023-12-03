use std::collections::{HashMap, HashSet};

type Data = (HashMap<(i32, i32), Entry>, HashMap<usize, i32>);

fn main() {
    let input = std::fs::read_to_string("inputs/03.txt").unwrap();
    let input = aoc_helper::run_parser(parse, &input);
    aoc_helper::run_solution!(part_1, &input);
    aoc_helper::run_solution!(part_2, &input);
}

fn parse(input: &str) -> Data {
    let width = input.lines().next().unwrap().len();
    let input = input.replace('\n', "");
    let chars = input.chars().collect::<Vec<_>>();

    let mut numbers = vec![];
    let mut number_str = String::new();
    let mut keys = vec![];
    let mut grid = HashMap::new();
    for (i, c) in chars.iter().enumerate() {
        let x = i % width;
        let y = i / width;
        let key = (x as i32, y as i32);
        let entry = if c.is_ascii_digit() {
            number_str.push(*c);
            keys.push(key);
            if (i / width) != ((i + 1) / width) {
                let number = number_str.parse::<i32>().unwrap();
                numbers.push((keys.clone(), number));
                number_str.clear();
                keys.clear();
            }

            Entry::Id(numbers.len())
        } else {
            if !number_str.is_empty() {
                let number = number_str.parse::<i32>().unwrap();
                numbers.push((keys.clone(), number));
                number_str.clear();
                keys.clear();
            }
            if *c == '.' {
                continue;
            }
            Entry::Symbol(*c)
        };

        grid.insert(key, entry);
    }
    let mut part_numbers = HashMap::new();
    for (i, (keys, number)) in numbers.iter().enumerate() {
        'keys: for (x, y) in keys {
            #[rustfmt::skip]
            let directions = [
                (-1,  1), (0,  1),  (1,  1),
                (-1,  0),           (1,  0),
                (-1, -1), (0, -1),  (1, -1),
            ];
            for (x_offset, y_offset) in directions {
                if let Some(Entry::Symbol(_)) = grid.get(&(*x + x_offset, *y + y_offset)) {
                    part_numbers.insert(i, *number);
                    break 'keys;
                }
            }
        }
    }

    (grid, part_numbers)
}

fn part_1((_grid, numbers): &Data) -> i32 {
    numbers.values().sum()
}

enum Entry {
    Id(usize),
    Symbol(char),
}

fn check_neighbours_digit(grid: &HashMap<(i32, i32), Entry>, (x, y): (i32, i32)) -> Vec<usize> {
    #[rustfmt::skip]
    let directions = [
        (-1,  1), (0,  1),  (1,  1),
        (-1,  0),           (1,  0),
        (-1, -1), (0, -1),  (1, -1),
    ];
    let mut set = HashSet::new();
    for (x_offset, y_offset) in directions {
        if let Some(Entry::Id(id)) = grid.get(&(x + x_offset, y + y_offset)) {
            set.insert(*id);
        }
    }
    set.iter().copied().collect()
}

fn part_2((grid, numbers): &Data) -> i32 {
    let mut total = 0;
    for ((x, y), _) in grid.iter().filter(|(_, c)| matches!(c, Entry::Symbol('*'))) {
        let digits = check_neighbours_digit(grid, (*x, *y));
        if digits.len() == 2 {
            total += numbers[&digits[0]] * numbers[&digits[1]];
        }
    }
    total
}

#[cfg(test)]
mod tests {
    const INPUTS: &str = indoc::indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 4361);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 467835);
    }
}
