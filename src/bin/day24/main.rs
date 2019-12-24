use std::collections::{HashMap, HashSet};

fn main() {
    let grid: Grid = Grid::parse(include_str!("input"));
    println!("Answer to part 1: {}", part1(grid.clone()));
    println!("Answer to part 2: {}", part2(grid.clone()));
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    EMPTY,
    BUG,
}

impl Tile {
    fn parse(value: char) -> Self {
        match value {
            '.' => Tile::EMPTY,
            '#' => Tile::BUG,
            _ => panic!("invalid tile: {}", value),
        }
    }

    fn print(&self) -> char {
        match *self {
            Tile::EMPTY => '.',
            Tile::BUG => '#',
        }
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile::EMPTY
    }
}

const SIZE: usize = 5;

#[derive(Debug, Default, Copy, Clone)]
struct Grid {
    tiles: [[Tile; SIZE]; SIZE],
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut tiles: [[Tile; SIZE]; SIZE] = Default::default();
        let mut y = 0;
        for line in input.trim().lines() {
            let mut x = 0;
            for value in line.chars() {
                tiles[y][x] = Tile::parse(value);
                x += 1;
            }
            y += 1;
        }
        Grid { tiles }
    }

    fn print(&self) {
        for row in &self.tiles {
            println!("{}", row.iter().map(|x| x.print()).collect::<String>());
        }
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<Tile> {
        let mut neighbours: Vec<Tile> = Vec::new();
        if y > 0 {
            neighbours.push(self.tiles[y - 1][x]);
        }
        if x > 0 {
            neighbours.push(self.tiles[y][x - 1]);
        }
        if x + 1 < SIZE {
            neighbours.push(self.tiles[y][x + 1]);
        }
        if y + 1 < SIZE {
            neighbours.push(self.tiles[y + 1][x]);
        }
        neighbours
    }

    fn step(&mut self) {
        let mut new_tiles: [[Tile; SIZE]; SIZE] = Default::default();
        for y in 0..SIZE {
            for x in 0..SIZE {
                let neighbour_bugs = self
                    .get_neighbours(x, y)
                    .iter()
                    .filter(|&x| x == &Tile::BUG)
                    .count();
                new_tiles[y][x] = match (self.tiles[y][x], neighbour_bugs) {
                    // A bug dies (becoming an empty space) unless there is exactly one bug
                    // adjacent to it.
                    (Tile::BUG, 1) => Tile::BUG,
                    (Tile::BUG, _) => Tile::EMPTY,
                    // An empty space becomes infested with a bug if exactly one or two bugs
                    // are adjacent to it.
                    (Tile::EMPTY, 1) | (Tile::EMPTY, 2) => Tile::BUG,
                    (Tile::EMPTY, _) => Tile::EMPTY,
                }
            }
        }
        self.tiles = new_tiles;
    }

    fn get_biodiversity_rating(&self) -> u32 {
        let mut rating = 0;
        let mut power = 1;
        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.tiles[y][x] == Tile::BUG {
                    rating |= power;
                }
                power <<= 1;
            }
        }
        rating
    }

    fn count_bugs(&self) -> usize {
        self.tiles
            .iter()
            .map(|row| row.iter().filter(|&x| x == &Tile::BUG).count())
            .sum()
    }
}

fn part1(mut grid: Grid) -> u32 {
    let mut seen_ratings: HashSet<u32> = HashSet::new();
    seen_ratings.insert(grid.get_biodiversity_rating());

    loop {
        grid.step();
        let rating = grid.get_biodiversity_rating();
        if seen_ratings.contains(&rating) {
            break;
        } else {
            seen_ratings.insert(rating);
        }
    }

    // grid.print();
    grid.get_biodiversity_rating()
}

#[derive(Debug)]
struct MultiGrid {
    grids: HashMap<isize, Grid>,
}

impl MultiGrid {
    fn new(grid: Grid) -> Self {
        let mut grids = HashMap::new();
        grids.insert(0, grid);
        MultiGrid { grids }
    }

    fn print(&self) {
        let mut levels = self.grids.keys().collect::<Vec<_>>();
        levels.sort();
        for level in levels {
            println!("Depth {}", level);
            self.grids[level].print();
            println!();
        }
    }

