use std::io;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
struct WrongGameFormat;

impl From<std::num::ParseIntError> for WrongGameFormat {
    fn from(_: std::num::ParseIntError) -> Self {
        WrongGameFormat
    }
}

fn sum_of_possible_games(games: &str) -> Result<usize, WrongGameFormat> {
    let mut sum = 0;
    for (n, game) in games.lines().enumerate() {
        if is_game_possible(game)? {
            sum += n + 1;
        }
    }

    Ok(sum)
}

fn is_game_possible(game: &str) -> Result<bool, WrongGameFormat> {
    let game = game.split_once(':').ok_or(WrongGameFormat)?.1;

    for pair in game.split_terminator(&[',', ';']) {
        let mut pair = pair.split_whitespace();
        let count = pair.next().ok_or(WrongGameFormat)?.parse::<u32>()?;
        let color = pair.next().ok_or(WrongGameFormat)?;

        let possible = match color {
            "red" => count <= 12,
            "green" => count <= 13,
            "blue" => count <= 14,
            _ => panic!("Invalid color"),
        };

        if !possible {
            return Ok(false);
        }
    }

    Ok(true)
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap();

    println!("{:?}", sum_of_possible_games(&buffer));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_possible_games() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(sum_of_possible_games(game), Ok(8));
    }
}
