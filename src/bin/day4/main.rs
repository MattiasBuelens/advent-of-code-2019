use std::cmp::Ordering;

fn main() {
    let start = 372037;
    let end = 905157;
    println!("Answer to part 1: {}", part1(start, end));
    println!("Answer to part 2: {}", part2(start, end));
}

fn part1(start: u32, end: u32) -> i32 {
    solve(start, end, is_password_part1)
}

fn part2(start: u32, end: u32) -> i32 {
    solve(start, end, is_password_part2)
}

fn solve<F: Fn([u8; 6]) -> bool>(start: u32, end: u32, is_password: F) -> i32 {
    let mut total_passwords = 0;
    for pass in start..=end {
        if is_password(get_digits(pass)) {
            total_passwords += 1
        }
    }
    total_passwords
}

fn get_digits(mut number: u32) -> [u8; 6] {
    let mut digits = [0u8; 6];
    for i in (0..6).rev() {
        digits[i] = (number % 10) as u8;
        number /= 10;
    }
    digits
}

fn is_password_part1(digits: [u8; 6]) -> bool {
    let mut has_same = false;
    for i in 1..digits.len() {
        match digits[i - 1].cmp(&digits[i]) {
            Ordering::Equal => {
                // Two adjacent digits are the same (like 22 in 122345).
                has_same = true;
            }
            Ordering::Less => {
                // Going from left to right, the digits never decrease;
                // they only ever increase or stay the same.
            }
            Ordering::Greater => return false,
        }
    }
    has_same
}

fn is_password_part2(digits: [u8; 6]) -> bool {
    let mut has_group_of_2_digits = false;
    let mut current_group_count: u8 = 1;
    for i in 1..digits.len() {
        match digits[i - 1].cmp(&digits[i]) {
            Ordering::Equal => {
                current_group_count += 1;
            }
            Ordering::Less => {
                if current_group_count == 2 {
                    has_group_of_2_digits = true
                }
                current_group_count = 1;
            }
            Ordering::Greater => return false,
        }
    }
    if current_group_count == 2 {
        has_group_of_2_digits = true
    }
    has_group_of_2_digits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(is_password_part1(get_digits(111111)), true);
        assert_eq!(is_password_part1(get_digits(223450)), false);
        assert_eq!(is_password_part1(get_digits(123789)), false);
    }

    #[test]
    fn test_part2() {
        assert_eq!(is_password_part2(get_digits(112233)), true);
        assert_eq!(is_password_part2(get_digits(123444)), false);
        assert_eq!(is_password_part2(get_digits(111122)), true);
    }
}
