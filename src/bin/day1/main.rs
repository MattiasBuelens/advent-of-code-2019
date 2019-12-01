fn main() {
    let input = parse_input();
    part1(&input);
}

fn parse_input() -> Vec<u32> {
    return include_str!("input")
        .trim()
        .split('\n')
        .map(|x| { x.parse().expect("expected number") })
        .collect();
}

fn part1(masses: &Vec<u32>) {
    let total_fuel: u32 = masses.iter()
        .map(|x| { get_fuel(*x) })
        .sum();
    println!("Answer to part 1: {}", total_fuel);
}

fn get_fuel(mass: u32) -> u32 {
    (mass / 3) - 2
}