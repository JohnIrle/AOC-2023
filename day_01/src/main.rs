use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let input = read_to_string("./day_01/input_part_1.txt").expect("Could Not read file");
    let sum_1: u32 = input.lines().map(|l| parse_line(l, Regex::new(r"[0-9]").unwrap())).sum();
    let sum_2: u32 = input.lines().map(|l| parse_line(l, Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap())).sum();

    println!("part 1 solution: {}", sum_1);
    println!("part 2 solution: {}", sum_2);
}

fn parse_line(line: &str, regex: Regex) -> u32 {
    let words = parse_words(line, regex);

    let numbers: Vec<u32> = words.iter().map(|w| word_to_value(w)).collect();

    let first = numbers.first().unwrap_or(&0u32);
    let last = numbers.last().unwrap_or(&0u32);

    first * 10 + last
}

fn parse_words(line: &str, re: Regex) -> Vec<&str> {
    let mut words: Vec<&str> = Vec::new();
    for i in 0..line.len() {
        if let Some(word)= re.find(&line[i..]) {
            words.push(word.as_str())
        }
    }
    words
}

fn word_to_value(word: &str) -> u32 {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        x => x.parse::<u32>().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_part_1() {
        let lines = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let sum_1: u32 = lines.lines().map(|l| parse_line(l, Regex::new(r"\d").unwrap())).sum();
        assert_eq!(sum_1, 142)
    }

    #[test]
    fn test_part_2() {
        let lines = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let sum_2: u32 = lines.lines().map(|l| parse_line(l, Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap())).sum();
        assert_eq!(sum_2, 281);
    }
}
