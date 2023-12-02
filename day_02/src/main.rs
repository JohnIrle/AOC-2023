use std::fs::read_to_string;

const RED_THRESHOLD:u32 = 12;
const GREEN_THRESHOLD:u32 = 13;
const BLUE_THRESHOLD:u32 = 14;

fn main() {
    let input = read_to_string("./day_02/input_part_1.txt").expect("Could Not read file");

    let total = get_total(&input);

    println!("Part one: {}", total);
}

fn get_total(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        let game = line.split(':').collect::<Vec<&str>>()[0];
        let number = game.trim().split(' ').collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let rest = line.split(':').collect::<Vec<&str>>()[1];
        let sets = rest.split(';').collect::<Vec<&str>>().join(",");
        let sets_over_threshold = sets.split(',').filter(|s| is_over_threshold(s)).collect::<Vec<&str>>();
        if sets_over_threshold.is_empty() {
            total += number;
        }
    }
    total
}


fn is_over_threshold(item: &str) -> bool {
    let number_color = item.trim().split(' ').collect::<Vec<&str>>();
    let number = number_color[0];
    let parsed_number = number.parse::<u32>().unwrap();
    let color = number_color[1];
    match color {
        "green" => parsed_number > GREEN_THRESHOLD,
        "blue" => parsed_number > BLUE_THRESHOLD,
        "red" => parsed_number > RED_THRESHOLD,
        _ => unreachable!(),
    }
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
        // assert_eq!(get_total(input), 2286)
    }
}