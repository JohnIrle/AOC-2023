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

// need to find number and positions
// need to find symbols and postions
// need to iter over numbers and see if they are near symbolso
#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
struct Symbol {
    column: isize,
    row: isize,
}

impl Symbol {
    fn new(column: isize, row: isize) -> Self {
        Self {
            column,
            row,
        }
    }
}

fn parse_numbers(input: &str) -> Vec<Number> {
    let re = Regex::new(r"\d+").unwrap();
    let y = input
        .lines()
        .enumerate()
        .flat_map(|(index, line)| re.find_iter(line)
            .map(|m| Number::new(
                m.as_str().parse().unwrap(),
                m.start() as isize,
                (m.end() - 1) as isize,
                index as isize)
            ).collect::<Vec<Number>>()).collect();
    y
}

fn parse_symbols(input: &str) -> Vec<Symbol> {
    let re = Regex::new(r"[+*=@&#$\-/%]+").unwrap();
    let y = input
        .lines()
        .enumerate()
        .flat_map(|(index, line)| re.find_iter(line)
            .map(|m| Symbol::new(
                m.start() as isize,
                index as isize)
            ).collect::<Vec<Symbol>>()).collect();
    y
}

fn sum_adjacent_numbers(input: &str) -> u32 {
    let numbers = parse_numbers(input);
    let symbols = parse_symbols(input);

    let sum = numbers
        .iter()
        .filter(
            |number| symbols.iter().any(
                |symbol| check_bounds(number, symbol)
            )
        )
        .map(|number| number.number).sum();
    sum
}


fn check_bounds(number: &Number, symbol: &Symbol) -> bool {
    let is_adjacent_col = (number.start - 1..=number.end + 1).contains(&symbol.column);
    let is_adjacent_row = (number.row - 1..=number.row + 1).contains(&symbol.row);
    if is_adjacent_col && is_adjacent_row {
        return true;
    }

    false
}


#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;


    #[test]
    fn check_bounds_missing_numbers() {
        let number = Number::new(592, 2, 4, 6);
        let symbol = Symbol::new(5, 5);
        assert!(check_bounds(&number, &symbol))
    }

    #[test]
    fn test_check_bounds() {
        let number = Number::new(467, 0, 2, 0);
        let symbol = Symbol::new(3, 1);

        assert!(check_bounds(&number, &symbol))
    }

    #[test]
    fn test_check_bounds_above() {
        let number = Number::new(467, 0, 2, 1);
        let symbol = Symbol::new(1, 0);

        assert!(check_bounds(&number, &symbol));
    }

    #[test]
    fn test_check_bounds_no_hit() {
        let number = Number::new(114, 5, 8, 0);
        let symbol = Symbol::new(3, 0);

        assert!(!check_bounds(&number, &symbol));
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
}
