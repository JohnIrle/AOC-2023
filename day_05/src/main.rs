use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./day_05/part_1.txt").expect("Could Not read file");

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        match args[1].as_str() {
            "part_1" => {}
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

fn parts(input: &str) {
    let mut parts = input.split("\n\n");
    let seeds = parse_seeds(parts.next().unwrap());
    let seed_to_soil = parse_map(parts.next().unwrap());
    let soil_to_fertilizer = parse_map(parts.next().unwrap());
    let fertilizer_to_water = parse_map(parts.next().unwrap());
    let water_to_light = parse_map(parts.next().unwrap());
    let light_to_temperature = parse_map(parts.next().unwrap());
    let temperature_to_humidity = parse_map(parts.next().unwrap());
    let humidity_to_location = parse_map(parts.next().unwrap());

    dbg!(
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location
    );
}

fn parse_map(input: &str) -> Vec<Option<(u32, u32, u32)>> {
    let numbers_part = input.split('\n').skip(1);
    let numbers = numbers_part.collect::<Vec<_>>();
    let number_tuples = numbers
        .iter()
        .map(|i| i.split_whitespace().filter_map(|x| x.parse::<u32>().ok()))
        .map(|x| match x.collect::<Vec<u32>>()[..] {
            [a, b, c] => Some((a, b, c)),
            _ => None,
        })
        .collect::<Vec<_>>();
    number_tuples
}

fn parse_seeds(input: &str) -> Vec<u32> {
    let mut seed_line = input.split(": ").skip(1);
    seed_line
        .next()
        .unwrap()
        .split_whitespace()
        .filter_map(|x| x.parse::<u32>().ok())
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
    fn test_part_1() {
        assert_eq!(parts(INPUT), ())
    }

    fn test_part_2() {}
}
