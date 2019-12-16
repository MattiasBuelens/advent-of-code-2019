use std::iter::*;

fn main() {
    let input: Vec<i32> = parse_input(include_str!("input"));
    println!("Answer to part 1: {}", part1(&input));
    println!("Answer to part 2: {}", part2(&input));
}

fn parse_input(input: &str) -> Vec<i32> {
    return input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).expect("invalid input") as i32)
        .collect();
}

struct WavePattern {
    index: usize,
    repeat: usize,
}

impl WavePattern {
    fn new(repeat: usize) -> WavePattern {
        WavePattern { index: 0, repeat }
    }
}

impl Iterator for WavePattern {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        // When applying the pattern, skip the very first value exactly once.
        // (In other words, offset the whole pattern left by one.)
        self.index = (self.index + 1) % (self.repeat * 4);
        // The base pattern is 0, 1, 0, -1.
        Some(match self.index / self.repeat {
            0 => 0,
            1 => 1,
            2 => 0,
            3 => -1,
            _ => panic!("cannot happen"),
        })
    }
}

fn fft_phase(input: &Vec<i32>) -> Vec<i32> {
    let mut output = input.clone();
    for i in 0..input.len() {
        output[i] = input
            .iter()
            .zip(WavePattern::new(i + 1))
            .map(|(x, y)| x * y)
            .sum::<i32>()
            .abs()
            % 10;
    }
    output
}

fn fft(input: &Vec<i32>, phases: usize) -> Vec<i32> {
    let mut output = input.clone();
    for _ in 0..phases {
        output = fft_phase(&output);
    }
    output
}

fn part1(input: &Vec<i32>) -> String {
    let output = fft(input, 100);
    output[0..8].iter().map(|x| x.to_string()).collect()
}

fn part2(input: &Vec<i32>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wave_pattern() {
        assert_eq!(
            WavePattern::new(1).take(16).collect::<Vec<i32>>(),
            vec![1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0, 1, 0, -1, 0]
        );
        assert_eq!(
            WavePattern::new(2).take(16).collect::<Vec<i32>>(),
            vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1, 0]
        );
        assert_eq!(
            WavePattern::new(4).take(16).collect::<Vec<i32>>(),
            vec![0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, -1, -1, -1, -1, 0]
        );
    }

    #[test]
    fn test_part1_example1() {
        assert_eq!(
            fft(&parse_input("12345678"), 4),
            vec![0, 1, 0, 2, 9, 4, 9, 8]
        );
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(
            fft(&parse_input("80871224585914546619083218645595"), 100)[0..8],
            vec![2, 4, 1, 7, 6, 1, 7, 6][..]
        );
    }

    #[test]
    fn test_part1_example3() {
        assert_eq!(
            fft(&parse_input("19617804207202209144916044189917"), 100)[0..8],
            vec![7, 3, 7, 4, 5, 4, 1, 8][..]
        );
    }

    #[test]
    fn test_part1_example4() {
        assert_eq!(
            fft(&parse_input("69317163492948606335995924319873"), 100)[0..8],
            vec![5, 2, 4, 3, 2, 1, 3, 3][..]
        );
    }
}
