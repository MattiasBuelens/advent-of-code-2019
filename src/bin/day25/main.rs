use std::collections::{HashMap, HashSet};
use std::io::{stdin, BufRead};

use regex::Regex;

use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::{Machine, ProgramMachine};
use advent_of_code_2019::vector2d::Vector2D;
use lazy_static::lazy_static;

fn main() {
    let program: Vec<i64> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&program));
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
enum Direction {
    West,
    South,
    East,
    North,
}

impl Direction {
    fn try_parse(s: &str) -> Result<Direction, String> {
        match s {
            "north" => Ok(Direction::North),
            "south" => Ok(Direction::South),
            "west" => Ok(Direction::West),
            "east" => Ok(Direction::East),
            _ => Err(String::from("invalid direction")),
        }
    }

    fn to_string(&self) -> &'static str {
        match *self {
            Direction::North => "north",
            Direction::South => "south",
            Direction::West => "west",
            Direction::East => "east",
        }
    }

    fn step(&self) -> Vector2D {
        match *self {
            Direction::North => Vector2D { x: 0, y: -1 },
            Direction::South => Vector2D { x: 0, y: 1 },
            Direction::West => Vector2D { x: -1, y: 0 },
            Direction::East => Vector2D { x: 1, y: 0 },
        }
    }

    fn reverse(&self) -> Direction {
        match *self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

enum Tile {
    Explored,
    Unknown,
}

impl Tile {
    fn print(&self) -> char {
        match *self {
            Tile::Explored => '.',
            Tile::Unknown => '?',
        }
    }
}

type Grid = HashMap<Vector2D, Tile>;

fn print_grid(grid: &Grid, droid_pos: &Vector2D) {
    let min_x = grid.keys().min_by_key(|pos| pos.x).unwrap().x;
    let min_y = grid.keys().min_by_key(|pos| pos.y).unwrap().y;
    let max_x = grid.keys().max_by_key(|pos| pos.x).unwrap().x;
    let max_y = grid.keys().max_by_key(|pos| pos.y).unwrap().y;
    for y in min_y..=max_y {
        let mut line = String::new();
        for x in min_x..=max_x {
            let pos = Vector2D::new(x, y);
            if &pos == droid_pos {
                line.push('D');
            } else if grid.contains_key(&pos) {
                line.push(grid.get(&pos).unwrap().print());
            } else {
                line.push(' ');
            }
        }
        println!("{}", line);
    }
}

fn read_line() -> String {
    let stdin = stdin();
    let mut lock = stdin.lock();
    let mut line = String::new();
    match lock.read_line(&mut line) {
        Ok(_) => line.trim().into(),
        Err(err) => panic!("input error: {}", err),
    }
}

fn parse_location(s: &str) -> Option<String> {
    lazy_static! {
        static ref LOCATION_RE: Regex = Regex::new(r"== ([^\n]+) ==").unwrap();
    }
    if let Some(captures) = LOCATION_RE.captures_iter(s).last() {
        let location = captures.get(1).unwrap().as_str();
        Some(location.into())
    } else {
        None
    }
}

fn parse_doors(s: &str) -> Option<Vec<Direction>> {
    lazy_static! {
        static ref DOORS_RE: Regex = Regex::new(r"Doors here lead:((?:\n\- \w+)+)").unwrap();
    }
    if let Some(captures) = DOORS_RE.captures(s) {
        let list = captures.get(1).unwrap().as_str().trim();
        let doors = list
            .split('\n')
            .map(|line| Direction::try_parse(&line[2..]).unwrap())
            .collect::<Vec<_>>();
        Some(doors)
    } else {
        None
    }
}

fn parse_items(s: &str) -> Option<Vec<String>> {
    lazy_static! {
        static ref ITEMS_RE: Regex = Regex::new(r"Items here:((?:\n\- [^\n]+)+)").unwrap();
    }
    if let Some(captures) = ITEMS_RE.captures(s) {
        let list = captures.get(1).unwrap().as_str().trim();
        let items = list
            .split('\n')
            .map(|line| line[2..].into())
            .collect::<Vec<String>>();
        Some(items)
    } else {
        None
    }
}

fn update_grid(output: &str, grid: &mut Grid, pos: Vector2D) {
    grid.insert(pos, Tile::Explored);
    if let Some(doors) = parse_doors(&output) {
        for dir in doors {
            let other_pos = pos + dir.step();
            if !grid.contains_key(&other_pos) {
                grid.insert(other_pos, Tile::Unknown);
            }
        }
    }
}

fn go_to_checkpoint(
    machine: &mut ProgramMachine,
    grid: &mut Grid,
    pos: &mut Vector2D,
    from: Direction,
) -> bool {
    lazy_static! {
        static ref BAD_ITEMS: HashSet<&'static str> = vec![
            "infinite loop",
            "molten lava",
            "escape pod",
            "giant electromagnet",
            "photons",
        ]
        .into_iter()
        .collect();
    }

    let output = machine.read_string();
    print!("{}", output);

    // Automatically update the grid
    update_grid(&output, grid, *pos);

    // Automatically pick up "good" items
    if let Some(items) = parse_items(&output) {
        for item in items {
            if !BAD_ITEMS.contains(&item[..]) {
                let command = format!("take {}", item);
                println!("{}", &command);
                machine.add_line(&command);
                print!("{}", machine.read_string());
            }
        }
    }

    let location = parse_location(&output).unwrap();
    if &location == "Security Checkpoint" {
        // Found the checkpoint!
        return true;
    }

    // Explore all doors
    let mut doors = parse_doors(&output).unwrap();
    doors.sort();
    for dir in doors {
        if dir == from {
            // Do not back track while exploring
            continue;
        }
        // Go through door
        let command = dir.to_string();
        println!("{}", &command);
        machine.add_line(&command);
        *pos = *pos + dir.step();
        // Explore
        if go_to_checkpoint(machine, grid, pos, dir.reverse()) {
            // Found the checkpoint!
            return true;
        }
        // Go back
        let command = dir.reverse().to_string();
        println!("{}", &command);
        machine.add_line(&command);
        *pos = *pos + dir.reverse().step();
        // Consume the redundant command output
        print!("{}", machine.read_string());
    }

    false
}

fn part1(program: &Vec<i64>) -> i32 {
    let mut machine = ProgramMachine::new(program.clone(), vec![]);
    let mut grid: Grid = HashMap::new();
    let mut pos = Vector2D::zero();

    go_to_checkpoint(&mut machine, &mut grid, &mut pos, Direction::North);

    let mut prev_pos = pos;
    loop {
        let output = machine.read_string();
        if output.contains("ejected back") {
            pos = prev_pos;
        }

        // Update grid
        update_grid(&output, &mut grid, pos.clone());

        // Print the map
        println!("== Map ==");
        print_grid(&grid, &pos);
        println!();

        // Print the command's output
        println!("{}", output.trim());

        let input = read_line();
        machine.add_line(&input);

        // Update position
        if let Ok(dir) = Direction::try_parse(&input) {
            prev_pos = pos;
            pos = pos + dir.step();
        }
    }
}
