use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_list<T, E>(input: &str, separator: char) -> Vec<T>
where
    T: FromStr<Err = E>,
    E: Debug,
{
    return input
        .trim()
        .split(separator)
        .map(|x| x.parse().expect("invalid input"))
        .collect();
}
