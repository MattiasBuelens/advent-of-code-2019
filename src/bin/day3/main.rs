use std::cmp::Ordering;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;

fn main() {
    let paths = parse_input();
    assert_eq!(paths.len(), 2);

    let trace1 = paths[0].trace();
    let trace2 = paths[1].trace();
    let crossings = find_crossings(&trace1, &trace2);

    part1(&crossings);
    part2(&trace1, &trace2, &crossings);
}

fn parse_input() -> Vec<Path> {
    return include_str!("input")
        .trim()
        .split('\n')
        .map(|x| x.parse().expect("expected number"))
        .collect();
}

fn part1(crossings: &Vec<Position>) {
    let min_crossing = crossings
        .iter()
        .min_by(|x, y| x.compare_by_manhattan_distance(y))
        .cloned()
        .expect("expected at least one crossing");

    let answer = min_crossing.manhattan_distance();
    println!("Answer to part 1: {}", answer);
}

fn part2(trace1: &Vec<Position>, trace2: &Vec<Position>, crossings: &Vec<Position>) {
    let min_crossing = crossings
        .iter()
        .min_by(|left, right| {
            total_steps_to_reach(left, &trace1, &trace2)
                .cmp(&total_steps_to_reach(right, &trace1, &trace2))
        })
        .cloned()
        .expect("expected at least one crossing");

    let answer = total_steps_to_reach(&min_crossing, &trace1, &trace2);
    println!("Answer to part 2: {}", answer);
}

fn find_crossings(trace1: &Vec<Position>, trace2: &Vec<Position>) -> Vec<Position> {
    let trace1_set: HashSet<Position, RandomState> = HashSet::from_iter(trace1.iter().cloned());
    let trace2_set: HashSet<Position, RandomState> = HashSet::from_iter(trace2.iter().cloned());
    (&trace1_set).intersection(&trace2_set).cloned().collect()
}

fn total_steps_to_reach(pos: &Position, trace1: &Vec<Position>, trace2: &Vec<Position>) -> usize {
    steps_to_reach(pos, trace1) + steps_to_reach(pos, trace2)
}

fn steps_to_reach(pos: &Position, trace: &Vec<Position>) -> usize {
    trace.iter().position(|x| x == pos).unwrap() + 1
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn zero() -> Position {
        Position { x: 0, y: 0 }
    }

    fn manhattan_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn compare_by_manhattan_distance(&self, other: &Position) -> Ordering {
        self.manhattan_distance().cmp(&other.manhattan_distance())
    }

    fn step(&self, dir: Direction) -> Position {
        let Position { x, y } = *self;
        match dir {
            Direction::Up => Position { x, y: y + 1 },
            Direction::Down => Position { x, y: y - 1 },
            Direction::Left => Position { x: x - 1, y },
            Direction::Right => Position { x: x + 1, y },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => panic!("invalid direction {}", s),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    direction: Direction,
    steps: i32,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = Direction::from_str(&s[0..1]).expect("invalid direction");
        let steps = i32::from_str(&s[1..]).expect("invalid steps");
        Ok(Move { direction, steps })
    }
}

#[derive(Debug, Clone)]
struct Path {
    moves: Vec<Move>,
}

impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .split(',')
            .map(|x| x.parse().expect("invalid move"))
            .collect();
        Ok(Path { moves })
    }
}

impl Path {
    fn trace(&self) -> Vec<Position> {
        let mut trace = Vec::new();
        let mut pos = Position::zero();
        for mv in &self.moves {
            for _ in 0..mv.steps {
                pos = pos.step(mv.direction);
                trace.push(pos);
            }
        }
        trace
    }
}
