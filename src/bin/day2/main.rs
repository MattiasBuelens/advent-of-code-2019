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
    program[1] = 12;
    program[2] = 2;
    run(&mut program);
    program[0]
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

fn part2(input: &Vec<i32>) -> i32 {
    let (noun, verb) = solve_for(input, 19690720);
    (noun * 100) + verb
}

fn solve_for(input: &Vec<i32>, target: i32) -> (i32, i32) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = input.clone();
            program[1] = noun;
            program[2] = verb;
            run(&mut program);
            if program[0] == target {
                return (noun, verb);
            }
        }
    }
    panic!("no solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_and_return(mut input: Vec<i32>) -> Vec<i32> {
        run(&mut input);
        input
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            run_and_return(vec!(1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50)),
            vec!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50)
        );
        assert_eq!(run_and_return(vec!(1, 0, 0, 0, 99)), vec!(2, 0, 0, 0, 99));
        assert_eq!(run_and_return(vec!(2, 3, 0, 3, 99)), vec!(2, 3, 0, 6, 99));
        assert_eq!(
            run_and_return(vec!(2, 4, 4, 5, 99, 0)),
            vec!(2, 4, 4, 5, 99, 9801)
        );
        assert_eq!(
            run_and_return(vec!(1, 1, 1, 4, 99, 5, 6, 0, 99)),
            vec!(30, 1, 1, 4, 2, 5, 6, 0, 99)
        );
    }
}
