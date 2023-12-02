use std::io;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, PartialEq)]
struct LineWithOutNumber<'a> (&'a str);

fn find_calibration_values(text: &str) -> impl Iterator<Item = Result<u32, LineWithOutNumber>> {
    text.lines().map(|line| {
        let first = line.chars().find_map(|c| c.to_digit(10));
        let last  = line.chars().rev().find_map(|c| c.to_digit(10));

        if let (Some(first), Some(last)) = (first, last) {
            Ok(first * 10 + last)
        } else {
            Err(LineWithOutNumber(line))
        }
    })
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
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";

        let mut values = find_calibration_values(input);

        assert_eq!(values.next(), Some(Ok(12)));
        assert_eq!(values.next(), Some(Ok(38)));
        assert_eq!(values.next(), Some(Ok(15)));
        assert_eq!(values.next(), Some(Ok(77)));
        assert_eq!(values.next(), None);
    }
}
