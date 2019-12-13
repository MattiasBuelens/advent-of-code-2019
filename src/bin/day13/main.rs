use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::*;
use advent_of_code_2019::vector2d::Vector2D;

fn main() {
    let program: Vec<i64> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&program));
    println!("Answer to part 2: {}", part2(&program, false));
}

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    EMPTY,
    WALL,
    BLOCK,
    PADDLE,
    BALL,
}

impl Tile {
    fn from_id(id: i32) -> Tile {
        match id {
            0 => Tile::EMPTY,
            1 => Tile::WALL,
            2 => Tile::BLOCK,
            3 => Tile::PADDLE,
            4 => Tile::BALL,
            _ => panic!("unknown tile id {}", id),
        }
    }

    fn print(&self) -> &'static str {
        match *self {
            Tile::EMPTY => " ",
            Tile::WALL => "#",
            Tile::BLOCK => "X",
            Tile::PADDLE => "_",
            Tile::BALL => "O",
        }
    }
}

type Screen = HashMap<Vector2D, Tile>;

fn part1(program: &Vec<i64>) -> usize {
    let mut screen: Screen = HashMap::new();
    let mut machine = ProgramMachine::new(program.clone(), vec![]);
    loop {
        let x = match machine.run_to_output() {
            Some(value) => value,
            None => {
                break;
            }
        };
        let y = machine.run_to_output().unwrap();
        let tile_id = machine.run_to_output().unwrap();
        screen.insert(
            Vector2D::new(x as i32, y as i32),
            Tile::from_id(tile_id as i32),
        );
    }
    screen.values().filter(|tile| **tile == Tile::BLOCK).count()
}

fn part2(program: &Vec<i64>, interactive: bool) -> i64 {
    let mut program = program.clone();
    program[0] = 2;

    let mut screen: Screen = HashMap::new();
    let mut score: i64 = 0;
    let mut machine = ProgramMachine::new(program, vec![]);
    'outer: loop {
        let x: i64 = loop {
            match machine.step() {
                StepResult::NeedInput => {
                    machine.add_input(if interactive {
                        print_screen(&screen);
                        println!("Score: {}", score);
                        read_joystick()
                    } else {
                        compute_joystick(&screen)
                    });
                }
                StepResult::Output(value) => {
                    break value;
                }
                StepResult::Halt => {
                    break 'outer;
                }
                _ => {}
            }
        };
        let y = machine.run_to_output().unwrap();
        let z = machine.run_to_output().unwrap();
        if x == -1 && y == 0 {
            score = z;
        } else {
            screen.insert(Vector2D::new(x as i32, y as i32), Tile::from_id(z as i32));
        }
    }

    score
}

fn print_screen(screen: &Screen) {
    let min_x = screen.keys().min_by_key(|pos| pos.x).unwrap().x;
    let min_y = screen.keys().min_by_key(|pos| pos.y).unwrap().y;
    let max_x = screen.keys().max_by_key(|pos| pos.x).unwrap().x;
    let max_y = screen.keys().max_by_key(|pos| pos.y).unwrap().y;
    for y in min_y..=max_y {
        let mut line = String::new();
        for x in min_x..=max_x {
            let tile = screen.get(&Vector2D::new(x, y)).unwrap_or(&Tile::EMPTY);
            line.push_str(tile.print());
        }
        println!("{}", line);
    }
}

fn read_joystick() -> i64 {
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    println!("Enter next move (L, R or nothing):");
    loop {
        let mut line = String::new();
        match lock.read_line(&mut line) {
            Ok(_) => match line.to_ascii_lowercase().trim() {
                "" => return 0,
                "l" => return -1,
                "r" => return 1,
                _ => {
                    println!("Unknown input {}", &line);
                }
            },
            Err(err) => panic!("input error: {}", err),
        }
    }
}

fn compute_joystick(screen: &Screen) -> i64 {
    let (paddle_pos, _) = screen
        .iter()
        .find(|(_, tile)| **tile == Tile::PADDLE)
        .expect("paddle must exist");
    let (ball_pos, _) = screen
        .iter()
        .find(|(_, tile)| **tile == Tile::BALL)
        .expect("ball must exist");
    match ball_pos.x.cmp(&paddle_pos.x) {
        Ordering::Equal => 0,   // neutral
        Ordering::Less => -1,   // left
        Ordering::Greater => 1, // right
    }
}
