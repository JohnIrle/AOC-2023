use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./day_04/part_1.txt").expect("Could Not read file");

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        match args[1].as_str() {
            "part_1" => {
                println!("Part 1: {}", total_points(&input));
            }
            "part_2" => {
                println!("Part 2: {}", total_cards(&input));
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

fn total_points(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let (winning_numbers, cards) = parse_line(line);
        let winners_score = cards
            .iter()
            .filter(|card| winning_numbers.contains(card))
            .collect::<Vec<_>>();
        match winners_score.len() {
            0 => {}
            1 => total += 1,
            n => total += 2_usize.pow(n as u32 - 1),
        }
    }
    total
}

fn total_cards(input: &str) -> usize {
    let mut cards_at_index = vec![1; input.lines().collect::<Vec<_>>().len()];

    input.lines().enumerate().for_each(|(index, line)| {
        let (winning_numbers, cards) = parse_line(line);
        let winners_score = cards
            .iter()
            .filter(|card| winning_numbers.contains(card))
            .collect::<Vec<_>>();

        let num_winners = winners_score.len();
        let number_of_cards_at_index = cards_at_index.get(index);

        if let Some(number) = number_of_cards_at_index {
            for _ in 0..*number {
                for i in 0..num_winners {
                    if let Some(value) = cards_at_index.get_mut((index + i) + 1) {
                        *value += 1
                    }
                }
            }
        }
    });

    cards_at_index.iter().sum()
}

fn parse_line(line: &str) -> (Vec<&str>, Vec<&str>) {
    let numbers = line.split(':').collect::<Vec<&str>>()[1].trim();
    let mut rest = numbers.split(" | ");
    let (winning, cards) = (rest.next().unwrap(), rest.next().unwrap());

    (
        winning.split_whitespace().collect(),
        cards.split_whitespace().collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_part_1() {
        assert_eq!(total_points(INPUT), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(total_cards(INPUT), 30);
    }
}
