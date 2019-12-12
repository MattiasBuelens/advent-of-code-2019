pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0 {
        let old_a = a;
        a = b % a;
        b = old_a;
    }
    b.abs()
}
