use std::cell::RefCell;
use std::collections::HashMap;

use pathfinding::directed::astar::*;
use pathfinding::directed::dijkstra::*;

use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::{Machine, ProgramMachine};
use advent_of_code_2019::vector2d::Vector2D;

fn main() {
    let program: Vec<i64> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&program));
    println!("Answer to part 2: {}", part2(&program));
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn from_step(step: Vector2D) -> Direction {
        match (step.x, step.y) {
            (0, 1) => Direction::North,
            (0, -1) => Direction::South,
            (-1, 0) => Direction::West,
            (1, 0) => Direction::East,
            _ => panic!("invalid step"),
        }
    }

    fn step(&self) -> Vector2D {
        match *self {
            Direction::North => Vector2D { x: 0, y: 1 },
            Direction::South => Vector2D { x: 0, y: -1 },
            Direction::West => Vector2D { x: -1, y: 0 },
            Direction::East => Vector2D { x: 1, y: 0 },
        }
    }

    fn to_command(&self) -> i64 {
        match *self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Status {
    Wall,
    Step,
    Found,
}

impl Status {
    fn parse(value: i64) -> Status {
        match value {
            0 => Status::Wall,
            1 => Status::Step,
            2 => Status::Found,
            _ => panic!("invalid status"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Wall,
    Empty,
    OxygenSystem,
}

impl Tile {
    fn print(&self) -> &'static str {
        match *self {
            Tile::Wall => "#",
            Tile::Empty => ".",
            Tile::OxygenSystem => "O",
        }
    }

    fn can_traverse(&self) -> bool {
        match *self {
            Tile::Empty | Tile::OxygenSystem => true,
            _ => false,
        }
    }
}

fn part1(program: &Vec<i64>) -> i32 {
    let mut machine = ProgramMachine::new(program.clone(), vec![]);
    let map: RefCell<HashMap<Vector2D, Tile>> = RefCell::new(HashMap::new());

    // The droid starts on an empty square
    let start = Vector2D::zero();
    map.borrow_mut().insert(start, Tile::Empty);

    // The droid's current position
    let mut current = start;

    let (_path, cost) = dijkstra(
        &start,
        |pos| -> Vec<(Vector2D, i32)> {
            let mut map = map.borrow_mut();
            explore_neighbours(&mut machine, &mut map, &mut current, pos)
        },
        |pos| map.borrow().get(pos) == Some(&Tile::OxygenSystem),
    )
    .expect("could not find oxygen system");

    print_map(&map.borrow(), &current);

    cost
}

fn get_neighbours(position: Vector2D) -> Vec<Vector2D> {
    vec![
        position + Direction::North.step(),
        position + Direction::South.step(),
        position + Direction::West.step(),
        position + Direction::East.step(),
    ]
}

fn explore_neighbours(
    machine: &mut ProgramMachine,
    map: &mut HashMap<Vector2D, Tile>,
    current: &mut Vector2D,
    pos: &Vector2D,
) -> Vec<(Vector2D, i32)> {
    get_neighbours(*pos)
        .iter()
        .filter(|neighbour| {
            if !map.contains_key(neighbour) {
                // Move to position
                go_to(machine, &map, current, *pos);
                // Explore neighbour
                let direction = Direction::from_step(**neighbour - *pos);
                let status = match step_droid(machine, current, direction) {
                    Status::Wall => Tile::Wall,
                    Status::Step => Tile::Empty,
                    Status::Found => Tile::OxygenSystem,
                };
                map.insert(**neighbour, status);
                // println!("Explored neighbour {:?} = {}", neighbour, map.get(neighbour).unwrap().print());
            }
            map.get(neighbour)
                .expect("neighbour should have been explored")
                .can_traverse()
        })
        .map(|neighbour| (*neighbour, 1))
        .collect()
}

fn go_to(
    machine: &mut ProgramMachine,
    map: &HashMap<Vector2D, Tile>,
    current: &mut Vector2D,
    dest: Vector2D,
) {
    if *current == dest {
        return;
    }

    // println!("Go from {:?} to {:?}", current, dest);
    let start = current.clone();
    let (path, _) = astar(
        &start,
        |pos| -> Vec<(Vector2D, i32)> {
            get_neighbours(*pos)
                .iter()
                .filter(|x| map.get(&x).unwrap_or(&Tile::Wall).can_traverse())
                .map(|x| (*x, 1))
                .collect()
        },
        |pos| (*pos - dest).manhattan_distance(),
        |pos| *pos == dest,
    )
    .expect("could not find path");

    debug_assert_eq!(path[0], start);
    for pos in path.iter().skip(1) {
        let direction = Direction::from_step(*pos - *current);
        let status = step_droid(machine, current, direction);
        assert_ne!(status, Status::Wall);
        *current = *pos;
    }
    assert_eq!(*current, dest);
}

fn step_droid(
    machine: &mut ProgramMachine,
    current: &mut Vector2D,
    direction: Direction,
) -> Status {
    machine.add_input(direction.to_command());
    let status = Status::parse(machine.run_to_output().unwrap());
    if status != Status::Wall {
        *current += direction.step();
    }
    status
}

fn print_map(map: &HashMap<Vector2D, Tile>, droid_position: &Vector2D) {
    let min_x = map.keys().min_by_key(|pos| pos.x).unwrap().x;
    let min_y = map.keys().min_by_key(|pos| pos.y).unwrap().y;
    let max_x = map.keys().max_by_key(|pos| pos.x).unwrap().x;
    let max_y = map.keys().max_by_key(|pos| pos.y).unwrap().y;
    for y in (min_y..=max_y).rev() {
        let mut line = String::new();
        for x in min_x..=max_x {
            let pos = Vector2D::new(x, y);
            if &pos == droid_position {
                line.push('D');
            } else {
                line.push_str(match map.get(&pos) {
                    Some(tile) => tile.print(),
                    None => " ",
                });
            }
        }
        println!("{}", line);
    }
}

fn part2(program: &Vec<i64>) -> i32 {
    let mut machine = ProgramMachine::new(program.clone(), vec![]);
    let mut map: HashMap<Vector2D, Tile> = HashMap::new();

    // The droid starts on an empty square
    let start = Vector2D::zero();
    map.insert(start, Tile::Empty);

    // The droid's current position
    let mut current = start;

    // Explore the entire map
    dijkstra_all(&start, |pos| -> Vec<(Vector2D, i32)> {
        explore_neighbours(&mut machine, &mut map, &mut current, pos)
    });

    let (oxygen_pos, _) = map
        .iter()
        .find(|(_, tile)| **tile == Tile::OxygenSystem)
        .expect("no oxygen system found");

    // Find the distance from the oxygen system to all explorable tiles
    let distances: HashMap<Vector2D, (Vector2D, i32)> =
        dijkstra_all(oxygen_pos, |pos| -> Vec<(Vector2D, i32)> {
            get_neighbours(*pos)
                .iter()
                .filter(|neighbour| map.get(neighbour).unwrap_or(&Tile::Wall).can_traverse())
                .map(|neighbour| (*neighbour, 1))
                .collect()
        });

    // Find the position with the furthest distance from the oxygen system
    let (_, max_distance) = distances.values().max_by_key(|(_, dist)| dist).unwrap();
    *max_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {}
}
