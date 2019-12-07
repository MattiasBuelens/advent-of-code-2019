use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::Machine;

fn main() {
    let input: Vec<i32> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&input));
    println!("Answer to part 2: {}", part2(&input));
}

fn part1(program: &Vec<i32>) -> i32 {
    // The TEST diagnostic program will start by requesting from the user the ID of the system to
    // test by running an input instruction - provide it 1, the ID for the ship's air conditioner unit.
    let mut output = run(program.clone(), vec![1]);

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

fn part2(program: &Vec<i32>) -> i32 {
    // This time, when the TEST diagnostic program runs its input instruction to get the ID
    // of the system to test, provide it 5, the ID for the ship's thermal radiator controller.
    let output = run(program.clone(), vec![5]);

    // This diagnostic test suite only outputs one number, the diagnostic code.
    assert_eq!(output.len(), 1);
    let answer = output[0];

    answer
}

fn run(program: Vec<i32>, input: Vec<i32>) -> Vec<i32> {
    Machine::new(program, input).run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(run(vec![1002, 4, 3, 4, 33], vec![]), vec!());
    }
}
