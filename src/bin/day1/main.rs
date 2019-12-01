fn main() {
    let input = parse_input();
    part1(&input);
    part2(&input);
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
        .map(|x| { get_fuel_part1(*x) })
        .sum();
    println!("Answer to part 1: {}", total_fuel);
}

fn get_fuel_part1(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn part2(masses: &Vec<u32>) {
    let total_fuel: u32 = masses.iter()
        .map(|x| { get_fuel_part2(*x) })
        .sum();
    println!("Answer to part 2: {}", total_fuel);
}

fn get_fuel_part2(mut mass: u32) -> u32 {
    let mut total_fuel = 0u32;
    while mass > 0 {
        let fuel = get_fuel_part1(mass);
        total_fuel += fuel;
        mass = fuel;
    }
    total_fuel
}