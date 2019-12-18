use std::collections::HashMap;

use pathfinding::directed::dijkstra::dijkstra;

use advent_of_code_2019::vector2d::Vector2D;

fn main() {
    let (grid, starts) = parse_grid(include_str!("input"));
    println!("Answer to part 1: {}", part1(&grid, &starts[0]));

    let (grid, starts) = split_grid(&grid, starts[0]);
    println!("Answer to part 2: {}", part2(&grid, &starts));
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

fn parse_grid(input: &str) -> (Grid, Vec<Vector2D>) {
    let mut grid = HashMap::new();
    let mut starts = Vec::new();
    let mut y = 0;
    for line in input.trim().lines() {
        let mut x = 0;
        for cell in line.chars() {
            let pos = Vector2D::new(x, y);
            if cell == '@' {
                starts.push(pos);
                grid.insert(pos, Tile::Open);
            } else {
                grid.insert(pos, Tile::parse(cell));
            }
            x += 1;
        }
        y += 1;
    }
    (grid, starts)
}

fn print_grid(grid: &Grid, robots: &Vec<Vector2D>) {
    let min_x = grid.keys().min_by_key(|pos| pos.x).unwrap().x;
    let min_y = grid.keys().min_by_key(|pos| pos.y).unwrap().y;
    let max_x = grid.keys().max_by_key(|pos| pos.x).unwrap().x;
    let max_y = grid.keys().max_by_key(|pos| pos.y).unwrap().y;
    for y in min_y..=max_y {
        let mut line = String::new();
        for x in min_x..=max_x {
            let pos = Vector2D::new(x, y);
            if robots.contains(&pos) {
                line.push('@');
            } else {
                line.push(grid.get(&pos).unwrap().print());
            }
        }
        println!("{}", line);
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct NodeSingle(Vector2D, String);

fn part1(grid: &Grid, start: &Vector2D) -> i32 {
    let all_keys = get_all_keys(grid);
    let start_node = NodeSingle(*start, String::new());
    let (_, cost) = dijkstra(
        &start_node,
        |NodeSingle(pos, keys)| {
            get_neighbours(*pos)
                .iter()
                .filter_map(|neighbour| match grid.get(neighbour) {
                    Some(Tile::Key(letter)) => {
                        Some((NodeSingle(*neighbour, add_key(keys.clone(), *letter)), 1))
                    }
                    Some(tile) if can_traverse(tile, keys) => {
                        Some((NodeSingle(*neighbour, keys.clone()), 1))
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        },
        |NodeSingle(_, keys)| keys.len() == all_keys.len(),
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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct NodeMulti(Vec<Vector2D>, String);

fn part2(grid: &Grid, starts: &Vec<Vector2D>) -> i32 {
    let all_keys = get_all_keys(&grid);
    let start_node = NodeMulti(starts.clone(), String::new());
    let (_, cost) = dijkstra(
        &start_node,
        |NodeMulti(positions, keys)| {
            positions
                .iter()
                .zip(0..)
                .flat_map(|(pos, i)| {
                    get_neighbours(*pos)
                        .iter()
                        .filter_map(|neighbour| match grid.get(neighbour) {
                            Some(Tile::Key(letter)) => Some((
                                NodeMulti(
                                    replace_pos(positions.clone(), i, *neighbour),
                                    add_key(keys.clone(), *letter),
                                ),
                                1,
                            )),
                            Some(tile) if can_traverse(tile, keys) => Some((
                                NodeMulti(
                                    replace_pos(positions.clone(), i, *neighbour),
                                    keys.clone(),
                                ),
                                1,
                            )),
                            _ => None,
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        },
        |NodeMulti(_, keys)| keys.len() == all_keys.len(),
    )
    .expect("could not find a path to all keys");

    cost
}

fn split_grid(grid: &Grid, start: Vector2D) -> (Grid, Vec<Vector2D>) {
    let mut grid = grid.clone();
    // place extra walls
    grid.insert(start, Tile::Wall);
    for neighbour in get_neighbours(start) {
        grid.insert(neighbour, Tile::Wall);
    }
    // update start positions
    let starts = vec![
        start + Vector2D::new(-1, -1),
        start + Vector2D::new(-1, 1),
        start + Vector2D::new(1, -1),
        start + Vector2D::new(1, 1),
    ];
    (grid, starts)
}

fn replace_pos(mut positions: Vec<Vector2D>, i: usize, new_pos: Vector2D) -> Vec<Vector2D> {
    positions[i] = new_pos;
    positions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let (grid, starts) = parse_grid(include_str!("example1"));
        assert_eq!(part1(&grid, &starts[0]), 8);
    }

    #[test]
    fn test_part1_example2() {
        let (grid, starts) = parse_grid(include_str!("example2"));
        assert_eq!(part1(&grid, &starts[0]), 86);
    }

    #[test]
    fn test_part1_example3() {
        let (grid, starts) = parse_grid(include_str!("example3"));
        assert_eq!(part1(&grid, &starts[0]), 132);
    }

    #[test]
    fn test_part1_example4() {
        let (grid, starts) = parse_grid(include_str!("example4"));
        assert_eq!(part1(&grid, &starts[0]), 136);
    }

    #[test]
    fn test_part1_example5() {
        let (grid, starts) = parse_grid(include_str!("example5"));
        assert_eq!(part1(&grid, &starts[0]), 81);
    }

    #[test]
    fn test_part2_example6() {
        let (grid, starts) = parse_grid(include_str!("example6"));
        assert_eq!(part2(&grid, &starts), 8);
    }

    #[test]
    fn test_part2_example7() {
        let (grid, starts) = parse_grid(include_str!("example7"));
        assert_eq!(part2(&grid, &starts), 24);
    }

    #[test]
    fn test_part2_example8() {
        let (grid, starts) = parse_grid(include_str!("example8"));
        assert_eq!(part2(&grid, &starts), 32);
    }

    #[test]
    fn test_part2_example9() {
        let (grid, starts) = parse_grid(include_str!("example9"));
        assert_eq!(part2(&grid, &starts), 72);
    }
}
