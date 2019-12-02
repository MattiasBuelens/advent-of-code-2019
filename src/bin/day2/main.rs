fn main() {
    let input = parse_input();
    part1(&input);
}

fn parse_input() -> Vec<i32> {
    return include_str!("input")
        .trim()
        .split(',')
        .map(|x| x.parse().expect("expected number"))
        .collect();
}

fn part1(input: &Vec<i32>) {
    let mut program = input.clone();
    program[1] = 12;
    program[2] = 2;
    run(&mut program);
    println!("Answer to part 1: {}", program[0]);
}

fn run(program: &mut Vec<i32>) {
    let mut i = 0;
    loop {
        match program[i] {
            1 => {
                // add
                let left = program[i + 1] as usize;
                let right = program[i + 2] as usize;
                let result = program[i + 3] as usize;
                program[result] = program[left] + program[right];
            }
            2 => {
                // multiply
                let left = program[i + 1] as usize;
                let right = program[i + 2] as usize;
                let result = program[i + 3] as usize;
                program[result] = program[left] * program[right];
            }
            99 => {
                // halt
                break;
            }
            _ => panic!("unexpected opcode {} at index {}", program[i], i),
        }
        // advance to next opcode
        i += 4;
    }
}
