use std::cmp::max;

fn main() {
    let input = parse_input();
    println!("Answer to part 1: {}", part1(&input));
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
        input: &Vec<i32>,
        input_index: &mut usize,
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
                let value = *input.get(*input_index).expect("missing input");
                *input_index += 1;
                result.write(program, value);
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

struct Machine<'a> {
    program: Vec<i32>,
    pc: usize,
    input: &'a Vec<i32>,
    input_index: usize,
    output: Vec<i32>,
}

impl<'a> Machine<'a> {
    fn new(program: Vec<i32>, input: &'a Vec<i32>) -> Machine<'a> {
        Machine {
            program,
            pc: 0usize,
            input,
            input_index: 0usize,
            output: Vec::new(),
        }
    }

    fn run(mut self) -> Vec<i32> {
        loop {
            if self.step() {
                break;
            }
        }
        self.output
    }

    fn step(&mut self) -> bool {
        let instr = Instruction::parse(&self.program, self.pc);
        self.pc += instr.length();
        instr.evaluate(
            &mut self.program,
            &mut self.pc,
            &self.input,
            &mut self.input_index,
            &mut self.output,
        )
    }
}

fn run(program: Vec<i32>, input: &Vec<i32>) -> Vec<i32> {
    Machine::new(program, input).run()
}

fn run_chain(program: &Vec<i32>, phase_settings: &Vec<i32>) -> i32 {
    let mut signal = 0;
    for phase_setting in phase_settings {
        let input = vec![*phase_setting, signal];
        let output = run(program.clone(), &input);
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
    for perm in get_permutations(&(0..5).collect()) {
        max_signal = max(max_signal, run_chain(input, &perm));
    }
    max_signal
}
