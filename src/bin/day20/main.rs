use std::collections::HashMap;
use std::iter::FromIterator;

use pathfinding::directed::bfs::bfs;

use advent_of_code_2019::vector2d::Vector2D;

fn main() {
    let maze: Maze = parse_input(include_str!("input"));
    println!("Answer to part 1: {}", part1(&maze));
    println!("Answer to part 2: {}", part2(&maze));
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Tile {
    Open,
    Wall,
    Portal(String, bool),
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
    let max_x = grid.keys().max_by_key(|pos| pos.x).unwrap().x;
    let max_y = grid.keys().max_by_key(|pos| pos.y).unwrap().y;
    for (&pos, &letter) in &portal_letters {
        for step in get_steps() {
            let other_pos = pos + step;
            if let Some(&other_letter) = portal_letters.get(&other_pos) {
                let open_pos = pos - step;
                if let Some(Tile::Open) = grid.get(&open_pos) {
                    let name = if pos.manhattan_distance() < other_pos.manhattan_distance() {
                        String::from_iter(vec![letter, other_letter])
                    } else {
                        String::from_iter(vec![other_letter, letter])
                    };
                    let outside = pos.x < 2 || pos.y < 2 || pos.x > max_x - 2 || pos.y > max_y - 2;
                    grid.insert(pos, Tile::Portal(name.clone(), outside));
                    portals.entry(name).or_default().push(open_pos);
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

fn get_successors(maze: &Maze, pos: Vector2D) -> Vec<Vector2D> {
    get_steps()
        .iter()
        .filter_map(|&step| {
            let other = pos + step;
            match maze.grid.get(&other) {
                Some(Tile::Open) => Some(other),
                Some(Tile::Portal(name, _)) => {
                    let portal = maze.portals.get(name).unwrap();
                    if portal.len() == 2 {
                        let other = if pos == portal[0] {
                            portal[1]
                        } else {
                            portal[0]
                        };
                        Some(other)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .collect()
}

fn part1(maze: &Maze) -> usize {
    let start = maze.portals.get("AA").unwrap()[0];
    let goal = maze.portals.get("ZZ").unwrap()[0];
    let path = bfs(&start, |&pos| get_successors(maze, pos), |pos| pos == &goal)
        .expect("could not find a path to goal portal");

    (path.len() - 1)
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
