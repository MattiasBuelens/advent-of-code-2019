use std::collections::HashMap;

use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::{Machine, ProgramMachine};
use advent_of_code_2019::vector2d::Vector2D;

fn main() {
    let program: Vec<i64> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&program));
    println!("Answer to part 2: {}", part2(&program));
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

fn part2(program: &Vec<i64>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {}
}
