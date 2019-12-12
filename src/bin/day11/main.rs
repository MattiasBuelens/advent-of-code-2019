use std::collections::HashMap;

use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::*;
use advent_of_code_2019::vector2d::Vector2D;

fn main() {
    let input: Vec<i64> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&input));
    println!("Answer to part 2:");
    part2(&input);
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Color {
    BLACK,
    WHITE,
}

impl Color {
    fn parse(value: i64) -> Color {
        match value {
            0 => Color::BLACK,
            1 => Color::WHITE,
            _ => panic!("invalid color {}", value),
        }
    }

    fn to_number(&self) -> i64 {
        match *self {
            Color::BLACK => 0,
            Color::WHITE => 1,
        }
    }

    fn print(&self) -> &'static str {
        match *self {
            Color::BLACK => "  ",
            Color::WHITE => "##",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn step(&self) -> Vector2D {
        match *self {
            Direction::Up => Vector2D { x: 0, y: 1 },
            Direction::Down => Vector2D { x: 0, y: -1 },
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
}

fn run(program: &Vec<i64>, start_color: Color) -> HashMap<Vector2D, Color> {
    let mut machine = ProgramMachine::new(program.clone(), vec![]);
    let mut grid: HashMap<Vector2D, Color> = HashMap::new();
    let mut pos = Vector2D::zero();
    let mut dir = Direction::Up;
    grid.insert(pos, start_color);
    loop {
        let color = grid.get(&pos).unwrap_or(&Color::BLACK);
        machine.add_input(color.to_number());
        match machine.run_to_output() {
            Some(value) => {
                grid.insert(pos, Color::parse(value));
            }
            None => {
                break;
            }
        }
        match machine.run_to_output() {
            Some(value) => {
                dir = match value {
                    0 => dir.rotate_left(),
                    1 => dir.rotate_right(),
                    _ => panic!("unexpected rotation"),
                }
            }
            None => {
                break;
            }
        }
        pos += dir.step();
    }
    grid
}

fn part1(program: &Vec<i64>) -> usize {
    let grid = run(program, Color::BLACK);
    grid.len()
}

fn part2(program: &Vec<i64>) {
    let grid = run(program, Color::WHITE);
    let min_x = grid.keys().min_by_key(|pos| pos.x).unwrap().x;
    let min_y = grid.keys().min_by_key(|pos| pos.y).unwrap().y;
    let max_x = grid.keys().max_by_key(|pos| pos.x).unwrap().x;
    let max_y = grid.keys().max_by_key(|pos| pos.y).unwrap().y;
    for y in (min_y..=max_y).rev() {
        let mut line = String::new();
        for x in min_x..=max_x {
            let color = grid.get(&Vector2D::new(x, y)).unwrap_or(&Color::BLACK);
            line.push_str(color.print());
        }
        println!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {}
}
