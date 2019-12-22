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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct MulAdd(u128, u128);

impl MulAdd {
    fn compose(self, other: MulAdd, modulo: u128) -> MulAdd {
        // c*(a*x + b) + d
        // = c*a*x + c*b + d
        let factor = (self.0 * other.0) % modulo;
        let offset = ((self.1 * other.0) + other.1) % modulo;
        MulAdd(factor, offset)
    }

    fn evaluate(&self, x: u128, modulo: u128) -> u128 {
        (((self.0 * x) % modulo) + self.1) % modulo
    }
}

impl Default for MulAdd {
    fn default() -> Self {
        MulAdd(1, 0)
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

    fn as_mul_add(&self, modulo: u128) -> MulAdd {
        match self {
            &Shuffle::Stack => {
                // i = (modulo - 1 - i) % modulo
                // i = (-1 % modulo) * i + (modulo - 1) % modulo
                // i = (modulo - 1) * i  + (modulo - 1) % modulo
                MulAdd(modulo - 1, modulo - 1)
            }
            &Shuffle::Cut(n) => {
                // i = (i - n) % modulo
                // i = (i - n + modulo) % modulo
                MulAdd(1, (((modulo as i128) - (n as i128)) as u128) % modulo)
            }
            &Shuffle::Inc(n) => {
                // i = i * n % modulo
                MulAdd((n as u128) % modulo, 0)
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

fn reverse_shuffles(shuffles: &[Shuffle], deck_length: usize) -> Vec<Shuffle> {
    shuffles
        .iter()
        .rev()
        .map(|shuffle| shuffle.reverse(deck_length))
        .collect()
}

fn combine_shuffles(shuffles: &[Shuffle], modulo: u128) -> MulAdd {
    shuffles.iter().fold(MulAdd::default(), |mul_add, shuffle| {
        mul_add.compose(shuffle.as_mul_add(modulo), modulo)
    })
}

fn part1(input: &Vec<Shuffle>) -> u128 {
    let deck_length = 10_007;
    let mul_add = combine_shuffles(input, deck_length);
    mul_add.evaluate(2019, deck_length)
}

fn part2(input: &Vec<Shuffle>) -> u128 {
    let deck_length: usize = 119_315_717_514_047;
    let modulo = deck_length as u128;

    let reversed = reverse_shuffles(&input, deck_length);
    let mul_add = combine_shuffles(&reversed, modulo);

    let mut iterations: usize = 101_741_582_076_661;
    let mut result = MulAdd::default();
    let mut power = mul_add;
    while iterations != 0 {
        if iterations % 2 != 0 {
            result = result.compose(power, modulo);
        }
        power = power.compose(power, modulo);
        iterations /= 2;
    }

    let final_pos: u128 = 2020;
    let original_pos = result.evaluate(final_pos, modulo);

    original_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shuffle_with_mul_add(shuffles: &[Shuffle], deck: Vec<i32>) -> Vec<i32> {
        let modulo = deck.len() as u128;
        let mul_add = combine_shuffles(&shuffles, modulo);
        let mut result: Vec<i32> = deck.clone();
        for i in 0..deck.len() {
            result[mul_add.evaluate(i as u128, modulo) as usize] = deck[i];
        }
        result
    }

    fn test_part1(input: &str, shuffled: Vec<i32>) {
        let deck: Vec<i32> = (0..10).collect();
        let shuffles = parse_input(input);
        let reversed = reverse_shuffles(&shuffles, deck.len());
        assert_eq!(shuffle_deck(&shuffles, deck.clone()), shuffled);
        assert_eq!(shuffle_deck(&reversed, shuffled.clone()), deck);
        assert_eq!(shuffle_with_mul_add(&shuffles, deck), shuffled);
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

    #[test]
    fn test_as_mul_add() {
        assert_eq!(Shuffle::Stack.as_mul_add(5), MulAdd(4, 4));
        assert_eq!(Shuffle::Cut(2).as_mul_add(5), MulAdd(1, 3));
        assert_eq!(Shuffle::Cut(-2).as_mul_add(5), MulAdd(1, 2));
        assert_eq!(Shuffle::Inc(2).as_mul_add(5), MulAdd(2, 0));

        let deck: Vec<i32> = (0..5).collect();
        assert_eq!(
            shuffle_with_mul_add(&[Shuffle::Stack], deck.clone()),
            vec![4, 3, 2, 1, 0]
        );
        assert_eq!(
            shuffle_with_mul_add(&[Shuffle::Cut(2)], deck.clone()),
            vec![2, 3, 4, 0, 1]
        );
        assert_eq!(
            shuffle_with_mul_add(&[Shuffle::Cut(-2)], deck.clone()),
            vec![3, 4, 0, 1, 2]
        );
        assert_eq!(
            shuffle_with_mul_add(&[Shuffle::Inc(1)], deck.clone()),
            vec![0, 1, 2, 3, 4]
        );
        assert_eq!(
            shuffle_with_mul_add(&[Shuffle::Inc(2)], deck.clone()),
            vec![0, 3, 1, 4, 2]
        );
        assert_eq!(
            shuffle_with_mul_add(&[Shuffle::Inc(3)], deck.clone()),
            vec![0, 2, 4, 1, 3]
        );
    }

    #[test]
    fn test_mul_add_compose() {
        assert_eq!(MulAdd(1, 0).compose(MulAdd(2, 3), 7), MulAdd(2, 3));
        assert_eq!(MulAdd(2, 3).compose(MulAdd(1, 0), 7), MulAdd(2, 3));
        assert_eq!(MulAdd(5, 6).compose(MulAdd(2, 3), 7), MulAdd(3, 1));
    }

    #[test]
    fn test_combine_shuffles() {
        let shuffles = vec![
            Shuffle::Stack,
            Shuffle::Cut(1),
            Shuffle::Inc(3),
            Shuffle::Stack,
            Shuffle::Cut(2),
            Shuffle::Inc(2),
        ];
        let deck: Vec<i32> = (0..5).collect();
        let shuffled = shuffle_deck(&shuffles, deck.clone());
        assert_eq!(shuffle_with_mul_add(&shuffles, deck), shuffled);
    }
}
