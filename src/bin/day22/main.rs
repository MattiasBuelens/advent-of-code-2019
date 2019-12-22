use std::str::FromStr;

use regex::Regex;

use advent_of_code_2019::input::parse_list;
use lazy_static::lazy_static;
use modinverse::modinverse;

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
                if n >= 0 {
                    deck.rotate_left(n as usize);
                } else {
                    deck.rotate_right(-n as usize);
                }
                deck
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

    fn shuffle_slow(&self, deck: Vec<i32>) -> Vec<i32> {
        let mut new_deck = deck.clone();
        for i in 0..deck.len() {
            let new_pos = self.shuffle_pos(deck.len(), i);
            new_deck[new_pos] = deck[i];
        }
        new_deck
    }

    fn shuffle_pos(&self, deck_length: usize, position: usize) -> usize {
        match self {
            &Shuffle::Stack => (deck_length - 1) - position,
            &Shuffle::Cut(n) => {
                ((position as isize) + (deck_length as isize) - n) as usize % deck_length
            }
            &Shuffle::Inc(n) => {
                (((n as u128) * (position as u128)) % (deck_length as u128)) as usize
            }
        }
    }

    fn reverse(&self, deck_length: usize) -> Shuffle {
        match *self {
            Shuffle::Stack => Shuffle::Stack,
            Shuffle::Cut(n) => Shuffle::Cut(-n),
            Shuffle::Inc(n) => {
                Shuffle::Inc(modinverse(n as isize, deck_length as isize).unwrap() as usize)
            }
        }
    }
}

fn shuffle_deck(shuffles: &[Shuffle], deck: Vec<i32>) -> Vec<i32> {
    shuffles
        .iter()
        .fold(deck, |deck, shuffle| shuffle.shuffle(deck))
}

fn shuffle_deck_slow(shuffles: &[Shuffle], deck: Vec<i32>) -> Vec<i32> {
    shuffles
        .iter()
        .fold(deck, |deck, shuffle| shuffle.shuffle_slow(deck))
}

fn shuffle_pos(shuffles: &[Shuffle], deck_length: usize, position: usize) -> usize {
    shuffles.iter().fold(position, |position, shuffle| {
        shuffle.shuffle_pos(deck_length, position)
    })
}

fn reverse_shuffles(shuffles: &[Shuffle], deck_length: usize) -> Vec<Shuffle> {
    shuffles
        .iter()
        .rev()
        .map(|shuffle| shuffle.reverse(deck_length))
        .collect()
}

fn part1(input: &Vec<Shuffle>) -> usize {
    shuffle_pos(input, 10_007, 2019)
}

fn part2(input: &Vec<Shuffle>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_part1(input: &str, shuffled: Vec<i32>) {
        let deck: Vec<i32> = (0..10).collect();
        let shuffles = parse_input(input);
        let reversed = reverse_shuffles(&shuffles, deck.len());
        assert_eq!(shuffle_deck(&shuffles, deck.clone()), shuffled);
        assert_eq!(shuffle_deck_slow(&shuffles, deck.clone()), shuffled);
        assert_eq!(shuffle_deck(&reversed, shuffled.clone()), deck);
        assert_eq!(shuffle_deck_slow(&reversed, shuffled.clone()), deck);
    }

    #[test]
    fn test_part1_example1() {
        test_part1(include_str!("example1"), vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_part1_example2() {
        test_part1(include_str!("example2"), vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_part1_example3() {
        test_part1(include_str!("example3"), vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_part1_example4() {
        test_part1(include_str!("example4"), vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
