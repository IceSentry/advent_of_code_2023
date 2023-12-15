use std::collections::{HashMap, HashSet};

type Data = (HashMap<(i32, i32), Entry>, HashMap<usize, i32>);

fn main() {
    let input = std::fs::read_to_string("inputs/03.txt").unwrap();
    let input = aoc_helper::run_parser(parse, &input);
    aoc_helper::run_solution!(part_1, &input);
    aoc_helper::run_solution!(part_2, &input);
}

#[derive(Debug)]
enum Entry {
    // Index into the numbers array
    Id(usize),
    Symbol(char),
}

fn parse(input: &str) -> Data {
    let mut grid = HashMap::new();
    let mut numbers = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut num: Option<(u32, Vec<(i32, i32)>)> = None;
        for (x, c) in line.chars().enumerate() {
            let grid_pos = (x as i32, y as i32);
            if let Some(d) = c.to_digit(10) {
                if let Some((value, positions)) = num.as_mut() {
                    *value = (*value * 10) + d;
                    positions.push((x as i32, y as i32));
                } else {
                    num = Some((d, vec![(x as i32, y as i32)]));
                }
                grid.insert(grid_pos, Entry::Id(numbers.len()));
            } else {
                if let Some(num) = num.take() {
                    numbers.push(num);
                }
                if c != '.' {
                    grid.insert(grid_pos, Entry::Symbol(c));
                }
            };
        }
        if let Some(num) = num.take() {
            numbers.push(num);
        }
    }

    let mut part_numbers = HashMap::new();
    for (i, (number, positions)) in numbers.iter().enumerate() {
        'positions: for (x, y) in positions {
            for x_offset in [-1, 0, 1] {
                for y_offset in [-1, 0, 1] {
                    if let Some(Entry::Symbol(_)) = grid.get(&(*x + x_offset, *y + y_offset)) {
                        part_numbers.insert(i, *number as i32);
                        break 'positions;
                    }
                }
            }
        }
    }

    (grid, part_numbers)
}

fn part_1((_grid, numbers): &Data) -> i32 {
    numbers.values().sum()
}

fn check_neighbours_digit(grid: &HashMap<(i32, i32), Entry>, (x, y): (i32, i32)) -> Vec<usize> {
    let mut set = HashSet::new();
    for x_offset in [-1, 0, 1] {
        for y_offset in [-1, 0, 1] {
            if let Some(Entry::Id(id)) = grid.get(&(x + x_offset, y + y_offset)) {
                set.insert(*id);
            }
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
