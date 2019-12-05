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

fn run(program: &mut Vec<i32>, input: &Vec<i32>) -> Vec<i32> {
    let mut input_iter = input.iter();
    let mut output = Vec::new();
    let mut pc = 0; // program counter
    loop {
        let instr = program[pc];
        let opcode = instr % 100;
        match opcode {
            1 => {
                // add
                let l = program[pc + 1];
                let r = program[pc + 2];
                let res = program[pc + 3];
                let l_imm = (instr / 100) % 10 == 1;
                let r_imm = (instr / 1000) % 10 == 1;
                let l_value = if l_imm { l as i32 } else { program[l as usize] };
                let r_value = if r_imm { r as i32 } else { program[r as usize] };
                program[res as usize] = l_value + r_value;
                pc += 4;
            }
            2 => {
                // multiply
                let l = program[pc + 1];
                let r = program[pc + 2];
                let res = program[pc + 3];
                let l_imm = (instr / 100) % 10 == 1;
                let r_imm = (instr / 1000) % 10 == 1;
                let l_value = if l_imm { l } else { program[l as usize] };
                let r_value = if r_imm { r } else { program[r as usize] };
                program[res as usize] = l_value * r_value;
                pc += 4;
            }
            3 => {
                // read input
                let res = program[pc + 1];
                let input_value = *input_iter.next().expect("missing input");
                program[res as usize] = input_value;
                pc += 2;
            }
            4 => {
                // write output
                let o = program[pc + 1];
                let o_imm = (instr / 100) % 10 == 1;
                let o_value = if o_imm { o as i32 } else { program[o as usize] };
                output.push(o_value);
                pc += 2;
            }
            99 => {
                // halt
                break;
            }
            _ => panic!("unexpected opcode {} at index {}", opcode, pc),
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
