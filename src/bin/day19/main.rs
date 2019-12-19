use std::collections::HashSet;

use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::{Machine, ProgramMachine};
use advent_of_code_2019::vector2d::Vector2D;

fn main() {
    let program: Vec<i64> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&program));
    println!("Answer to part 2: {}", part2(&program));
}

fn part1(input: &Vec<i64>) -> usize {
    let mut map: HashSet<Vector2D> = HashSet::new();
    for y in 0..50 {
        for x in 0..50 {
            let mut machine = ProgramMachine::new(input.clone(), vec![]);
            machine.add_input(x as i64);
            machine.add_input(y as i64);
            match machine.run_to_output() {
                Some(1) => {
                    map.insert(Vector2D::new(x, y));
                }
                Some(0) => {}
                output => panic!("unexpected output {:?}", output),
            };
        }
    }
    map.len()
}

fn part2(input: &Vec<i64>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {}
}
