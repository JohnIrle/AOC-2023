use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let input = read_to_string("./day_03/part_1.txt").expect("Could Not read file");

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        match args[1].as_str() {
            "part_1" => {
                let part1_total = sum_adjacent_numbers(&input);
                println!("Part one: {}", part1_total);
            }
            "part_2" => {
                let part2_total = sum_gear_ratios(&input);
                println!("Part two: {}", part2_total);
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

#[derive(Debug)]
struct Number {
    number: u32,
    start: isize,
    end: isize,
    row: isize,
}

impl Number {
    fn new(number: u32, start: isize, end: isize, row: isize) -> Self {
        Self {
            number,
            start,
            end,
            row,
        }
    }
}

#[derive(Debug)]
struct Symbol<'a> {
    symbol: &'a str,
    column: isize,
    row: isize,
}

impl<'a> Symbol<'a> {
    fn new(symbol: &'a str, column: isize, row: isize) -> Self {
        Self {
            symbol,
            column,
            row,
        }
    }
}

fn parse_numbers(input: &str) -> Vec<Number> {
    let re = Regex::new(r"\d+").unwrap();
    input
        .lines()
        .enumerate()
        .flat_map(|(index, line)| re.find_iter(line)
            .map(|m| Number::new(
                m.as_str().parse().unwrap(),
                m.start() as isize,
                (m.end() - 1) as isize,
                index as isize)
            ).collect::<Vec<Number>>()).collect()
}

fn parse_symbols(input: &str) -> Vec<Symbol> {
    let re = Regex::new(r"[+*=@&#$\-/%]+").unwrap();
    input
        .lines()
        .enumerate()
        .flat_map(|(index, line)| re.find_iter(line)
            .map(|m| Symbol::new(
                m.as_str(),
                m.start() as isize,
                index as isize)
            ).collect::<Vec<Symbol>>()).collect()
}

fn sum_adjacent_numbers(input: &str) -> u32 {
    let numbers = parse_numbers(input);
    let symbols = parse_symbols(input);

    let sum = numbers
        .iter()
        .filter(
            |number| symbols.iter().any(
                |symbol| check_symbols_adjacent(number, symbol)
            )
        )
        .map(|number| number.number).sum();
    sum
}

fn sum_gear_ratios(input: &str) -> u32 {
    let numbers = parse_numbers(input);
    let symbols = parse_symbols(input);
    let gears = symbols.iter().filter(|&s| s.symbol == "*").collect::<Vec<&Symbol>>();
    gears.iter().map(|g| check_two_numbers_adjacent(g, &numbers)).sum()
}

fn check_symbols_adjacent(number: &Number, symbol: &Symbol) -> bool {
    let is_adjacent_col = (number.start - 1..=number.end + 1).contains(&symbol.column);
    let is_adjacent_row = (number.row - 1..=number.row + 1).contains(&symbol.row);

    is_adjacent_col && is_adjacent_row
}

fn check_two_numbers_adjacent(gear: &Symbol, numbers: &[Number]) -> u32 {
    let adjacent_numbers = numbers
        .iter()
        .filter(|n| check_symbols_adjacent(n, gear))
        .map(|n| n.number)
        .collect::<Vec<u32>>();

    match adjacent_numbers.len() {
        2 => adjacent_numbers.iter().product::<u32>(),
        _ => 0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;


    #[test]
    fn check_bounds_missing_numbers() {
        let number = Number::new(592, 2, 4, 6);
        let symbol = Symbol::new("*", 5, 5);
        assert!(check_symbols_adjacent(&number, &symbol))
    }

    #[test]
    fn test_check_bounds() {
        let number = Number::new(467, 0, 2, 0);
        let symbol = Symbol::new("+", 3, 1);

        assert!(check_symbols_adjacent(&number, &symbol))
    }

    #[test]
    fn test_check_bounds_above() {
        let number = Number::new(467, 0, 2, 1);
        let symbol = Symbol::new("-", 1, 0);

        assert!(check_symbols_adjacent(&number, &symbol));
    }

    #[test]
    fn test_check_bounds_no_hit() {
        let number = Number::new(114, 5, 8, 0);
        let symbol = Symbol::new("&", 3, 0);

        assert!(!check_symbols_adjacent(&number, &symbol));
    }

    #[test]
    fn test_parse_numbers_parses_one_line() {
        let input = r#"467..114.."#;
        let numbers = parse_numbers(input);
        assert_debug_snapshot!(numbers, @r###"
        [
            Number {
                number: 467,
                start: 0,
                end: 2,
                row: 0,
            },
            Number {
                number: 114,
                start: 5,
                end: 7,
                row: 0,
            },
        ]
        "###);
    }

    #[test]
    fn test_parse_numbers_parses_multi_line() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
        let numbers = parse_numbers(input);
        assert_debug_snapshot!(numbers);
    }

    #[test]
    fn test_parse_numbers_parses_single_line() {
        let input = r#"...*......"#;
        let symbols = parse_symbols(input);
        assert_debug_snapshot!(symbols, @r###"
        [
            Symbol {
                symbol: "*",
                column: 3,
                row: 0,
            },
        ]
        "###)
    }

    #[test]
    fn test_parse_symbols_parses_multi_line() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
        let symbols = parse_symbols(input);
        assert_debug_snapshot!(symbols);
    }

    #[test]
    fn test_part_1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
        assert_eq!(sum_adjacent_numbers(input), 4361);
    }

    #[test]
    fn test_part_2() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;
        assert_eq!(sum_gear_ratios(input), 467835);
    }
}
