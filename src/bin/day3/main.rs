use std::cmp::Ordering;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;
use std::str::FromStr;

fn main() {
    let input = parse_input();
    assert_eq!(input.len(), 2);

    part1(&input);
    part2(&input);
}

fn parse_input() -> Vec<Path> {
    return include_str!("input")
        .trim()
        .split('\n')
        .map(|x| x.parse().expect("expected number"))
        .collect();
}

fn part1(paths: &Vec<Path>) {
    let min_crossing =
        find_closest_crossing(&paths[0], &paths[1]).expect("expected at least one crossing");

    let answer = min_crossing.manhattan_distance();
    println!("Answer to part 1: {}", answer);
}

fn find_closest_crossing(path1: &Path, path2: &Path) -> Option<Position> {
    let trace1: HashSet<Position, RandomState> = HashSet::from_iter(path1.trace());
    let trace2: HashSet<Position, RandomState> = HashSet::from_iter(path2.trace());

    let crossings = trace1.intersection(&trace2);
    crossings
        .min_by(|x, y| x.compare_by_manhattan_distance(y))
        .cloned()
}

fn part2(paths: &Vec<Path>) {
    let trace1 = paths[0].trace();
    let trace2 = paths[1].trace();

    let trace1_set: HashSet<Position, RandomState> = HashSet::from_iter(trace1.iter().cloned());
    let trace2_set: HashSet<Position, RandomState> = HashSet::from_iter(trace2.iter().cloned());
    let crossings = trace1_set.intersection(&trace2_set);

    let trace1 = Rc::new(trace1);
    let trace2 = Rc::new(trace2);
    let min_crossing = crossings
        .min_by(|left, right| {
            total_steps_to_reach(left, &trace1, &trace2)
                .cmp(&total_steps_to_reach(right, &trace1, &trace2))
        })
        .cloned()
        .expect("expected at least one crossing");

    let answer = total_steps_to_reach(&min_crossing, &trace1, &trace2);
    println!("Answer to part 2: {}", answer);
}

fn total_steps_to_reach(pos: &Position, trace1: &Vec<Position>, trace2: &Vec<Position>) -> usize {
    let index1 = trace1.iter().position(|x| x == pos).unwrap() + 1;
    let index2 = trace2.iter().position(|x| x == pos).unwrap() + 1;
    index1 + index2
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
