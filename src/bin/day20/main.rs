use std::collections::HashMap;
use std::iter::FromIterator;

use crate::Tile::Portal;
use advent_of_code_2019::vector2d::Vector2D;

fn main() {
    let maze: Maze = parse_input(include_str!("input"));
    println!("{:?}", maze);
    println!("Answer to part 1: {}", part1(&maze));
    println!("Answer to part 2: {}", part2(&maze));
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Tile {
    Open,
    Wall,
    Portal(String),
}

#[derive(Debug)]
struct Maze {
    grid: HashMap<Vector2D, Tile>,
    portals: HashMap<String, Vec<Vector2D>>,
}

fn parse_input(input: &str) -> Maze {
    let mut grid = HashMap::new();
    let mut portal_letters: HashMap<Vector2D, char> = HashMap::new();
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for cell in line.chars() {
            let pos = Vector2D::new(x, y);
            if cell == '#' {
                grid.insert(pos, Tile::Wall);
            } else if cell == '.' {
                grid.insert(pos, Tile::Open);
            } else if cell == ' ' {
                // ignore
            } else if cell.is_ascii_uppercase() {
                portal_letters.insert(pos, cell);
            } else {
                panic!("unexpected cell {} at {:?}", cell, pos);
            }
            x += 1;
        }
        y += 1;
    }
    let mut portals: HashMap<String, Vec<Vector2D>> = HashMap::new();
    for (&pos, &letter) in &portal_letters {
        for step in get_steps() {
            let other_pos = pos + step;
            if let Some(&other_letter) = portal_letters.get(&other_pos) {
                if let Some(Tile::Open) = grid.get(&(pos - step)) {
                    let name = if pos.manhattan_distance() < other_pos.manhattan_distance() {
                        String::from_iter(vec![letter, other_letter])
                    } else {
                        String::from_iter(vec![other_letter, letter])
                    };
                    grid.insert(pos, Portal(name.clone()));
                    portals.entry(name).or_default().push(pos)
                }
            }
        }
    }
    Maze { grid, portals }
}

fn get_steps() -> Vec<Vector2D> {
    vec![
        Vector2D::new(0, 1),
        Vector2D::new(0, -1),
        Vector2D::new(1, 0),
        Vector2D::new(-1, 0),
    ]
}

fn part1(maze: &Maze) -> i32 {
    0
}

fn part2(maze: &Maze) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {}
}