    fn get_tile(&self, level: isize, x: usize, y: usize) -> Tile {
        assert!(x != 2 || y != 2);
        if let Some(grid) = self.grids.get(&level) {
            grid.tiles[y][x]
        } else {
            Tile::EMPTY
        }
    }

    fn get_neighbours(&self, level: isize, x: usize, y: usize) -> Vec<Tile> {
        let mut neighbours: Vec<Tile> = Vec::new();
        // Top neighbour
        match (x, y) {
            (_, 0) => {
                // Top row wraps to upper level
                neighbours.push(self.get_tile(level - 1, 2, 1));
            }
            (2, 3) => {
                // Center wraps to bottom row of lower level
                for x in 0..SIZE {
                    neighbours.push(self.get_tile(level + 1, x, SIZE - 1));
                }
            }
            _ => {
                neighbours.push(self.get_tile(level, x, y - 1));
            }
        }
        // Left neighbour
        match (x, y) {
            (0, _) => {
                // Left column wraps to upper level
                neighbours.push(self.get_tile(level - 1, 1, 2));
            }
            (3, 2) => {
                // Center wraps to right column of lower level
                for y in 0..SIZE {
                    neighbours.push(self.get_tile(level + 1, SIZE - 1, y));
                }
            }
            _ => {
                neighbours.push(self.get_tile(level, x - 1, y));
            }
        }
        // Right neighbour
        match (x, y) {
            (4, _) => {
                // Right column wraps to upper level
                neighbours.push(self.get_tile(level - 1, 3, 2));
            }
            (1, 2) => {
                // Center wraps to left column of lower level
                for y in 0..SIZE {
                    neighbours.push(self.get_tile(level + 1, 0, y));
                }
            }
            _ => {
                neighbours.push(self.get_tile(level, x + 1, y));
            }
        }
        // Bottom neighbour
        match (x, y) {
            (_, 4) => {
                // Bottom row wraps to upper level
                neighbours.push(self.get_tile(level - 1, 2, 3));
            }
            (2, 1) => {
                // Center wraps to top row of lower level
                for x in 0..SIZE {
                    neighbours.push(self.get_tile(level + 1, x, 0));
                }
            }
            _ => {
                neighbours.push(self.get_tile(level, x, y + 1));
            }
        }
        neighbours
    }

    fn step_tile(&self, level: isize, x: usize, y: usize) -> Tile {
        let neighbour_bugs = self
            .get_neighbours(level, x, y)
            .iter()
            .filter(|&x| x == &Tile::BUG)
            .count();
        match (self.get_tile(level, x, y), neighbour_bugs) {
            // A bug dies (becoming an empty space) unless there is exactly one bug
            // adjacent to it.
            (Tile::BUG, 1) => Tile::BUG,
            (Tile::BUG, _) => Tile::EMPTY,
            // An empty space becomes infested with a bug if exactly one or two bugs
            // are adjacent to it.
            (Tile::EMPTY, 1) | (Tile::EMPTY, 2) => Tile::BUG,
            (Tile::EMPTY, _) => Tile::EMPTY,
        }
    }

    fn step(&mut self) {
        let mut new_grids: HashMap<isize, Grid> = self.grids.clone();
        let min_level = *self.grids.keys().min().unwrap();
        let max_level = *self.grids.keys().max().unwrap();
        // Try to expand one more upper and outer level
        for level in (min_level - 1)..=(max_level + 1) {
            for y in 0..SIZE {
                for x in 0..SIZE {
                    if x == 2 && y == 2 {
                        // Skip the center tile
                        continue;
                    }
                    let new_tile = self.step_tile(level, x, y);
                    // Only create new levels for bug tiles
                    if new_tile == Tile::BUG || self.grids.contains_key(&level) {
                        let new_grid = new_grids.entry(level).or_insert(Default::default());
                        new_grid.tiles[y][x] = new_tile;
                    }
                }
            }
        }
        self.grids = new_grids;
    }

    fn count_bugs(&self) -> usize {
        self.grids.values().map(|grid| grid.count_bugs()).sum()
    }
}

fn part2(grid: Grid) -> usize {
    let mut multi_grid = MultiGrid::new(grid);
    for _ in 0..200 {
        multi_grid.step();
    }
    // multi_grid.print();
    multi_grid.count_bugs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let grid = Grid::parse(include_str!("example"));
        assert_eq!(part1(grid), 2129920);
    }
}
