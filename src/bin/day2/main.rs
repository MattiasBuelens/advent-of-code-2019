use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::ProgramMachine;

fn main() {
    let input: Vec<i64> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&input));
    println!("Answer to part 2: {}", part2(&input));
}

fn part1(program: &Vec<i64>) -> i64 {
    run_with_noun_and_verb(program, 12, 2)
}

fn part2(program: &Vec<i64>) -> i64 {
    let (noun, verb) = solve_for(program, 19690720);
    (noun * 100) + verb
}

fn run_with_noun_and_verb(program: &Vec<i64>, noun: i64, verb: i64) -> i64 {
    let mut program = program.clone();
    program[1] = noun;
    program[2] = verb;
    let mut machine = ProgramMachine::new(program, vec![]);
    machine.run();
    machine.program()[0]
}

fn solve_for(program: &Vec<i64>, target: i64) -> (i64, i64) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_with_noun_and_verb(program, noun, verb) == target {
                return (noun, verb);
            }
        }
    }
    panic!("no solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_and_return(program: Vec<i64>) -> Vec<i64> {
        let mut machine = ProgramMachine::new(program, vec![]);
        machine.run();
        machine.program().clone()
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
