use std::io;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
struct WrongGameFormat;

impl From<std::num::ParseIntError> for WrongGameFormat {
    fn from(_: std::num::ParseIntError) -> Self {
        WrongGameFormat
    }
}

fn sum_of_powers_of_games(games: &str) -> Result<u32, WrongGameFormat> {
    let mut sum = 0;
    for game in games.lines() {
        let (r, g, b) = minimal_rgb(game)?;
        sum += r * g * b;
    }

    Ok(sum)
}

fn minimal_rgb(game: &str) -> Result<(u32, u32, u32), WrongGameFormat> {
    let game = game.split_once(':').ok_or(WrongGameFormat)?.1;

    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    for pair in game.split_terminator(&[',', ';']) {
        let mut pair = pair.split_whitespace();
        let count = pair.next().ok_or(WrongGameFormat)?.parse::<u32>()?;
        let color = pair.next().ok_or(WrongGameFormat)?;

        match color {
            "red" => r = r.max(count),
            "green" => g = g.max(count),
            "blue" => b = b.max(count),
            _ => panic!("Invalid color"),
        }
    }

    Ok((r, g, b))
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap();

    println!("{:?}", sum_of_powers_of_games(&buffer));
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

        assert_eq!(sum_of_powers_of_games(game), Ok(2286));
    }
}
