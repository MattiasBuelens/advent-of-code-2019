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
            &Shuffle::Cut(n) if n > 0 => {
                let mut bottom = deck.split_off(n as usize);
                bottom.extend(deck);
                bottom
            }
            &Shuffle::Cut(n) if n < 0 => {
                let mut bottom = deck.split_off(deck.len() - (-n as usize));
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
            _ => panic!("invalid shuffle: {:?}", self),
        }
    }
}

fn shuffle_deck(deck: Vec<i32>, shuffles: &[Shuffle]) -> Vec<i32> {
    shuffles
        .iter()
        .fold(deck, |deck, shuffle| shuffle.shuffle(deck))
}

fn part1(input: &Vec<Shuffle>) -> usize {
    let deck: Vec<i32> = shuffle_deck((0..10_007).collect(), input);
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
            shuffle_deck((0..10).collect(), &parse_input(include_str!("example1"))),
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]
        )
    }
}
