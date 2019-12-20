use advent_of_code_2019::input::parse_list;

fn main() {
    let input: Vec<u32> = parse_list(include_str!("input"), '\n');
    println!("Answer to part 1: {}", part1(&input));
    println!("Answer to part 2: {}", part2(&input));
}

fn part1(masses: &Vec<u32>) -> u32 {
    masses.iter().map(|&x| get_fuel_part1(x)).sum()
}

fn get_fuel_part1(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn part2(masses: &Vec<u32>) -> u32 {
    masses.iter().map(|&x| get_fuel_part2(x)).sum()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(get_fuel_part1(12), 2);
        assert_eq!(get_fuel_part1(14), 2);
        assert_eq!(get_fuel_part1(1969), 654);
        assert_eq!(get_fuel_part1(100756), 33583);
    }

    #[test]
    fn test_part2() {
        assert_eq!(get_fuel_part2(14), 2);
        assert_eq!(get_fuel_part2(1969), 966);
        assert_eq!(get_fuel_part2(100756), 50346);
    }
}
