use std::fs::read_to_string;

fn main() {
    let part1_input = read_to_string("./day_01/input_part_1.txt").expect("Could Not read file");
    let total: u32 = part1_input.lines().map(|l| parse_line(l.to_owned())).sum();
    println!("part 1 solution: {}", total);

    let total: u32 = part1_input.lines().map(parse_numbers).map(parse_line).sum();
    println!("part 2 solution: {}", total);
}

fn parse_numbers(input: &str) -> String {
    let index_values = [("one", "1"), ("two", "2"), ("three", "3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")];

    let mut results = index_values
        .iter()
        .filter_map(
            |(word, value)| input.find(word).map(
                |index| (index, *value, *word))
        )
        .collect::<Vec<(usize, &str, &str)>>();
    results.sort_by(|(a, _, _), (b, _, _)| a.partial_cmp(b).unwrap());

    let mut output = input.to_string();
    for (_, value, word) in results {
        output = output.replace(word, value);
    }
    output
}

fn parse_line(input: String) -> u32 {
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;

    for place in input.chars() {
        if place.is_numeric() {
            match first {
                None => {
                    first = place.to_digit(10);
                    last = first;
                }
                Some(_) => last = place.to_digit(10)
            }
        }
    }


    match (first, last) {
        (Some(x), Some(y)) => x * 10 + y,
        _ => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_parse_line() {
    //     let input = "1abc2";
    //     let output = parse_line(input);
    //
    //     assert_eq!(output, 12);
    //
    //     let input = "pqr3stu8vwx";
    //     let output = parse_line(input);
    //     assert_eq!(output, 38);
    //
    //     let input = "a1b2c3d4e5f";
    //     let output = parse_line(input);
    //     assert_eq!(output, 15);
    //
    //     let input = "treb7uchet";
    //     let output = parse_line(input);
    //     assert_eq!(output, 77)
    // }

    #[test]
    fn test_parse_numbers() {
        let input = "two1nine";
        let output = "219";
        assert_eq!(parse_numbers(input), output);

        let input = "eightwothree";
        let output = "8wo3";
        assert_eq!(parse_numbers(input), output);

        let input = "abcone2threexyz";
        let output = "abc123xyz";
        assert_eq!(parse_numbers(input), output);

        let input = "xtwone3four";
        let output = "x2ne34";
        assert_eq!(parse_numbers(input), output);

        let input = "4nineeightseven2";
        let output = "49872";
        assert_eq!(parse_numbers(input), output);

        let input = "zoneight234";
        let output = "z1ight234";
        assert_eq!(parse_numbers(input), output);

        let input = "7pqrstsixteen";
        let output = "7pqrst6teen";
        assert_eq!(parse_numbers(input), output);
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

        let total: u32 = lines.lines().map(parse_numbers).map(parse_line).sum();
        assert_eq!(total, 281);
    }
}
