fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day1.txt"));
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

// Part 1
fn part1(input: &str) -> String {
    let mut sum = 0;
    for line in input.split_whitespace() {
        let first_digit = line.chars().find(|c| c.is_numeric()).unwrap();
        let last_digit = line.chars().rfind(|c| c.is_numeric()).unwrap();
        let digit = String::from_iter([first_digit, last_digit])
            .parse::<u64>()
            .unwrap();
        sum += digit;
    }
    sum.to_string()
}

// Part 2
fn part2(input: &str) -> String {
    let digits_map: Vec<_> = vec![
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ]
    .into_iter()
    .map(|(k, v)| (k.chars().collect::<Vec<_>>(), v))
    .collect();

    let mut sum = 0;
    for line in input.split_whitespace() {
        let line_chars: Vec<_> = line.chars().collect();
        let mut first_digit = None;
        'first: for i in 0..line_chars.len() {
            let end_idx = line_chars.len().min(i + 5);
            for (pred, val) in &digits_map {
                if line_chars[i..end_idx].starts_with(&pred) {
                    first_digit = Some(val);
                    break 'first;
                }
            }
        }
        let mut last_digit = None;
        'last: for i in (0..line_chars.len()).rev() {
            let end_idx = line_chars.len().min(i + 5);
            for (pred, val) in &digits_map {
                if line_chars[i..end_idx].starts_with(&pred) {
                    last_digit = Some(val);
                    break 'last;
                }
            }
        }
        sum += first_digit.unwrap() * 10 + last_digit.unwrap();
    }
    sum.to_string()
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "#;
        assert_eq!(part1(input), "142");
    }

    #[test]
    fn part2_example() {
        let input = r#"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "#;
        assert_eq!(part2(input), "281");
    }
}
