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

fn fft_phase(input: &Vec<i32>) -> Vec<i32> {
    let mut output = input.clone();
    for i in 0..input.len() {
        let single_pattern: Vec<i32> = vec![0, 1, 0, -1]
            .into_iter()
            .flat_map(|x| repeat(x).take(i + 1))
            .collect();
        let pattern = repeat(single_pattern).into_iter().flatten().skip(1);
        output[i] = input
            .iter()
            .zip(pattern)
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
}
