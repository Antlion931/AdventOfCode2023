use std::io;
use std::io::prelude::*;

fn make_matrix_of_chars(lines: &str) -> Vec<Vec<char>> {
    lines.lines().map(|line| line.chars().collect()).collect()
}

fn cut_number_from(lines: &mut [Vec<char>], x: usize, y: usize) -> Option<u32> {
    if !lines[y][x].is_ascii_digit() {
        return None;
    }

    let mut x_s = x;
    let mut x_e = x;

    while x_s > 0 && lines[y][x_s - 1].is_ascii_digit() {
        x_s -= 1;
    }

    while x_e + 1 < lines[y].len() && lines[y][x_e + 1].is_ascii_digit() {
        x_e += 1;
    }

    let mut result = 0;

    for x in x_s..=x_e {
        let digit = lines[y][x]
            .to_digit(10)
            .expect("Every char between x_s and x_e is a digit");
        lines[y][x] = '.';
        result = result * 10 + digit;
    }

    Some(result)
}

fn sum_of_number_parts(engine: &str) -> u32 {
    let mut lines = make_matrix_of_chars(engine);

    let mut sum = 0;

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y][x] == '*' {
                let x_s = x.saturating_sub(1);
                let x_e = (x + 1).min(lines[y].len() - 1);

                let y_s = y.saturating_sub(1);
                let y_e = (y + 1).min(lines.len() - 1);

                let mut numbers = Vec::new();

                for y in y_s..=y_e {
                    for x in x_s..=x_e {
                        if let Some(number) = cut_number_from(&mut lines, x, y) {
                            numbers.push(number);
                        }
                    }
                }

                if numbers.len() == 2 {
                    sum += numbers[0] * numbers[1];
                }
            }
        }
    }

    sum
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap();

    println!("{:?}", sum_of_number_parts(&buffer));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let game = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(sum_of_number_parts(game), 467835);
    }

    #[test]
    fn test_cut_from_simple() {
        let mut matrix = make_matrix_of_chars("...\n.1.\n...");
        assert_eq!(cut_number_from(&mut matrix, 1, 1), Some(1));
        assert_eq!(matrix, make_matrix_of_chars("...\n...\n..."));
    }

    #[test]
    fn test_cut_from_complex() {
        let mut matrix = make_matrix_of_chars("...\n.12\n...");
        assert_eq!(cut_number_from(&mut matrix, 1, 1), Some(12));
        assert_eq!(matrix, make_matrix_of_chars("...\n...\n..."));
    }

    #[test]
    fn test_cut_dont_cut() {
        let mut matrix = make_matrix_of_chars("...\n.1.\n...");
        assert_eq!(cut_number_from(&mut matrix, 0, 1), None);
        assert_eq!(matrix, make_matrix_of_chars("...\n.1.\n..."));
    }
}
