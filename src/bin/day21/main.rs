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

    // J = A
    // J = J & B
    // J = J & C
    // (J == A & B & C)
    machine.add_line("OR A J");
    machine.add_line("AND B J");
    machine.add_line("AND C J");
    // J = !J
    // J = J & D
    // (J == !(A & B & C) & D)
    machine.add_line("NOT J J");
    machine.add_line("AND D J");
    machine.add_line("WALK");

    assert_eq!(machine.read_line(), "");
    assert_eq!(machine.read_line(), "Walking...");
    assert_eq!(machine.read_line(), "");

    let output = machine.run_to_output().unwrap();
    assert!(output > 255);

    output
}

fn part2(program: &Vec<i64>) -> i64 {
    let mut machine = ProgramMachine::new(program.clone(), vec![]);
    machine.read_string();

    // If there's ground at D (=4) but a hole at E (=5) and H (=8), then we won't be able
    // to move or jump from D.
    // Jump if there is a hole at A, B or C and ground at D and ground at either E or H.
    // J = (!A | !B | !C) & D & (E | H)
    // J = !(A & B & C) & D & (E | H)

    // J = A
    // J = J & B
    // J = J & C
    // (J == A & B & C)
    machine.add_line("OR A J");
    machine.add_line("AND B J");
    machine.add_line("AND C J");
    // J = !J
    // J = J & D
    // (J == !(A & B & C) & D)
    machine.add_line("NOT J J");
    machine.add_line("AND D J");
    // T = E
    // T = T | H
    // (T == E | H)
    machine.add_line("OR E T");
    machine.add_line("OR H T");
    // J = J & T
    // (J == !(A & B & C) & D & (E | H))
    machine.add_line("AND T J");
    machine.add_line("RUN");

    assert_eq!(machine.read_line(), "");
    assert_eq!(machine.read_line(), "Running...");
    assert_eq!(machine.read_line(), "");

    let output = machine.run_to_output().unwrap();
    assert!(output > 255);

    output
}
