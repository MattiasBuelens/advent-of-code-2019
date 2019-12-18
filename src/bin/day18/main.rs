use std::collections::HashMap;

use pathfinding::directed::dijkstra::dijkstra;

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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Node(Vector2D, String);

fn part1(grid: &Grid, start: &Vector2D) -> i32 {
    let all_keys = get_all_keys(grid);
    let start_node = Node(*start, String::new());
    let (_, cost) = dijkstra(
        &start_node,
        |Node(pos, keys)| {
            get_neighbours(*pos)
                .iter()
                .filter_map(|neighbour| match grid.get(neighbour) {
                    Some(Tile::Key(letter)) => {
                        Some((Node(*neighbour, add_key(keys.clone(), *letter)), 1))
                    }
                    Some(tile) if can_traverse(tile, keys) => {
                        Some((Node(*neighbour, keys.clone()), 1))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        },
        |Node(_, keys)| keys.len() == all_keys.len(),
    )
    .expect("could not find a path to all keys");

    cost
}

fn get_neighbours(position: Vector2D) -> Vec<Vector2D> {
    vec![
        position + Vector2D::new(0, 1),
        position + Vector2D::new(0, -1),
        position + Vector2D::new(1, 0),
        position + Vector2D::new(-1, 0),
    ]
}

fn get_all_keys(grid: &Grid) -> String {
    let mut keys = grid
        .values()
        .filter_map(|tile| match *tile {
            Tile::Key(letter) => Some(letter),
            _ => None,
        })
        .collect::<Vec<char>>();
    keys.sort();
    keys.iter().collect()
}

fn add_key(mut keys: String, letter: char) -> String {
    if !keys.contains(&letter.to_string()) {
        let idx = keys.chars().position(|c| letter < c).unwrap_or(keys.len());
        keys.insert(idx, letter);
    }
    keys
}

fn can_traverse(tile: &Tile, owned_keys: &str) -> bool {
    match *tile {
        Tile::Open => true,
        Tile::Wall => false,
        Tile::Key(_) => true,
        Tile::Door(letter) => owned_keys.contains(&letter.to_ascii_lowercase().to_string()),
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

    #[test]
    fn test_part1_example4() {
        let (grid, start) = parse_grid(include_str!("example4"));
        assert_eq!(part1(&grid, &start), 136);
    }

    #[test]
    fn test_part1_example5() {
        let (grid, start) = parse_grid(include_str!("example5"));
        assert_eq!(part1(&grid, &start), 81);
    }
}
