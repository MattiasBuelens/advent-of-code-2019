use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::{Machine, ProgramMachine};

fn main() {
    let program: Vec<i64> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&program));
    println!("Answer to part 2: {}", part2(&program));
}

fn part1(program: &Vec<i64>) -> i64 {
    let mut machine = ProgramMachine::new(program.clone(), vec![]);
    machine.read_string();

    // Jump if there is a hole at A, B or C and ground at D
    // J = (!A | !B | !C) & D
    // J = !(A & B & C) & D

    // T = !!A
    // T = T & B
    // T = T & C
    // (T == A & B & C)
    machine.add_line("NOT A T");
    machine.add_line("NOT T T");
    machine.add_line("AND B T");
    machine.add_line("AND C T");
    // T = !T
    // T = T & D
    // (T == !(A & B & C) & D)
    machine.add_line("NOT T T");
    machine.add_line("AND D T");
    // T = !T
    // J = !T
    // (J == T)
    machine.add_line("NOT T T");
    machine.add_line("NOT T J");
    machine.add_line("WALK");

    assert_eq!(machine.read_line(), "");
    assert_eq!(machine.read_line(), "Walking...");
    assert_eq!(machine.read_line(), "");

    let output = machine.run_to_output().unwrap();
    assert!(output > 255);

    output
}

fn part2(program: &Vec<i64>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {}
}
