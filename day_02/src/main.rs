use std::fs::read_to_string;

const RED_THRESHOLD: u32 = 12;
const GREEN_THRESHOLD: u32 = 13;
const BLUE_THRESHOLD: u32 = 14;

fn main() {
    let input = read_to_string("./day_02/input_part_1.txt").expect("Could Not read file");

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        match args[1].as_str() {
            "part_1" => {
                let part1_total = get_total(&input);
                println!("Part one: {}", part1_total);
            }
            "part_2" => {
                let part2_total = get_power_total(&input);
                println!("Part two: {}", part2_total);
            }
            _ =>  {
                println!("Usage: <day> <part>");
                std::process::exit(64);
            }
        }
    } else {
        println!("Usage: <day> <part>");
        std::process::exit(64);
    }
}

fn get_total(input: &str) -> u32 {
    input.lines().enumerate().map(|(index, line)| {
        let game_number = index + 1;
        let cubes = parse_cubes(line);
        if cubes.iter().all(|c| !is_over_threshold(*c)) {
            return game_number as u32
        }
        0
    }).sum::<u32>()
}

fn is_over_threshold((qty, color): (u32, &str)) -> bool {
    match color {
        "green" => qty > GREEN_THRESHOLD,
        "blue" => qty > BLUE_THRESHOLD,
        "red" => qty > RED_THRESHOLD,
        _ => unreachable!(),
    }
}

fn parse_cubes(line: &str) -> Vec<(u32, &str)> {
    let game = line.split(": ").collect::<Vec<&str>>()[1];
    let sets = game.split("; ");
    let cubes = sets
        .flat_map(|s| s.split(", "))
        .filter_map(|c| {
            let mut parts = c.split(' ');
            let qty = parts.next().map(|q| q.parse::<u32>().unwrap());
            let color = parts.next();
            match (qty, color) {
                (Some(qty), Some(color)) => Some((qty, color)),
                _ => None
            }
        });
    cubes.collect()
}

fn get_power_total(input: &str) -> u32 {
    input.lines().map(|line| {
        let cubes = parse_cubes(line);
        let mut max_green = 0;
        let mut max_blue = 0;
        let mut max_red = 0;
        for (qty, color) in cubes {
            match color {
                "red" => if qty > max_red {max_red = qty}
                "green" => if qty > max_green { max_green = qty}
                "blue" => if qty > max_blue { max_blue = qty }
                _ => unreachable!()
            }
        }
        max_green * max_blue * max_red
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;
        assert_eq!(get_total(input), 8)
    }

    #[test]
    fn test_part_2() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;
        assert_eq!(get_power_total(input), 2286)
    }
}