use std::collections::HashSet;

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

#[derive(Debug, Copy, Clone)]
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

fn part2(grid: Grid) -> u32 {
    0
}
