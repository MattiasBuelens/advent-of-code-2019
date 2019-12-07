use std::cmp::max;
use std::collections::VecDeque;

fn main() {
    let input = parse_input();
    println!("Answer to part 1: {}", part1(&input));
    println!("Answer to part 2: {}", part2(&input));
}

fn parse_input() -> Vec<i32> {
    return include_str!("input")
        .trim()
        .split(',')
        .map(|x| x.parse().expect("expected number"))
        .collect();
}

#[derive(Debug)]
enum InputValue {
    Position(usize),
    Immediate(i32),
}

impl InputValue {
    #[inline]
    fn parse(mode: i32, value: i32) -> InputValue {
        match mode {
            0 => InputValue::Position(value as usize),
            1 => InputValue::Immediate(value),
            _ => panic!("unexpected parameter mode {}", mode),
        }
    }

    #[inline]
    fn read(&self, program: &Vec<i32>) -> i32 {
        match *self {
            InputValue::Position(pos) => program[pos],
            InputValue::Immediate(value) => value,
        }
    }
}

#[derive(Debug)]
struct OutputValue(usize);

impl OutputValue {
    #[inline]
    fn write(&self, program: &mut Vec<i32>, value: i32) {
        program[self.0] = value;
    }
}

#[derive(Debug)]
enum Instruction {
    Add(InputValue, InputValue, OutputValue),
    Multiply(InputValue, InputValue, OutputValue),
    Read(OutputValue),
    Write(InputValue),
    JumpIfTrue(InputValue, InputValue),
    JumpIfFalse(InputValue, InputValue),
    LessThan(InputValue, InputValue, OutputValue),
    Equals(InputValue, InputValue, OutputValue),
    Halt,
}

#[derive(Debug, Copy, Clone)]
enum StepResult {
    Ok,
    NeedInput,
    Jump(usize),
    Output(i32),
    Halt,
}

impl Instruction {
    fn parse(program: &Vec<i32>, pc: usize) -> Instruction {
        let opcode = program[pc];
        let mode1 = (opcode / 100) % 10;
        let mode2 = (opcode / 1000) % 10;
        match opcode % 100 {
            1 => Instruction::Add(
                InputValue::parse(mode1, program[pc + 1]),
                InputValue::parse(mode2, program[pc + 2]),
                OutputValue(program[pc + 3] as usize),
            ),
            2 => Instruction::Multiply(
                InputValue::parse(mode1, program[pc + 1]),
                InputValue::parse(mode2, program[pc + 2]),
                OutputValue(program[pc + 3] as usize),
            ),
            3 => Instruction::Read(OutputValue(program[pc + 1] as usize)),
            4 => Instruction::Write(InputValue::parse(mode1, program[pc + 1])),
            5 => Instruction::JumpIfTrue(
                InputValue::parse(mode1, program[pc + 1]),
                InputValue::parse(mode2, program[pc + 2]),
            ),
            6 => Instruction::JumpIfFalse(
                InputValue::parse(mode1, program[pc + 1]),
                InputValue::parse(mode2, program[pc + 2]),
            ),
            7 => Instruction::LessThan(
                InputValue::parse(mode1, program[pc + 1]),
                InputValue::parse(mode2, program[pc + 2]),
                OutputValue(program[pc + 3] as usize),
            ),
            8 => Instruction::Equals(
                InputValue::parse(mode1, program[pc + 1]),
                InputValue::parse(mode2, program[pc + 2]),
                OutputValue(program[pc + 3] as usize),
            ),
            99 => Instruction::Halt,
            _ => panic!("unexpected opcode {} at index {}", opcode, pc),
        }
    }

    #[inline]
    fn length(&self) -> usize {
        match self {
            Instruction::Add(_, _, _)
            | Instruction::Multiply(_, _, _)
            | Instruction::LessThan(_, _, _)
            | Instruction::Equals(_, _, _) => 4,
            Instruction::JumpIfTrue(_, _) | Instruction::JumpIfFalse(_, _) => 3,
            Instruction::Read(_) | Instruction::Write(_) => 2,
            Instruction::Halt => 1,
        }
    }

    fn evaluate(&self, program: &mut Vec<i32>, input: &mut VecDeque<i32>) -> StepResult {
        match self {
            Instruction::Add(left, right, result) => {
                result.write(program, left.read(program) + right.read(program));
            }
            Instruction::Multiply(left, right, result) => {
                result.write(program, left.read(program) * right.read(program));
            }
            Instruction::Read(result) => match input.pop_front() {
                Some(value) => {
                    result.write(program, value);
                }
                None => return StepResult::NeedInput,
            },
            Instruction::Write(value) => {
                return StepResult::Output(value.read(program));
            }
            Instruction::JumpIfTrue(test, jump) => {
                if test.read(program) != 0 {
                    return StepResult::Jump(jump.read(program) as usize);
                }
            }
            Instruction::JumpIfFalse(test, jump) => {
                if test.read(program) == 0 {
                    return StepResult::Jump(jump.read(program) as usize);
                }
            }
            Instruction::LessThan(left, right, result) => {
                let test = left.read(program) < right.read(program);
                result.write(program, if test { 1 } else { 0 });
            }
            Instruction::Equals(left, right, result) => {
                let test = left.read(program) == right.read(program);
                result.write(program, if test { 1 } else { 0 });
            }
            Instruction::Halt => return StepResult::Halt,
        }
        StepResult::Ok
    }
}

struct Machine {
    program: Vec<i32>,
    pc: usize,
    input: VecDeque<i32>,
}

impl Machine {
    fn new(program: Vec<i32>, input: &Vec<i32>) -> Machine {
        Machine {
            program,
            pc: 0usize,
            input: VecDeque::from(input.clone()),
        }
    }

    fn run_to_output(&mut self, input: i32) -> Option<i32> {
        self.input.push_back(input);
        loop {
            match self.step() {
                StepResult::NeedInput => panic!("missing input"),
                StepResult::Output(value) => return Some(value),
                StepResult::Halt => return None,
                _ => {}
            };
        }
    }

    fn step(&mut self) -> StepResult {
        let instr = Instruction::parse(&self.program, self.pc);
        let result = instr.evaluate(&mut self.program, &mut self.input);
        match result {
            StepResult::Ok | StepResult::Output(_) => {
                self.pc += instr.length();
                result
            }
            StepResult::Jump(jump) => {
                self.pc = jump;
                StepResult::Ok
            }
            StepResult::NeedInput | StepResult::Halt => {
                // program is paused, do not increment program counter
                result
            }
        }
    }
}

fn run_chain(program: &Vec<i32>, phase_settings: &Vec<i32>) -> i32 {
    let mut signal = 0;
    for phase_setting in phase_settings {
        let mut machine = Machine::new(program.clone(), &vec![*phase_setting]);
        let output = machine.run_to_output(signal);
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
            match machine.run_to_output(signal) {
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
