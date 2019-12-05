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

fn part1(input: &Vec<i32>) -> i32 {
    let mut program = input.clone();

    // The TEST diagnostic program will start by requesting from the user the ID of the system to
    // test by running an input instruction - provide it 1, the ID for the ship's air conditioner unit.
    let input = vec![1];
    let mut output = run(&mut program, &input);

    let last_output = output.pop();

    // For each test, it will run an output instruction indicating how far the result of the test
    // was from the expected value, where 0 means the test was successful.
    assert!(
        output.iter().all(|x| *x == 0),
        "all outputs except the last one should be 0"
    );

    // Finally, the program will output a diagnostic code and immediately halt.
    let answer = last_output.expect("expected at least one output");
    answer
}

fn part2(input: &Vec<i32>) -> i32 {
    let mut program = input.clone();

    // This time, when the TEST diagnostic program runs its input instruction to get the ID
    // of the system to test, provide it 5, the ID for the ship's thermal radiator controller.
    let input = vec![5];
    let output = run(&mut program, &input);

    // This diagnostic test suite only outputs one number, the diagnostic code.
    assert_eq!(output.len(), 1);
    let answer = output[0];

    answer
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
    fn parse(program: &Vec<i32>, pc: &mut usize) -> Instruction {
        let opcode = program[*pc];
        let instruction = match opcode % 100 {
            1 => Instruction::Add(
                InputValue::parse(opcode, ParameterPosition::Pos1, program[*pc + 1]),
                InputValue::parse(opcode, ParameterPosition::Pos2, program[*pc + 2]),
                OutputValue(program[*pc + 3] as usize),
            ),
            2 => Instruction::Multiply(
                InputValue::parse(opcode, ParameterPosition::Pos1, program[*pc + 1]),
                InputValue::parse(opcode, ParameterPosition::Pos2, program[*pc + 2]),
                OutputValue(program[*pc + 3] as usize),
            ),
            3 => Instruction::Read(OutputValue(program[*pc + 1] as usize)),
            4 => Instruction::Write(InputValue::parse(
                opcode,
                ParameterPosition::Pos1,
                program[*pc + 1],
            )),
            5 => Instruction::JumpIfTrue(
                InputValue::parse(opcode, ParameterPosition::Pos1, program[*pc + 1]),
                InputValue::parse(opcode, ParameterPosition::Pos2, program[*pc + 2]),
            ),
            6 => Instruction::JumpIfFalse(
                InputValue::parse(opcode, ParameterPosition::Pos1, program[*pc + 1]),
                InputValue::parse(opcode, ParameterPosition::Pos2, program[*pc + 2]),
            ),
            7 => Instruction::LessThan(
                InputValue::parse(opcode, ParameterPosition::Pos1, program[*pc + 1]),
                InputValue::parse(opcode, ParameterPosition::Pos2, program[*pc + 2]),
                OutputValue(program[*pc + 3] as usize),
            ),
            8 => Instruction::Equals(
                InputValue::parse(opcode, ParameterPosition::Pos1, program[*pc + 1]),
                InputValue::parse(opcode, ParameterPosition::Pos2, program[*pc + 2]),
                OutputValue(program[*pc + 3] as usize),
            ),
            99 => Instruction::Halt,
            _ => panic!("unexpected opcode {} at index {}", opcode, *pc),
        };
        *pc += instruction.length();
        instruction
    }

    #[inline]
    fn length(&self) -> usize {
        match *self {
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
                result.write(program, left.read(&program) + right.read(&program));
            }
            Instruction::Multiply(left, right, result) => {
                result.write(program, left.read(&program) * right.read(&program));
            }
            Instruction::Read(result) => {
                result.write(program, *input.next().expect("missing input"));
            }
            Instruction::Write(value) => {
                output.push(value.read(&program));
            }
            Instruction::JumpIfTrue(test, jump) => {
                if test.read(&program) != 0 {
                    *pc = jump.read(&program) as usize;
                }
            }
            Instruction::JumpIfFalse(test, jump) => {
                if test.read(&program) == 0 {
                    *pc = jump.read(&program) as usize;
                }
            }
            Instruction::LessThan(left, right, result) => {
                let test = left.read(&program) < right.read(program);
                result.write(program, if test { 1 } else { 0 });
            }
            Instruction::Equals(left, right, result) => {
                let test = left.read(&program) == right.read(program);
                result.write(program, if test { 1 } else { 0 });
            }
            Instruction::Halt => return true,
        }
        false
    }
}

fn run(program: &mut Vec<i32>, input: &Vec<i32>) -> Vec<i32> {
    let mut input_iter = input.iter();
    let mut output = Vec::new();
    let mut pc = 0usize; // program counter
    loop {
        let instr = Instruction::parse(program, &mut pc);
        if instr.evaluate(program, &mut pc, &mut input_iter, &mut output) {
            break;
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(run(&mut vec![1002, 4, 3, 4, 33], &vec![]), vec!());
    }
}
