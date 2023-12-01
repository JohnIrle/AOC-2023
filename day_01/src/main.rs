fn main() {
    println!("Hello, world!");
}

fn parse_line(input: &str) -> usize {
    let first_number = "";
    let second_numer = "";

    let digits = input.chars().filter(|c| c.is_numeric()).collect::<String>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "1abc2";
        let output = parse_line(input);

        assert_eq!(output, 12);

        let input = "pqr3stu8vwx";
        let output = parse_line(input);
        assert_eq!(output, 38);

        let input = "a1b2c3d4e5f";
        let output = parse_line(input);
        assert_eq!(output, 15);

        let input = "treb7uchet";
        let output = parse_line(input);
        assert_eq!(output, 77)
    }
}