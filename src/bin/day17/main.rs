use std::collections::HashMap;

use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::{Machine, ProgramMachine, StepResult};
use advent_of_code_2019::vector2d::Vector2D;

fn main() {
    let program: Vec<i64> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&program));
    println!("Answer to part 2: {}", part2(&program));
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(value: char) -> Direction {
        match value {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("invalid direction"),
        }
    }

    fn step(&self) -> Vector2D {
        match *self {
            Direction::Up => Vector2D { x: 0, y: -1 },
            Direction::Down => Vector2D { x: 0, y: 1 },
            Direction::Left => Vector2D { x: -1, y: 0 },
            Direction::Right => Vector2D { x: 1, y: 0 },
        }
    }

    fn rotate_left(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn rotate_right(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn print(&self) -> char {
        match *self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Scaffold,
    Open,
    Robot(Direction),
}

impl Tile {
    fn parse(value: char) -> Tile {
        match value {
            '#' => Tile::Scaffold,
            '.' => Tile::Open,
            '^' | 'v' | '<' | '>' => Tile::Robot(Direction::parse(value)),
            'X' => panic!("robot fell off the scaffold"),
            _ => panic!("invalid tile"),
        }
    }

    fn print(&self) -> char {
        match *self {
            Tile::Scaffold => '#',
            Tile::Open => '.',
            Tile::Robot(dir) => dir.print(),
        }
    }
}

type Grid = HashMap<Vector2D, Tile>;

fn part1(program: &Vec<i64>) -> i32 {
    let mut machine = ProgramMachine::new(program.clone(), vec![]);
    let grid: Grid = read_grid(&mut machine);
    intersection_alignment(&grid)
}

fn read_grid(machine: &mut ProgramMachine) -> Grid {
    let mut grid: Grid = HashMap::new();
    let mut y = 0;
    let mut x = 0;
    let mut halt_on_next_newline = false;
    loop {
        match machine.run_to_output() {
            Some(value) => match value as u8 as char {
                '\n' => {
                    if halt_on_next_newline {
                        break;
                    }
                    halt_on_next_newline = true;
                    x = 0;
                    y += 1;
                }
                _ => {
                    halt_on_next_newline = false;
                    grid.insert(Vector2D::new(x, y), Tile::parse(value as u8 as char));
                    x += 1;
                }
            },
            None => panic!("unexpected halt"),
        };
    }
    grid
}

fn parse_grid(s: &str) -> Grid {
    let mut grid: Grid = HashMap::new();
    let mut y = 0;
    let mut x = 0;
    for value in s.chars() {
        match value {
            '\n' => {
                x = 0;
                y += 1;
            }
            _ => {
                grid.insert(Vector2D::new(x, y), Tile::parse(value as u8 as char));
                x += 1;
            }
        };
    }
    grid
}

fn intersection_alignment(grid: &Grid) -> i32 {
    find_intersections(grid)
        .iter()
        .map(|pos| pos.x * pos.y)
        .sum()
}

fn find_intersections(grid: &Grid) -> Vec<Vector2D> {
    let mut result = Vec::new();
    for (pos, _) in grid {
        if is_intersection(grid, *pos) {
            result.push(*pos);
        }
    }
    result
}

fn is_intersection(grid: &Grid, pos: Vector2D) -> bool {
    grid.get(&pos) == Some(&Tile::Scaffold)
        && grid.get(&(pos + Direction::Up.step())) == Some(&Tile::Scaffold)
        && grid.get(&(pos + Direction::Down.step())) == Some(&Tile::Scaffold)
        && grid.get(&(pos + Direction::Left.step())) == Some(&Tile::Scaffold)
        && grid.get(&(pos + Direction::Right.step())) == Some(&Tile::Scaffold)
}

fn print_grid(grid: &Grid) {
    let min_x = grid.keys().min_by_key(|pos| pos.x).unwrap().x;
    let min_y = grid.keys().min_by_key(|pos| pos.y).unwrap().y;
    let max_x = grid.keys().max_by_key(|pos| pos.x).unwrap().x;
    let max_y = grid.keys().max_by_key(|pos| pos.y).unwrap().y;
    for y in min_y..=max_y {
        let mut line = String::new();
        for x in min_x..=max_x {
            let pos = Vector2D::new(x, y);
            line.push(match grid.get(&pos) {
                Some(tile) => tile.print(),
                None => '?',
            });
        }
        println!("{}", line);
    }
}

fn part2(program: &Vec<i64>) -> i64 {
    let mut program = program.clone();
    program[0] = 2;
    let mut machine = ProgramMachine::new(program, vec![]);

    // start grid
    let grid: Grid = read_grid(&mut machine);
    // print_grid(&grid);

    // compute the path
    let path = trace_path(&grid);

    // find functions to cover the path
    let functions = find_functions(&path).expect("failed to find functions");
    let main = build_main(&path, &functions);

    // fill in the prompts
    expect_prompt(&mut machine, "Main:\n");
    input_string(&mut machine, &main);
    expect_prompt(&mut machine, "Function A:\n");
    input_string(&mut machine, &commands_to_string(&functions.a));
    expect_prompt(&mut machine, "Function B:\n");
    input_string(&mut machine, &commands_to_string(&functions.b));
    expect_prompt(&mut machine, "Function C:\n");
    input_string(&mut machine, &commands_to_string(&functions.c));
    expect_prompt(&mut machine, "Continuous video feed?\n");
    input_string(&mut machine, "n");

    // final grid
    let grid: Grid = read_grid(&mut machine);
    // print_grid(&grid);

    // collected dust
    let output = machine.run();
    assert_eq!(output.len(), 1);
    output[0]
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Command {
    Move(i32),
    Left,
    Right,
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match *self {
            Command::Move(amount) => amount.to_string(),
            Command::Left => "L".to_string(),
            Command::Right => "R".to_string(),
        }
    }
}

fn trace_path(grid: &Grid) -> Vec<Command> {
    let (robot_pos, robot_tile) = grid
        .iter()
        .find(|(_, tile)| match tile {
            Tile::Robot(_) => true,
            _ => false,
        })
        .expect("robot not found");

    let mut robot_pos = *robot_pos;
    let mut robot_dir = match *robot_tile {
        Tile::Robot(dir) => dir,
        _ => panic!("cannot happen"),
    };

    let mut commands: Vec<Command> = Vec::new();
    let mut forward = 0;
    loop {
        if grid.get(&(robot_pos + robot_dir.step())) == Some(&Tile::Scaffold) {
            // continue forward
        } else if grid.get(&(robot_pos + robot_dir.rotate_left().step())) == Some(&Tile::Scaffold) {
            // turn left
            if forward > 0 {
                commands.push(Command::Move(forward));
                forward = 0;
            }
            commands.push(Command::Left);
            robot_dir = robot_dir.rotate_left();
        } else if grid.get(&(robot_pos + robot_dir.rotate_right().step())) == Some(&Tile::Scaffold)
        {
            // turn right
            if forward > 0 {
                commands.push(Command::Move(forward));
                forward = 0;
            }
            commands.push(Command::Right);
            robot_dir = robot_dir.rotate_right();
        } else {
            // dead end
            break;
        }
        forward += 1;
        robot_pos += robot_dir.step();
    }
    if forward > 0 {
        commands.push(Command::Move(forward));
    }

    commands
}

fn commands_to_string(commands: &[Command]) -> String {
    commands
        .iter()
        .map(|cmd| cmd.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

struct Functions {
    a: Vec<Command>,
    b: Vec<Command>,
    c: Vec<Command>,
}

fn find_functions(path: &[Command]) -> Option<Functions> {
    // A: break up the path somewhere
    for a_len in 1..=path.len() {
        let a = &path[0..a_len];
        if commands_to_string(a).len() > 20 {
            break; // function is too long
        }
        let a_parts: Vec<Vec<Command>> = split_vec(path, a)
            .into_iter()
            .filter(|x| !x.is_empty())
            .collect();
        // B: break up the first part that was broken up by A
        let b_part = &a_parts[0];
        for b_len in 1..=b_part.len() {
            let b = &b_part[0..b_len];
            if commands_to_string(b).len() > 20 {
                break; // function is too long
            }
            let b_parts: Vec<Vec<Command>> = a_parts
                .iter()
                .flat_map(|part| split_vec(part, b))
                .filter(|x| !x.is_empty())
                .collect();
            let c = &b_parts[0];
            if commands_to_string(c).len() > 20 {
                continue; // function is too long
            }
            // C: must cover all parts left over by A and B
            if b_parts.iter().all(|part| part == c) {
                return Some(Functions {
                    a: Vec::from(a),
                    b: Vec::from(b),
                    c: c.clone(),
                });
            }
        }
    }
    None
}

fn split_vec<T: Eq + Clone + std::fmt::Debug>(input: &[T], sep: &[T]) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = Vec::new();
    let mut i = 0;
    'outer: while i <= input.len() {
        for j in i..input.len() {
            if input[j..].starts_with(sep) {
                result.push(Vec::from(&input[i..j]));
                i = j + sep.len();
                continue 'outer;
            }
        }
        result.push(Vec::from(&input[i..]));
        break;
    }
    result
}

fn build_main(mut path: &[Command], functions: &Functions) -> String {
    let mut result: Vec<&str> = Vec::new();
    while path.len() > 0 {
        if path.starts_with(&functions.a) {
            result.push("A");
            path = &path[functions.a.len()..];
        } else if path.starts_with(&functions.b) {
            result.push("B");
            path = &path[functions.b.len()..];
        } else if path.starts_with(&functions.c) {
            result.push("C");
            path = &path[functions.c.len()..];
        } else {
            panic!("failed to match path with a function");
        }
    }
    result.into_iter().collect::<Vec<&str>>().join(",")
}

fn expect_prompt(machine: &mut ProgramMachine, expected: &str) {
    let output = read_string(machine);
    assert_eq!(output, expected);
}

fn read_string(machine: &mut ProgramMachine) -> String {
    let mut output = String::new();
    loop {
        match machine.step() {
            StepResult::Ok => {}
            StepResult::Output(value) => output.push(value as u8 as char),
            StepResult::NeedInput => break,
            StepResult::Halt => panic!("unexpected halt"),
        }
    }
    output
}

fn input_string(machine: &mut ProgramMachine, input: &str) {
    for byte in input.bytes() {
        machine.add_input(byte as i64);
    }
    machine.add_input('\n' as u8 as i64);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(
            intersection_alignment(&parse_grid(include_str!("example1"))),
            76
        );
    }
}
