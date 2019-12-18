use std::collections::HashMap;

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
    print_grid(grid, start);

    0
}

fn part2(grid: &Grid, start: &Vector2D) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {}
}
