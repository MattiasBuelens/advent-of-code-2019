use std::collections::VecDeque;
use std::ops::DerefMut;

#[derive(Debug)]
enum InputValue {
    Position(i64),
    Immediate(i64),
    Relative(i64),
}

impl InputValue {
    #[inline]
    fn parse(mode: i32, value: i64) -> InputValue {
        match mode {
            0 => InputValue::Position(value),
            1 => InputValue::Immediate(value),
            2 => InputValue::Relative(value),
            _ => panic!("unexpected parameter mode {}", mode),
        }
    }

    #[inline]
    fn read(&self, program: &Vec<i64>, base: i64) -> i64 {
        match *self {
            InputValue::Position(pos) => *program.get(pos as usize).unwrap_or(&0),
            InputValue::Immediate(value) => value,
            InputValue::Relative(pos) => *program.get((base + pos) as usize).unwrap_or(&0),
        }
    }
}

#[derive(Debug)]
enum OutputValue {
    Position(i64),
    Relative(i64),
}

impl OutputValue {
    #[inline]
    fn parse(mode: i32, value: i64) -> OutputValue {
        match mode {
            0 => OutputValue::Position(value),
            2 => OutputValue::Relative(value),
            _ => panic!("unexpected parameter mode {}", mode),
        }
    }

    #[inline]
    fn write(&self, program: &mut Vec<i64>, base: i64, value: i64) {
        let pos = match *self {
            OutputValue::Position(pos) => pos as usize,
            OutputValue::Relative(pos) => (base + pos) as usize,
        };
        if pos >= program.len() {
            program.resize(pos + 1, 0);
        }
        program[pos as usize] = value;
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
    RelativeBaseOffset(InputValue),
    Halt,
}

#[derive(Debug, Copy, Clone)]
pub enum StepResult {
    Ok,
    NeedInput,
    Output(i64),
    Halt,
}

impl Instruction {
    fn parse(program: &Vec<i64>, pc: usize) -> Instruction {
        let opcode = program[pc] as i32;
        let mode1 = (opcode / 100) % 10;
        let mode2 = (opcode / 1000) % 10;
        let mode3 = (opcode / 10000) % 10;
        match opcode % 100 {
            1 => Instruction::Add(
                InputValue::parse(mode1, program[pc + 1]),
                InputValue::parse(mode2, program[pc + 2]),
                OutputValue::parse(mode3, program[pc + 3]),
            ),
            2 => Instruction::Multiply(
                InputValue::parse(mode1, program[pc + 1]),
                InputValue::parse(mode2, program[pc + 2]),
                OutputValue::parse(mode3, program[pc + 3]),
            ),
            3 => Instruction::Read(OutputValue::parse(mode1, program[pc + 1])),
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
                OutputValue::parse(mode3, program[pc + 3]),
            ),
            8 => Instruction::Equals(
                InputValue::parse(mode1, program[pc + 1]),
                InputValue::parse(mode2, program[pc + 2]),
                OutputValue::parse(mode3, program[pc + 3]),
            ),
            9 => Instruction::RelativeBaseOffset(InputValue::parse(mode1, program[pc + 1])),
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
            Instruction::Read(_) | Instruction::Write(_) | Instruction::RelativeBaseOffset(_) => 2,
            Instruction::Halt => 1,
        }
    }
}

pub trait Machine {
    fn add_input(&mut self, value: i64);
    fn step(&mut self) -> StepResult;

    fn run(&mut self) -> Vec<i64> {
        let mut output = Vec::new();
        loop {
            match self.run_to_output() {
                Some(value) => output.push(value),
                None => break,
            };
        }
        output
    }

    fn run_to_output(&mut self) -> Option<i64> {
        loop {
            match self.step() {
                StepResult::NeedInput => panic!("missing input"),
                StepResult::Output(value) => return Some(value),
                StepResult::Halt => return None,
                StepResult::Ok => {}
            };
        }
    }

    fn add_line(&mut self, line: &str) {
        for byte in line.bytes() {
            self.add_input(byte as i64);
        }
        self.add_input('\n' as u8 as i64);
    }

