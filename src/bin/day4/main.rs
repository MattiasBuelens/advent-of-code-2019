use std::cmp::Ordering;

fn main() {
    println!("Answer to part 1: {}", part1(372037, 905157));
}

fn part1(start: u32, end: u32) -> i32 {
    let mut total_passwords = 0;
    for pass in start..=end {
        if is_password(pass) {
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

fn is_password(pass: u32) -> bool {
    let digits = get_digits(pass);
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
