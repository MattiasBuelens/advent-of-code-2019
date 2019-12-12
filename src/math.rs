pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0 {
        let old_a = a;
        a = b % a;
        b = old_a;
    }
    b.abs()
}

pub fn gcd_64(mut a: i64, mut b: i64) -> i64 {
    while a != 0 {
        let old_a = a;
        a = b % a;
        b = old_a;
    }
    b.abs()
}

pub fn lcm(a: i32, b: i32) -> i32 {
    (a * b).abs() / gcd(a, b)
}

pub fn lcm_64(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd_64(a, b)
}
