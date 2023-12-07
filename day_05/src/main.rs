use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./day_05/part_1.txt").expect("Could Not read file");

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        match args[1].as_str() {
            "part_1" => {
                println!("Part 1: {}", part_1(&input))
            }
            "part_2" => {}
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

type Section = (u64, u64, u64);

fn part_1(input: &str) -> u64 {
    let mut parts = input.split("\n\n");
    let seeds = parse_seeds(parts.next().unwrap());
    let sections = parts.map(parse_map);

    seeds
        .iter()
        .map(|&seed| {
            sections.clone().fold(seed, |current_seed, section| {
                section
                    .iter()
                    .fold(None, |result, &z| match result {
                        None => map_src_to_destination(current_seed, z),
                        Some(x) => Some(x),
                    })
                    .unwrap_or(current_seed)
            })
        })
        .min()
        .unwrap()
}

fn map_src_to_destination(incoming: u64, map_row: Section) -> Option<u64> {
    match map_row {
        (dst, src, range) if src <= incoming && incoming <= (src + range) => {
            Some(dst + incoming - src)
        }
        _ => None,
    }
}

fn parse_map(input: &str) -> Vec<Section> {
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

    fn test_part_2() {}
}
