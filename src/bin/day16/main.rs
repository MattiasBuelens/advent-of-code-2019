use std::char::from_digit;
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
        self.nth(0)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        // When applying the pattern, skip the very first value exactly once.
        // (In other words, offset the whole pattern left by one.)
        self.index = (self.index + n + 1) % (self.repeat * 4);
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

fn fft_phase(input: &Vec<i32>, offset: usize) -> Vec<i32> {
    // Split the input in the middle, taking the offset into account.
    let mid = ((input.len() + offset) / 2).saturating_sub(offset);
    // Use the general approach for the left half,
    // and the optimized approach for the right half.
    // For part 2, we should *only* be using the optimized approach.
    let mut output = input.clone();
    fft_phase_left_half(&input[..], &mut output[0..mid], offset);
    fft_phase_right_half(&input[mid..], &mut output[mid..]);
    output
}

fn fft_phase_left_half(input: &[i32], output: &mut [i32], offset: usize) {
    for (i, out) in output.iter_mut().enumerate() {
        *out = input
            .iter()
            .zip(WavePattern::new(offset + i + 1).skip(offset))
            .map(|(x, y)| x * y)
            .sum::<i32>()
            .abs()
            % 10;
    }
}

fn fft_phase_right_half(input: &[i32], output: &mut [i32]) {
    // When `N/2 <= i < N`, the wave pattern degenerates into: [0, 0,... 0, 1, 1,... 1]
    // where the number of 0's equals `i` and the number of 1's equals `N - i`.
    // This means the i'th output is the sum of the i'th input and all subsequent elements.
    // We can compute these sums efficiently by starting from the last one, and working backwards.
    let mut sum = 0;
    for (i, out) in output.iter_mut().enumerate().rev() {
        sum = (sum + input[i]) % 10;
        *out = sum;
    }
}

fn fft(input: &Vec<i32>, offset: usize, phases: usize) -> Vec<i32> {
    let mut output = input.clone();
    for _ in 0..phases {
        output = fft_phase(&output, offset);
    }
    output
}

fn digits_to_string(digits: &[i32]) -> String {
    digits
        .iter()
        .map(|&x| from_digit(x as u32, 10).unwrap())
        .collect()
}

fn part1(input: &Vec<i32>) -> String {
    let output = fft(input, 0, 100);
    digits_to_string(&output[0..8])
}

fn part2(input: &Vec<i32>) -> String {
    let repeats = 10_000;
    let offset: usize = digits_to_string(&input[0..7]).parse().unwrap();

    // The i-th output only depends on input elements from i to the end.
    // Construct the repeated input from `offset` to `input.len() * repeats`.
    let input_len = input.len();
    let input: Vec<i32> = repeat(input.clone())
        .flatten()
        .skip(offset % input_len)
        .take(input_len * repeats - offset)
        .collect();

    // The offset must be between `N/2` and `N` for the optimization to work.
    assert!(offset >= (input_len * repeats / 2));
    let output = fft(&input, offset, 100);

    digits_to_string(&output[0..8])
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
        assert_eq!(
            WavePattern::new(4).skip(1).take(16).collect::<Vec<i32>>(),
            vec![0, 0, 1, 1, 1, 1, 0, 0, 0, 0, -1, -1, -1, -1, 0, 0]
        );
        assert_eq!(
            WavePattern::new(4).skip(4).take(16).collect::<Vec<i32>>(),
            vec![1, 1, 1, 0, 0, 0, 0, -1, -1, -1, -1, 0, 0, 0, 0, 1]
        );
    }

    #[test]
    fn test_part1_example1() {
        assert_eq!(
            digits_to_string(&fft(&parse_input("12345678"), 0, 4)),
            "01029498"
        );
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(
            part1(&parse_input("80871224585914546619083218645595")),
            "24176176"
        );
    }

    #[test]
    fn test_part1_example3() {
        assert_eq!(
            part1(&parse_input("19617804207202209144916044189917")),
            "73745418"
        );
    }

    #[test]
    fn test_part1_example4() {
        assert_eq!(
            part1(&parse_input("69317163492948606335995924319873")),
            "52432133"
        );
    }

    #[test]
    fn test_part2_example1() {
        assert_eq!(
            part2(&parse_input("03036732577212944063491565474664")),
            "84462026"
        );
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(
            part2(&parse_input("02935109699940807407585447034323")),
            "78725270"
        );
    }

    #[test]
    fn test_part2_example3() {
        assert_eq!(
            part2(&parse_input("03081770884921959731165446850517")),
            "53553731"
        );
    }
}
