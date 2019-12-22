use std::str::FromStr;

use regex::Regex;

use advent_of_code_2019::input::parse_list;
use lazy_static::lazy_static;

fn main() {
    let input: Vec<Shuffle> = parse_input(include_str!("input"));
    println!("Answer to part 1: {}", part1(&input));
    println!("Answer to part 2: {}", part2(&input));
}

fn parse_input(input: &str) -> Vec<Shuffle> {
    parse_list(input, '\n')
}

#[derive(Debug)]
enum Shuffle {
    Stack,
    Cut(isize),
    Inc(usize),
}

impl FromStr for Shuffle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref CUT_RE: Regex = Regex::new(r"^cut (\-?\d+)$").unwrap();
            static ref INC_RE: Regex = Regex::new(r"^deal with increment (\d+)$").unwrap();
        }
        if s == "deal into new stack" {
            Ok(Shuffle::Stack)
        } else if let Some(captures) = CUT_RE.captures(s) {
            let n = captures.get(1).unwrap().as_str().parse().unwrap();
            Ok(Shuffle::Cut(n))
        } else if let Some(captures) = INC_RE.captures(s) {
            let n = captures.get(1).unwrap().as_str().parse().unwrap();
            Ok(Shuffle::Inc(n))
        } else {
            panic!("unexpected shuffle: {}", s);
        }
    }
}

impl Shuffle {
    fn shuffle(&self, mut deck: Vec<i32>) -> Vec<i32> {
        match self {
            &Shuffle::Stack => {
                deck.reverse();
                deck
            }
            &Shuffle::Cut(n) => {
                let cut_position = if n >= 0 {
                    n as usize
                } else {
                    deck.len() - (-n as usize)
                };
                let mut bottom = deck.split_off(cut_position);
                bottom.extend(deck);
                bottom
            }
            &Shuffle::Inc(n) => {
                let mut new_deck = deck.clone();
                let mut j: usize = 0;
                for i in 0..deck.len() {
                    new_deck[j] = deck[i];
                    j = (j + n) % deck.len();
                }
                new_deck
            }
        }
    }
}

fn shuffle_deck(shuffles: &[Shuffle], deck: Vec<i32>) -> Vec<i32> {
    shuffles
        .iter()
        .fold(deck, |deck, shuffle| shuffle.shuffle(deck))
}

fn part1(input: &Vec<Shuffle>) -> usize {
    let deck: Vec<i32> = shuffle_deck(input, (0..10_007).collect());
    deck.iter().position(|&x| x == 2019).unwrap()
}

fn part2(input: &Vec<Shuffle>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(
            shuffle_deck(&parse_input(include_str!("example1")), (0..10).collect()),
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]
        )
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(
            shuffle_deck(&parse_input(include_str!("example2")), (0..10).collect()),
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]
        )
    }

    #[test]
    fn test_part1_example3() {
        assert_eq!(
            shuffle_deck(&parse_input(include_str!("example3")), (0..10).collect()),
            vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]
        )
    }

    #[test]
    fn test_part1_example4() {
        assert_eq!(
            shuffle_deck(&parse_input(include_str!("example4")), (0..10).collect()),
            vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]
        )
    }
}
