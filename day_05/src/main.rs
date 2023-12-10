use std::fs::read_to_string;
use std::thread;

fn main() {
    let input = read_to_string("./day_05/part_1.txt").expect("Could Not read file");

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        match args[1].as_str() {
            "part_1" => {
                println!("Part 1: {}", part_1(&input))
            }
            "part_2" => {
                println!("Part 2: {}", part_2(&input))
            }
            _ => {
                println!("Usage: <day> <part>");
                std::process::exit(64);
            }
        }
    } else {
        println!("Usage: <day> <part>");
        std::process::exit(64);
    }
}

type Row = (u64, u64, u64);
type Section = Vec<Row>;

fn part_1(input: &str) -> u64 {
    let mut parts = input.split("\n\n");
    let seeds = parse_seeds(parts.next().unwrap());
    let sections = parts.map(parse_section).collect::<Vec<Section>>();

    seeds
        .iter()
        .map(|&seed| get_lowest_location(seed, &sections))
        .min()
        .unwrap()
}

fn get_lowest_location(seed: u64, sections: &[Section]) -> u64 {
    sections.iter().fold(seed, |current_seed, section| {
        section
            .iter()
            .fold(None, |result, &z| match result {
                None => map_src_to_destination(current_seed, z),
                Some(x) => Some(x),
            })
            .unwrap_or(current_seed)
    })
}

fn part_2(input: &str) -> u64 {
    let mut parts = input.split("\n\n");
    let seeds = parse_seeds(parts.next().unwrap());
    let sections = parts.map(parse_section).collect::<Vec<Section>>();
    let pairs = seeds
        .chunks(2)
        .map(|c| (c[0], c[0] + c[1]))
        .collect::<Vec<(u64, u64)>>();
    let mut handles = vec![];
    for pair in pairs.clone() {
        let sections_clone = sections.clone();
        let handle = thread::spawn(move || {
            let (from, to) = pair;
            dbg!(from, to);
            let mut result = u64::MAX;
            for i in from..to {
                let lowest = get_lowest_location(i, &sections_clone);
                if lowest < result {
                    result = lowest
                }
            }
            println!("{result}");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    0
}

fn map_src_to_destination(incoming: u64, map_section: Row) -> Option<u64> {
    match map_section {
        (dst, src, range) if src <= incoming && incoming <= (src + range) => {
            Some(dst + incoming - src)
        }
        _ => None,
    }
}

fn parse_section(input: &str) -> Section {
    input
        .split('\n')
        .skip(1)
        .map(|i| i.split_whitespace().filter_map(|x| x.parse::<u64>().ok()))
        .map(|mut x| (x.next().unwrap(), x.next().unwrap(), x.next().unwrap()))
        .collect()
}

fn parse_seeds(input: &str) -> Vec<u64> {
    let mut seed_line = input.split(": ").skip(1);
    seed_line
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"seeds: 79 14 55 13

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
56 93 4";

    #[test]
    fn test_map_src_to_destination_no_match() {
        let res = map_src_to_destination(14, (50, 98, 2));

        assert_eq!(res, Some(14));
    }
    #[test]
    fn test_map_src_to_destination_match() {
        let res = map_src_to_destination(51, (50, 98, 2));

        assert_eq!(res, Some(51));
    }
    #[test]
    fn test_map_src_to_destination_match_2() {
        let res = map_src_to_destination(79, (52, 50, 48));

        assert_eq!(res, Some(81));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 35)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 46)
    }
}
