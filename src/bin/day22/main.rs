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
                let cut_position = if n >= 0 {
                    n as usize
                } else {
                    deck_length - (-n as usize)
                };
                if position < cut_position {
                    position + (deck_length - cut_position)
                } else {
                    position - cut_position
                }
            }
            &Shuffle::Inc(n) => {
                let mut j: usize = 0;
                for _ in 0..position {
                    j = (j + n) % deck_length;
                }
                j
            }
        }
    }

    fn unshuffle_pos(&self, deck_length: usize, position: usize) -> usize {
        match self {
            &Shuffle::Stack => (deck_length - 1) - position,
            &Shuffle::Cut(n) => {
                let cut_position = if n >= 0 {
                    deck_length - (n as usize)
                } else {
                    -n as usize
                };
                if position < cut_position {
                    position + (deck_length - cut_position)
                } else {
                    position - cut_position
                }
            }
            &Shuffle::Inc(n) => {
                let mut j: usize = 0;
                for i in 0..deck_length {
                    if position == j {
                        return i;
                    }
                    j = (j + n) % deck_length;
                }
                panic!("failed to unshuffle with increment {}", n);
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

fn unshuffle_pos(shuffles: &[Shuffle], deck_length: usize, position: usize) -> usize {
    shuffles.iter().rev().fold(position, |position, shuffle| {
        shuffle.unshuffle_pos(deck_length, position)
    })
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

    fn test_part1(input: &str, shuffled: Vec<i32>) {
        let shuffles = parse_input(input);
        let deck: Vec<i32> = (0..10).collect();
        assert_eq!(shuffle_deck(&shuffles, deck.clone()), shuffled);
        assert_eq!(shuffle_deck_slow(&shuffles, deck.clone()), shuffled);
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

    fn test_part2(input: &str) {
        let shuffles = parse_input(input);
        let deck: Vec<i32> = (0..10).collect();
        let shuffled = shuffle_deck(&shuffles, deck.clone());
        let unshuffled = (0..deck.len())
            .map(|i| unshuffle_pos(&shuffles, deck.len(), i) as i32)
            .collect::<Vec<_>>();
        assert_eq!(unshuffled, shuffled);
    }

    #[test]
    fn test_part2_example1() {
        test_part2(include_str!("example1"));
    }

    #[test]
    fn test_part2_example2() {
        test_part2(include_str!("example2"));
    }

    #[test]
    fn test_part2_example3() {
        test_part2(include_str!("example3"));
    }

    #[test]
    fn test_part2_example4() {
        test_part2(include_str!("example4"));
    }

    #[test]
    fn test_part2_with_part1() {
        let shuffles: Vec<Shuffle> = parse_input(include_str!("input"));
        let pos_of_2019 = part1(&shuffles);
        assert_eq!(unshuffle_pos(&shuffles, 10_007, pos_of_2019), 2019);
    }
}
