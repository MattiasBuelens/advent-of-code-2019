use std::collections::HashMap;

use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::{Machine, ProgramMachine};
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
    // print_grid(&grid);
    let intersections = find_intersections(&grid);
    intersection_alignment(&intersections)
}

fn read_grid(machine: &mut ProgramMachine) -> Grid {
    let mut grid: Grid = HashMap::new();
    let mut y = 0;
    let mut x = 0;
    loop {
        match machine.run_to_output() {
            Some(value) => match value as u8 as char {
                '\n' => {
                    x = 0;
                    y += 1;
                }
                _ => {
                    grid.insert(Vector2D::new(x, y), Tile::parse(value as u8 as char));
                    x += 1;
                }
            },
            None => break,
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

fn find_intersections(grid: &Grid) -> Vec<Vector2D> {
    let mut result = Vec::new();
    for (pos, _) in grid {
        if is_intersection(grid, *pos) {
            result.push(*pos);
        }
    }
    result
}

fn intersection_alignment(intersections: &Vec<Vector2D>) -> i32 {
    intersections.iter().map(|pos| pos.x * pos.y).sum()
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

fn part2(program: &Vec<i64>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        assert_eq!(
            intersection_alignment(&find_intersections(&parse_grid(include_str!("example1")))),
            76
        );
    }
}
