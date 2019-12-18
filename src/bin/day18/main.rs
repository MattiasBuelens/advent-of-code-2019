use std::collections::{HashMap, HashSet};

use pathfinding::directed::dijkstra::dijkstra_all;

use advent_of_code_2019::vector2d::Vector2D;

fn main() {
    let (grid, start) = parse_grid(include_str!("input"));
    println!("Answer to part 1: {}", part1(&grid, &start));
    println!("Answer to part 2: {}", part2(&grid, &start));
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Open,
    Wall,
    Key(char),
    Door(char),
}

impl Tile {
    fn parse(value: char) -> Tile {
        match value {
            '#' => Tile::Wall,
            '.' => Tile::Open,
            letter @ 'a'..='z' => Tile::Key(letter),
            letter @ 'A'..='Z' => Tile::Door(letter),
            _ => panic!("invalid tile"),
        }
    }

    fn print(&self) -> char {
        match *self {
            Tile::Wall => '#',
            Tile::Open => '.',
            Tile::Key(letter) => letter,
            Tile::Door(letter) => letter,
        }
    }
}

type Grid = HashMap<Vector2D, Tile>;

fn parse_grid(input: &str) -> (Grid, Vector2D) {
    let mut grid = HashMap::new();
    let mut start = None;
    let mut y = 0;
    for line in input.trim().lines() {
        let mut x = 0;
        for cell in line.chars() {
            let pos = Vector2D::new(x, y);
            if cell == '@' {
                start = Some(pos);
                grid.insert(pos, Tile::Open);
            } else {
                grid.insert(pos, Tile::parse(cell));
            }
            x += 1;
        }
        y += 1;
    }
    (grid, start.unwrap())
}

fn print_grid(grid: &Grid, you: &Vector2D) {
    let min_x = grid.keys().min_by_key(|pos| pos.x).unwrap().x;
    let min_y = grid.keys().min_by_key(|pos| pos.y).unwrap().y;
    let max_x = grid.keys().max_by_key(|pos| pos.x).unwrap().x;
    let max_y = grid.keys().max_by_key(|pos| pos.y).unwrap().y;
    for y in min_y..=max_y {
        let mut line = String::new();
        for x in min_x..=max_x {
            let pos = Vector2D::new(x, y);
            if pos == *you {
                line.push('@');
            } else {
                line.push(grid.get(&pos).unwrap().print());
            }
        }
        println!("{}", line);
    }
}

fn part1(grid: &Grid, start: &Vector2D) -> i32 {
    // print_grid(grid, start);
    let owned_keys: HashSet<char> = HashSet::new();
    let result = get_least_steps(grid, start, &owned_keys);
    result
}

fn get_least_steps(grid: &Grid, start: &Vector2D, owned_keys: &HashSet<char>) -> i32 {
    let distances = get_distances_from(grid, start, &owned_keys);
    let mut best: Option<i32> = None;
    for (pos, (_, distance)) in distances.iter() {
        if let Some(Tile::Key(letter)) = grid.get(pos) {
            if !owned_keys.contains(letter) {
                // Found a key that we don't have yet.
                // Pick it up and continue from here.
                let mut owned_keys = owned_keys.clone();
                owned_keys.insert(*letter);
                let result = *distance + get_least_steps(grid, pos, &owned_keys);
                best = Some(best.map_or(result, |best| best.min(result)));
            }
        }
    }
    // If we already own all keys, then `best` will be `None` and we don't need to go anywhere else.
    best.unwrap_or(0)
}

fn get_distances_from(
    grid: &Grid,
    start: &Vector2D,
    owned_keys: &HashSet<char>,
) -> HashMap<Vector2D, (Vector2D, i32)> {
    dijkstra_all(start, |pos| {
        get_neighbours(*pos)
            .iter()
            .filter(|neighbour| {
                grid.get(neighbour)
                    .map_or(false, |tile| can_traverse(tile, owned_keys))
            })
            .map(|neighbour| (*neighbour, 1))
            .collect::<Vec<_>>()
    })
}

fn get_neighbours(position: Vector2D) -> Vec<Vector2D> {
    vec![
        position + Vector2D::new(0, 1),
        position + Vector2D::new(0, -1),
        position + Vector2D::new(1, 0),
        position + Vector2D::new(-1, 0),
    ]
}

fn can_traverse(tile: &Tile, owned_keys: &HashSet<char>) -> bool {
    match *tile {
        Tile::Open => true,
        Tile::Wall => false,
        Tile::Key(_) => true,
        Tile::Door(letter) => owned_keys.contains(&letter.to_ascii_lowercase()),
    }
}

fn part2(grid: &Grid, start: &Vector2D) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let (grid, start) = parse_grid(include_str!("example1"));
        assert_eq!(part1(&grid, &start), 8);
    }

    #[test]
    fn test_part1_example2() {
        let (grid, start) = parse_grid(include_str!("example2"));
        assert_eq!(part1(&grid, &start), 86);
    }

    #[test]
    fn test_part1_example3() {
        let (grid, start) = parse_grid(include_str!("example3"));
        assert_eq!(part1(&grid, &start), 132);
    }
}
