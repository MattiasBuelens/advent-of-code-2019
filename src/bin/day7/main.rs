use std::cmp::max;
use std::ops::DerefMut;

fn main() {
    let input = parse_input();
    //    println!("Answer to part 1: {}", part1(&input));
    println!("Answer to part 2: {}", part2(&input));
}

fn parse_input() -> Vec<i32> {
    return include_str!("input")
        .trim()
        .split(',')
        .map(|x| x.parse().expect("expected number"))
        .collect();
}

enum ParameterPosition {
    Pos1,
    Pos2,
}

#[derive(Debug)]
enum InputValue {
    Position(usize),
    Immediate(i32),
}

impl InputValue {
    #[inline]
    fn parse(opcode: i32, pos: ParameterPosition, value: i32) -> InputValue {
        let mode = match pos {
            ParameterPosition::Pos1 => (opcode / 100) % 10,
            ParameterPosition::Pos2 => (opcode / 1000) % 10,
        };
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
        match opcode % 100 {
            1 => Instruction::Add(
                InputValue::parse(opcode, ParameterPosition::Pos1, program[pc + 1]),
                InputValue::parse(opcode, ParameterPosition::Pos2, program[pc + 2]),
                OutputValue(program[pc + 3] as usize),
            ),
            2 => Instruction::Multiply(
                InputValue::parse(opcode, ParameterPosition::Pos1, program[pc + 1]),
                InputValue::parse(opcode, ParameterPosition::Pos2, program[pc + 2]),
                OutputValue(program[pc + 3] as usize),
            ),
            3 => Instruction::Read(OutputValue(program[pc + 1] as usize)),
            4 => Instruction::Write(InputValue::parse(
                opcode,
                ParameterPosition::Pos1,
                program[pc + 1],
            )),
            5 => Instruction::JumpIfTrue(
                InputValue::parse(opcode, ParameterPosition::Pos1, program[pc + 1]),
                InputValue::parse(opcode, ParameterPosition::Pos2, program[pc + 2]),
            ),
            6 => Instruction::JumpIfFalse(
                InputValue::parse(opcode, ParameterPosition::Pos1, program[pc + 1]),
                InputValue::parse(opcode, ParameterPosition::Pos2, program[pc + 2]),
            ),
            7 => Instruction::LessThan(
                InputValue::parse(opcode, ParameterPosition::Pos1, program[pc + 1]),
                InputValue::parse(opcode, ParameterPosition::Pos2, program[pc + 2]),
                OutputValue(program[pc + 3] as usize),
            ),
            8 => Instruction::Equals(
                InputValue::parse(opcode, ParameterPosition::Pos1, program[pc + 1]),
                InputValue::parse(opcode, ParameterPosition::Pos2, program[pc + 2]),
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

    fn evaluate<'a>(
        &self,
        program: &mut Vec<i32>,
        input: &Vec<i32>,
        input_index: &mut usize,
    ) -> StepResult {
        match self {
            Instruction::Add(left, right, result) => {
                result.write(program, left.read(program) + right.read(program));
            }
            Instruction::Multiply(left, right, result) => {
                result.write(program, left.read(program) * right.read(program));
            }
            Instruction::Read(result) => match input.get(*input_index) {
                Some(value) => {
                    *input_index += 1;
                    result.write(program, *value);
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

trait Machine {
    fn add_input(&mut self, value: i32);
    fn step(&mut self) -> StepResult;
}

impl Machine for Box<dyn Machine> {
    fn add_input(&mut self, value: i32) {
        self.deref_mut().add_input(value)
    }

    fn step(&mut self) -> StepResult {
        self.deref_mut().step()
    }
}

struct ProgramMachine {
    program: Vec<i32>,
    pc: usize,
    input: Vec<i32>,
    input_index: usize,
}

impl ProgramMachine {
    fn new(program: Vec<i32>, input: &Vec<i32>) -> ProgramMachine {
        ProgramMachine {
            program,
            pc: 0usize,
            input: input.clone(),
            input_index: 0usize,
        }
    }

    fn run(mut self) -> Vec<i32> {
        let mut output = Vec::new();
        loop {
            match self.step() {
                StepResult::NeedInput => panic!("missing input"),
                StepResult::Output(value) => output.push(value),
                StepResult::Halt => {
                    break;
                }
                _ => {}
            };
        }
        output
    }
}

impl Machine for ProgramMachine {
    fn add_input(&mut self, value: i32) {
        self.input.push(value);
    }

    fn step(&mut self) -> StepResult {
        let instr = Instruction::parse(&self.program, self.pc);
        let result = instr.evaluate(&mut self.program, &self.input, &mut self.input_index);
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
        let input = vec![*phase_setting, signal];
        let output = ProgramMachine::new(program.clone(), &input).run();
        assert_eq!(output.len(), 1, "expected exactly one output");
        signal = output[0];
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

struct Chain<M1: Machine, M2: Machine> {
    head: M1,
    tail: M2,
}

impl<M1: Machine, M2: Machine> Chain<M1, M2> {
    fn new(head: M1, tail: M2) -> Chain<M1, M2> {
        Chain { head, tail }
    }
}

impl<M1: Machine, M2: Machine> Machine for Chain<M1, M2> {
    fn add_input(&mut self, value: i32) {
        self.head.add_input(value);
    }

    fn step(&mut self) -> StepResult {
        let head_result = self.head.step();
        let tail_result = self.tail.step();
        match tail_result {
            StepResult::Output(value) => {
                // output from tail
                tail_result
            }
            _ => match head_result {
                StepResult::Output(value) => {
                    // forward outputs from head to tail
                    self.tail.add_input(value);
                    StepResult::Ok
                }
                StepResult::Halt => {
                    // keep running tail after head has halted
                    tail_result
                }
                _ => head_result,
            },
        }
    }
}

fn make_chain(mut machines: Vec<Box<dyn Machine>>) -> Box<dyn Machine> {
    match machines.len() {
        0 => panic!("no machines"),
        1 => Box::new(machines.pop().unwrap()),
        _ => {
            let tail = machines.pop().unwrap();
            Box::new(Chain::new(make_chain(machines), tail))
        }
    }
}

fn run_feedback_loop(program: &Vec<i32>, phase_settings: &Vec<i32>) -> i32 {
    let machines: Vec<Box<dyn Machine>> = phase_settings
        .iter()
        .map(|setting| {
            let machine = ProgramMachine::new(program.clone(), &vec![*setting]);
            Box::new(machine) as Box<dyn Machine>
        })
        .collect();
    let mut chain = make_chain(machines);
    // To start the process, a 0 signal is sent to amplifier A's input exactly once.
    chain.add_input(0);
    let mut output = vec![];
    loop {
        let result = chain.step();
        match chain.step() {
            StepResult::NeedInput => {
                // keep going
            }
            StepResult::Output(value) => {
                chain.add_input(value);
                output.push(value);
            }
            StepResult::Halt => {
                break;
            }
            StepResult::Ok => {
                // keep going
            }
            StepResult::Jump(_) => panic!("cannot happen"),
        };
    }
    output.pop().unwrap()
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut max_signal = 0;
    for perm in get_permutations(&(5..=9).collect()) {
        max_signal = max(max_signal, run_feedback_loop(input, &perm));
    }
    max_signal
}