    fn read_line(&mut self) -> String {
        let mut output = String::new();
        loop {
            match self.step() {
                StepResult::Ok => {}
                StepResult::Output(value) => match value as u8 as char {
                    '\n' => break,
                    _ => output.push(value as u8 as char),
                },
                StepResult::NeedInput => break,
                StepResult::Halt => panic!("unexpected halt"),
            }
        }
        output
    }

    fn read_string(&mut self) -> String {
        let mut output = String::new();
        loop {
            match self.step() {
                StepResult::Ok => {}
                StepResult::Output(value) => output.push(value as u8 as char),
                StepResult::NeedInput | StepResult::Halt => break,
            }
        }
        output
    }
}

impl<T: DerefMut<Target = dyn Machine>> Machine for T {
    fn add_input(&mut self, value: i64) {
        self.deref_mut().add_input(value)
    }

    fn step(&mut self) -> StepResult {
        self.deref_mut().step()
    }
}

pub struct ProgramMachine {
    program: Vec<i64>,
    pc: usize,
    base: i64,
    input: VecDeque<i64>,
}

impl ProgramMachine {
    pub fn new(program: Vec<i64>, input: Vec<i64>) -> ProgramMachine {
        ProgramMachine {
            program,
            pc: 0,
            base: 0,
            input: VecDeque::from(input),
        }
    }

    pub fn program(&self) -> &Vec<i64> {
        &self.program
    }
}

impl Machine for ProgramMachine {
    fn add_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    fn step(&mut self) -> StepResult {
        let program = &mut self.program;
        let base = &mut self.base;
        let instr = Instruction::parse(&program, self.pc);
        match &instr {
            Instruction::Add(left, right, result) => {
                result.write(
                    program,
                    *base,
                    left.read(program, *base) + right.read(program, *base),
                );
            }
            Instruction::Multiply(left, right, result) => {
                result.write(
                    program,
                    *base,
                    left.read(program, *base) * right.read(program, *base),
                );
            }
            Instruction::Read(result) => match self.input.pop_front() {
                Some(value) => {
                    result.write(program, *base, value);
                }
                None => return StepResult::NeedInput,
            },
            Instruction::Write(value) => {
                self.pc += instr.length();
                return StepResult::Output(value.read(program, *base));
            }
            Instruction::JumpIfTrue(test, jump) => {
                if test.read(program, *base) != 0 {
                    self.pc = jump.read(program, *base) as usize;
                    return StepResult::Ok;
                }
            }
            Instruction::JumpIfFalse(test, jump) => {
                if test.read(program, *base) == 0 {
                    self.pc = jump.read(program, *base) as usize;
                    return StepResult::Ok;
                }
            }
            Instruction::LessThan(left, right, result) => {
                let test = left.read(program, *base) < right.read(program, *base);
                result.write(program, *base, if test { 1 } else { 0 });
            }
            Instruction::Equals(left, right, result) => {
                let test = left.read(program, *base) == right.read(program, *base);
                result.write(program, *base, if test { 1 } else { 0 });
            }
            Instruction::RelativeBaseOffset(offset) => {
                *base += offset.read(program, *base);
            }
            Instruction::Halt => return StepResult::Halt,
        };
        self.pc += instr.length();
        StepResult::Ok
    }
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
    fn add_input(&mut self, value: i64) {
        self.head.add_input(value);
    }

    fn step(&mut self) -> StepResult {
        let head_result = self.head.step();
        if let StepResult::Output(value) = head_result {
            // forward outputs from head to tail
            self.tail.add_input(value);
        }
        let tail_result = self.tail.step();
        match (head_result, tail_result) {
            (_, StepResult::Output(value)) => {
                // output from tail
                StepResult::Output(value)
            }
            (_, StepResult::Halt) => {
                // if tail has halted, then head must have halted as well
                // since it can no longer output values
                StepResult::Halt
            }
            (StepResult::Halt, _) => {
                // if head has halted but tail hasn't yet,
                // continue running tail
                StepResult::Ok
            }
            (StepResult::Output(_), _) => {
                // output from head was already forwarded internally
                StepResult::Ok
            }
            (head_result, _) => head_result,
        }
    }
}

pub fn make_chain(mut machines: VecDeque<Box<dyn Machine>>) -> Box<dyn Machine> {
    let head = machines.pop_front().expect("expected at least one machine");
    if machines.is_empty() {
        head
    } else {
        Box::new(Chain::new(head, make_chain(machines)))
    }
}
