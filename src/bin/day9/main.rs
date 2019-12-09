use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::Machine;

fn main() {
    let input: Vec<i64> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&input));
}

fn part1(program: &Vec<i64>) -> i64 {
    let mut machine = Machine::new(program.clone(), vec![1]);
    let output = machine.run();
    assert_eq!(output.len(), 1);
    output[0]
}
