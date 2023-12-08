use std::io::prelude::*;
use std::{io, process};

#[derive(Debug, PartialEq)]
enum ScratchCardError {
    WrongFormat,
}

fn points_for_scratchcard(card: &str) -> Result<u32, ScratchCardError> {
    let card = card.split_once(':').ok_or(ScratchCardError::WrongFormat)?.1;
    let (winning_numbers, numbers) = card.split_once('|').ok_or(ScratchCardError::WrongFormat)?;

    let mut winning_numbers = winning_numbers
        .split_whitespace()
        .map(|n| n.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ScratchCardError::WrongFormat)?;
    let numbers = numbers
        .split_whitespace()
        .map(|n| n.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ScratchCardError::WrongFormat)?;

    winning_numbers.sort_unstable();

    let winning_numbers_count = numbers
        .iter()
        .filter(|n| winning_numbers.binary_search(n).is_ok())
        .count();

    if winning_numbers_count == 0 {
        return Ok(0);
    }

    Ok(2u32.pow(winning_numbers_count as u32 - 1))
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut sum = 0;

    for card in buffer.lines() {
        if let Ok(x) = points_for_scratchcard(card) {
            sum += x;
        } else {
            println!("Wrong format");
            process::exit(1);
        }
    }

    println!("{}", sum);
}

#[cfg(test)]
mod test {
    use super::*;

    fn test(card: &str, points: u32) {
        assert_eq!(points_for_scratchcard(card), Ok(points));
    }

    #[test]
    fn test_game_1() {
        test("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8);
    }

    #[test]
    fn test_game_2() {
        test("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2);
    }

    #[test]
    fn test_game_3() {
        test("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2);
    }

    #[test]
    fn test_game_4() {
        test("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1);
    }

    #[test]
    fn test_game_5() {
        test("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0);
    }

    #[test]
    fn test_game_6() {
        test("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0);
    }
}
