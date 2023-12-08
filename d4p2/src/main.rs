use std::collections::VecDeque;
use std::io::prelude::*;
use std::{io, process};

#[derive(Debug, PartialEq)]
enum ScratchCardError {
    WrongFormat,
}

fn points_for_scratchcards(cards: &str) -> Result<u32, ScratchCardError> {
    let mut sum = 0;
    let mut next_cards = VecDeque::new();

    for card in cards.lines() {
        if let Ok(s) = points_for_scratchcard(card) {
            let amount_of_this_card = next_cards.pop_front().unwrap_or(0) + 1;

            sum += amount_of_this_card;

            while next_cards.len() < s as usize {
                next_cards.push_back(0);
            }

            next_cards
                .iter_mut()
                .take(s as usize)
                .for_each(|n| *n += amount_of_this_card);
        } else {
            return Err(ScratchCardError::WrongFormat);
        }
    }

    Ok(sum)
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

    Ok(winning_numbers_count as u32)
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap();

    if let Ok(sum) = points_for_scratchcards(&buffer) {
        println!("{}", sum);
    } else {
        eprintln!("Wrong format");
        process::exit(1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_points_from_scratchcards() {
        let cards = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(points_for_scratchcards(cards), Ok(30));
    }
}
