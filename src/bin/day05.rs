use std::collections::HashMap;

type Data = (Vec<u32>, HashMap<String, (String, Vec<(u32, u32, u32)>)>);

fn main() {
    let input = std::fs::read_to_string("inputs/05.txt").unwrap();
    let input = aoc_helper::run_parser(parse, &input);
    aoc_helper::run_solution!(part_1, &input);
    aoc_helper::run_solution!(part_2, &input);
}

fn parse(input: &str) -> Data {
    let mut almanac = input.split("\n\n");
    let seeds = almanac.next().unwrap();
    let (_, seeds) = seeds.split_once(':').unwrap();
    let seeds = seeds
        .split_ascii_whitespace()
        .map(|seed| seed.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut maps = HashMap::new();
    for map in almanac {
        let (category, ranges) = map.split_once(" map:").unwrap();
        let (source_category, dest_category) = category.split_once("-to-").unwrap();
        for range in ranges.trim().lines() {
            let range = range.split_ascii_whitespace().collect::<Vec<_>>();
            let [dest_start, source_start, len] = range.as_slice() else {
                unreachable!()
            };
            maps.entry(source_category.to_string())
                .or_insert((dest_category.to_string(), vec![]))
                .1
                .push((
                    dest_start.parse::<u32>().unwrap(),
                    source_start.parse::<u32>().unwrap(),
                    len.parse::<u32>().unwrap(),
                ));
        }
    }

    for (_, map) in maps.values_mut() {
        map.sort_by(|(_, a, _), (_, b, _)| a.cmp(b));
    }

    (seeds, maps)
}

fn part_1((seeds, maps): &Data) -> u32 {
    let mut min_location = u32::MAX;
    for &seed in seeds {
        let mut value = seed;
        let mut source_category = String::from("seed");
        loop {
            let (dest_category, map) = &maps[&source_category];
            source_category = dest_category.clone();

            // remap
            for (dest_start, source_start, len) in map {
                let source_end = source_start + len;
                if *source_start <= value && value <= source_end {
                    value = dest_start + (value - source_start);
                    break;
                }
            }

            if dest_category == "location" {
                break;
            }
        }
        min_location = min_location.min(value);
    }
    min_location
}

fn part_2((seeds, maps): &Data) -> u32 {
    let mut source_category = String::from("seed");
    let mut intervals = seeds
        .chunks_exact(2)
        .map(|interval| [interval[0], interval[0] + interval[1]])
        .collect::<Vec<_>>();

    loop {
        let (dest_category, map) = &maps[&source_category];
        source_category = dest_category.clone();

        // remap intervals
        let mut next_intervals = vec![];
        for [interval_start, interval_end] in &intervals {
            let mut map_iter = map.iter().peekable();
            let mut overlap = false;
            while let Some((dest_start, source_start, len)) = map_iter.next() {
                let dest_end = dest_start + len;
                let source_end = source_start + len;
                // check if overlap between map range and interval
                if !(*source_start <= *interval_end && *interval_start <= source_end) {
                    continue;
                }
                overlap = true;

                let start = (*source_start).max(*interval_start);
                let end = (source_end).min(*interval_end);

                // interval before the overlap
                if start > *interval_start {
                    next_intervals.push([*interval_start, start]);
                }

                // overlap
                let remapped_interval = [
                    *dest_start + (start - *source_start),
                    dest_end + (end - source_end),
                ];
                next_intervals.push(remapped_interval);

                // interval after overlap
                if let Some((_, next_source_start, _)) = map_iter.peek() {
                    if end < *interval_end && *interval_end < *next_source_start {
                        next_intervals.push([end, *interval_end.min(next_source_start)]);
                    }
                }
            }
            // in some cases there aren't any overlap so just push it back
            if !overlap {
                next_intervals.push([*interval_start, *interval_end]);
            }
        }
        intervals = next_intervals;

        if dest_category == "location" {
            break;
        }
    }
    intervals.iter().map(|[a, _]| *a).min().unwrap()
}

#[cfg(test)]
mod tests {
    const INPUTS: &str = indoc::indoc! {"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 35);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 46);
    }
}
