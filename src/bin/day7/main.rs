use std::cmp::max;

use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::*;

fn main() {
    let input: Vec<i32> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&input));
    println!("Answer to part 2: {}", part2(&input));
}

fn run_chain(program: &Vec<i32>, phase_settings: &Vec<i32>) -> i32 {
    let mut signal = 0;
    for phase_setting in phase_settings {
        let mut machine = Machine::new(program.clone(), &vec![*phase_setting]);
        machine.add_input(signal);
        let output = machine.run_to_output();
        signal = output.expect("expected an output");
    }
    signal
}

fn get_permutations(values: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut perms = Vec::new();
    for i in 0..5 {
        for j in 0..5 {
            if j != i {
                for k in 0..5 {
                    if k != i && k != j {
                        for l in 0..5 {
                            if l != i && l != j && l != k {
                                for m in 0..5 {
                                    if m != i && m != j && m != k && m != l {
                                        perms.push(vec![
                                            values[i], values[j], values[k], values[l], values[m],
                                        ]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    perms
}

fn part1(input: &Vec<i32>) -> i32 {
    let mut max_signal = 0;
    for perm in get_permutations(&(0..=4).collect()) {
        max_signal = max(max_signal, run_chain(input, &perm));
    }
    max_signal
}

fn run_feedback_loop(program: &Vec<i32>, phase_settings: &Vec<i32>) -> i32 {
    let mut machines: Vec<Machine> = phase_settings
        .iter()
        .map(|setting| Machine::new(program.clone(), &vec![*setting]))
        .collect();
    // To start the process, a 0 signal is sent to amplifier A's input exactly once.
    let mut signal = 0;
    'outer: loop {
        for machine in machines.iter_mut() {
            machine.add_input(signal);
            match machine.run_to_output() {
                Some(output) => {
                    signal = output;
                }
                None => {
                    // If the first machine halts, all other machines must halt as well
                    // since they can never get a new input.
                    break 'outer;
                }
            }
        }
    }
    signal
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut max_signal = 0;
    for perm in get_permutations(&(5..=9).collect()) {
        max_signal = max(max_signal, run_feedback_loop(input, &perm));
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
