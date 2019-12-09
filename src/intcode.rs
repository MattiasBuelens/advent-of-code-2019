use std::collections::VecDeque;

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
            InputValue::Position(pos) => program[pos as usize],
            InputValue::Immediate(value) => value,
            InputValue::Relative(pos) => program[(base + pos) as usize],
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
        match *self {
            OutputValue::Position(pos) => {
                program[pos as usize] = value;
            }
            OutputValue::Relative(pos) => program[(base + pos) as usize] = value,
        }
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
    Jump(usize),
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

    fn evaluate(
        &self,
        program: &mut Vec<i64>,
        input: &mut VecDeque<i64>,
        base: &mut i64,
    ) -> StepResult {
        match self {
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
            Instruction::Read(result) => match input.pop_front() {
                Some(value) => {
                    result.write(program, *base, value);
                }
                None => return StepResult::NeedInput,
            },
            Instruction::Write(value) => {
                return StepResult::Output(value.read(program, *base));
            }
            Instruction::JumpIfTrue(test, jump) => {
                if test.read(program, *base) != 0 {
                    return StepResult::Jump(jump.read(program, *base) as usize);
                }
            }
            Instruction::JumpIfFalse(test, jump) => {
                if test.read(program, *base) == 0 {
                    return StepResult::Jump(jump.read(program, *base) as usize);
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
        }
        StepResult::Ok
    }
}

pub struct Machine {
    program: Vec<i64>,
    pc: usize,
    base: i64,
    input: VecDeque<i64>,
}

impl Machine {
    pub fn new(program: Vec<i64>, input: Vec<i64>) -> Machine {
        Machine {
            program,
            pc: 0,
            base: 0,
            input: VecDeque::from(input),
        }
    }

    pub fn program(&self) -> &Vec<i64> {
        &self.program
    }

    pub fn add_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    pub fn run(&mut self) -> Vec<i64> {
        let mut output = Vec::new();
        loop {
            match self.run_to_output() {
                Some(value) => output.push(value),
                None => break,
            };
        }
        output
    }

    pub fn run_to_output(&mut self) -> Option<i64> {
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
        let result = instr.evaluate(&mut self.program, &mut self.input, &mut self.base);
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
