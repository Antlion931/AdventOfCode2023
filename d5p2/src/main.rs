use std::io;
use std::io::prelude::*;

type Number = u64;

#[derive(Debug)]
struct Matcher {
    start: Number,
    end: Number,
    value: Number,
}

impl Matcher {
    fn new(to: Number, from: Number, amount: Number) -> Self {
        Self {
            start: from,
            end: from + amount,
            value: to,
        }
    }

    fn matches(&self, x: Number) -> Option<Number> {
        if x >= self.start && x < self.end {
            Some(self.value + x - self.start)
        } else {
            None
        }
    }
}

fn lowest_location_for_seed(almanac: &str) -> Option<Number> {
    let mut lines = almanac.lines();
    let mut seeds_pairs = lines
        .next()?
        .split_once(':')?
        .1
        .split_whitespace()
        .map(|s| s.parse::<Number>())
        .collect::<Result<Vec<_>, _>>()
        .ok()?
        .into_iter();

    let mut numbers_corresponding_to_seeds = vec![];

    while let (Some(from), Some(amount)) = (seeds_pairs.next(), seeds_pairs.next()) {
        for i in from..from + amount {
            numbers_corresponding_to_seeds.push(i);
        }
    }

    let mut matchers: Vec<Matcher> = vec![];

    for line in lines {
        match line {
            l if l.ends_with("map:") => {
                // use old matchers and clear them to get ready for new
                for n in numbers_corresponding_to_seeds.iter_mut() {
                    *n = matchers.iter().find_map(|m| m.matches(*n)).unwrap_or(*n);
                }

                matchers.clear();
            }
            l if l.is_empty() => {}
            l => {
                let numbers = l
                    .split_whitespace()
                    .map(|s| s.parse::<Number>())
                    .collect::<Result<Vec<_>, _>>()
                    .ok()?;

                if numbers.len() != 3 {
                    return None;
                }

                matchers.push(Matcher::new(numbers[0], numbers[1], numbers[2]));
            }
        }
    }

    for n in numbers_corresponding_to_seeds.iter_mut() {
        *n = matchers.iter().find_map(|m| m.matches(*n)).unwrap_or(*n);
    }

    Some(*numbers_corresponding_to_seeds.iter().min()?)
}

fn main() {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer).unwrap();

    println!("{:?}", lowest_location_for_seed(&buffer));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_of_possible_games() {
        let almanac = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        assert_eq!(lowest_location_for_seed(almanac), Some(46));
    }
}
