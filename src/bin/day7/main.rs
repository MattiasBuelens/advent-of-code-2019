fn main() {
    let input = parse_input();
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

struct OutputValue(usize);

impl OutputValue {
    #[inline]
    fn write(&self, program: &mut Vec<i32>, value: i32) {
        program[self.0] = value;
    }
}

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
        pc: &mut usize,
        input: &mut impl Iterator<Item = &'a i32>,
        output: &mut Vec<i32>,
    ) -> bool {
        match self {
            Instruction::Add(left, right, result) => {
                result.write(program, left.read(program) + right.read(program));
            }
            Instruction::Multiply(left, right, result) => {
                result.write(program, left.read(program) * right.read(program));
            }
            Instruction::Read(result) => {
                result.write(program, *input.next().expect("missing input"));
            }
            Instruction::Write(value) => {
                output.push(value.read(program));
            }
            Instruction::JumpIfTrue(test, jump) => {
                if test.read(program) != 0 {
                    *pc = jump.read(program) as usize;
                }
            }
            Instruction::JumpIfFalse(test, jump) => {
                if test.read(program) == 0 {
                    *pc = jump.read(program) as usize;
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
            Instruction::Halt => return true,
        }
        false
    }
}

fn run(program: &mut Vec<i32>, input: &Vec<i32>) -> Vec<i32> {
    let mut input = input.iter();
    let mut output = Vec::new();
    let mut pc = 0usize; // program counter
    loop {
        let instr = Instruction::parse(program, pc);
        pc += instr.length();
        if instr.evaluate(program, &mut pc, &mut input, &mut output) {
            break;
        }
    }
    output
}
