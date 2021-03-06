use std::cmp::max;
use std::collections::VecDeque;

use permutohedron::Heap;

use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::*;

fn main() {
    let input: Vec<i64> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&input));
    println!("Answer to part 2: {}", part2(&input));
}

fn run_chain(program: &Vec<i64>, phase_settings: &Vec<i64>) -> i64 {
    let mut signal = 0;
    for &phase_setting in phase_settings {
        let mut machine = ProgramMachine::new(program.clone(), vec![phase_setting]);
        machine.add_input(signal);
        let output = machine.run_to_output();
        signal = output.expect("expected an output");
    }
    signal
}

fn part1(program: &Vec<i64>) -> i64 {
    let mut max_signal = 0;
    let mut settings: Vec<i64> = (0..=4).collect();
    for permutation in Heap::new(&mut settings) {
        max_signal = max(max_signal, run_chain(program, &permutation));
    }
    max_signal
}

fn run_feedback_loop(program: &Vec<i64>, phase_settings: &Vec<i64>) -> i64 {
    let machines: Vec<Box<dyn Machine>> = phase_settings
        .iter()
        .map(|&setting| {
            let machine = ProgramMachine::new(program.clone(), vec![setting]);
            Box::new(machine) as Box<dyn Machine>
        })
        .collect();
    let mut chain = make_chain(VecDeque::from(machines));

    // To start the process, a 0 signal is sent to amplifier A's input exactly once.
    let mut signal = 0;
    chain.add_input(signal);

    loop {
        match chain.step() {
            StepResult::Ok | StepResult::NeedInput => {
                // keep going
            }
            StepResult::Output(value) => {
                chain.add_input(value);
                signal = value;
            }
            StepResult::Halt => {
                break;
            }
        };
    }

    signal
}

fn part2(program: &Vec<i64>) -> i64 {
    let mut max_signal = 0;
    let mut settings: Vec<i64> = (5..=9).collect();
    for permutation in Heap::new(&mut settings) {
        max_signal = max(max_signal, run_feedback_loop(program, &permutation));
    }
    max_signal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            ]),
            43210
        );
        assert_eq!(
            part1(&vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ]),
            54321
        );
        assert_eq!(
            part1(&vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ]),
            65210
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ]),
            139629729
        );
        assert_eq!(
            part2(&vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ]),
            18216
        );
    }
}
