use std::io;
use std::io::prelude::*;

mod pattern_matcher;

use pattern_matcher::PatternMatcher;

fn find_calibration_values(text: &str) -> impl Iterator<Item = Result<u32, LineWithOutNumber>> {
    let matchers = vec![
        PatternMatcher::new("one", 1),
        PatternMatcher::new("two", 2),
        PatternMatcher::new("three", 3),
        PatternMatcher::new("four", 4),
        PatternMatcher::new("five", 5),
        PatternMatcher::new("six", 6),
        PatternMatcher::new("seven", 7),
        PatternMatcher::new("eight", 8),
        PatternMatcher::new("nine", 9),
        PatternMatcher::new("0", 0),
        PatternMatcher::new("1", 1),
        PatternMatcher::new("2", 2),
        PatternMatcher::new("3", 3),
        PatternMatcher::new("4", 4),
        PatternMatcher::new("5", 5),
        PatternMatcher::new("6", 6),
        PatternMatcher::new("7", 7),
        PatternMatcher::new("8", 8),
        PatternMatcher::new("9", 9),
    ];

    let matchers_rev = matchers
        .iter()
        .map(|matcher| matcher.reverse())
        .collect::<Vec<_>>();

    text.lines().map(move |line| {
        let first = read_string_unitl_pattern_matched(line.chars(), matchers.clone());
        let last = read_string_unitl_pattern_matched(line.chars().rev(), matchers_rev.clone());

        if let (Some(first), Some(last)) = (first, last) {
            Ok(first * 10 + last)
        } else {
            Err(LineWithOutNumber(line))
        }
    })
}

#[derive(Debug, PartialEq)]
struct LineWithOutNumber<'a>(&'a str);

// Uses Iterator to avoid allocating a new string
fn read_string_unitl_pattern_matched(
    chars: impl Iterator<Item = char>,
    mut matchers: Vec<PatternMatcher>,
) -> Option<u32> {
    for c in chars {
        for matcher in matchers.iter_mut() {
            if matcher.matches(c) {
                return Some(matcher.value());
            }
        }
    }

    None
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)?;

    let sum = find_calibration_values(buffer.as_str())
        .try_fold(0, |acc, value| value.map(|value| acc + value));

    if let Ok(sum) = sum {
        println!("Sum of calibration values: {}", sum);
    } else {
        println!("Error on line: {:?}", sum);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_calibration_values() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

        let mut values = find_calibration_values(input);

        assert_eq!(values.next(), Some(Ok(29)));
        assert_eq!(values.next(), Some(Ok(83)));
        assert_eq!(values.next(), Some(Ok(13)));
        assert_eq!(values.next(), Some(Ok(24)));
        assert_eq!(values.next(), Some(Ok(42)));
        assert_eq!(values.next(), Some(Ok(14)));
        assert_eq!(values.next(), Some(Ok(76)));
        assert_eq!(values.next(), None);
    }
}
