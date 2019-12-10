use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::*;

fn main() {
    let input: Vec<i64> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&input));
    println!("Answer to part 2: {}", part2(&input));
}

fn part1(program: &Vec<i64>) -> i64 {
    let mut machine = ProgramMachine::new(program.clone(), vec![1]);
    let output = machine.run();
    assert_eq!(output.len(), 1);
    output[0]
}

fn part2(program: &Vec<i64>) -> i64 {
    let mut machine = ProgramMachine::new(program.clone(), vec![2]);
    let output = machine.run();
    assert_eq!(output.len(), 1);
    output[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quine() {
        let program = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let output = ProgramMachine::new(program.clone(), vec![]).run();
        assert_eq!(output, program);
    }

    #[test]
    fn test_16_digit() {
        let program = vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        let output = ProgramMachine::new(program, vec![]).run();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0].to_string().len(), 16);
    }

    #[test]
    fn test_large_number() {
        let large_number = 1125899906842624;
        let program = vec![104, large_number, 99];
        let output = ProgramMachine::new(program, vec![]).run();
        assert_eq!(output, vec![large_number]);
    }
}
