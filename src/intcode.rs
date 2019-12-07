use std::collections::VecDeque;

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
pub enum StepResult {
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

pub struct Machine {
    program: Vec<i32>,
    pc: usize,
    input: VecDeque<i32>,
}

impl Machine {
    pub fn new(program: Vec<i32>, input: &Vec<i32>) -> Machine {
        Machine {
            program,
            pc: 0usize,
            input: VecDeque::from(input.clone()),
        }
    }

    pub fn program(&self) -> &Vec<i32> {
        &self.program
    }

    pub fn add_input(&mut self, input: i32) {
        self.input.push_back(input);
    }

    pub fn run(&mut self) -> Vec<i32> {
        let mut output = Vec::new();
        loop {
            match self.run_to_output() {
                Some(value) => output.push(value),
                None => break,
            };
        }
        output
    }

    pub fn run_to_output(&mut self) -> Option<i32> {
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
