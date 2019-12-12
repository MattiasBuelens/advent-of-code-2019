use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str::FromStr;

use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::vector2d::Vector2D;

fn main() {
    let paths: Vec<Path> = parse_list(include_str!("input"), '\n');
    assert_eq!(paths.len(), 2);

    let trace1 = paths[0].trace();
    let trace2 = paths[1].trace();
    let crossings = find_crossings(&trace1, &trace2);

    println!("Answer to part 1: {}", part1(&crossings));
    println!("Answer to part 2: {}", part2(&trace1, &trace2, &crossings));
}

fn part1(crossings: &Vec<Vector2D>) -> i32 {
    let min_crossing = crossings
        .iter()
        .min_by(|x, y| x.compare_by_manhattan_distance(y))
        .cloned()
        .expect("expected at least one crossing");

    min_crossing.manhattan_distance()
}

fn part2(trace1: &Vec<Vector2D>, trace2: &Vec<Vector2D>, crossings: &Vec<Vector2D>) -> usize {
    let min_crossing = crossings
        .iter()
        .min_by(|left, right| {
            total_steps_to_reach(left, &trace1, &trace2)
                .cmp(&total_steps_to_reach(right, &trace1, &trace2))
        })
        .cloned()
        .expect("expected at least one crossing");

    total_steps_to_reach(&min_crossing, &trace1, &trace2)
}

fn find_crossings(trace1: &Vec<Vector2D>, trace2: &Vec<Vector2D>) -> Vec<Vector2D> {
    let trace1_set: HashSet<Vector2D, RandomState> = HashSet::from_iter(trace1.iter().cloned());
    let trace2_set: HashSet<Vector2D, RandomState> = HashSet::from_iter(trace2.iter().cloned());
    (&trace1_set).intersection(&trace2_set).cloned().collect()
}

fn total_steps_to_reach(pos: &Vector2D, trace1: &Vec<Vector2D>, trace2: &Vec<Vector2D>) -> usize {
    steps_to_reach(pos, trace1) + steps_to_reach(pos, trace2)
}

fn steps_to_reach(pos: &Vector2D, trace: &Vec<Vector2D>) -> usize {
    trace.iter().position(|x| x == pos).unwrap() + 1
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn step(&self) -> Vector2D {
        match *self {
            Direction::Up => Vector2D { x: 0, y: 1 },
            Direction::Down => Vector2D { x: 0, y: -1 },
            Direction::Left => Vector2D { x: -1, y: 0 },
            Direction::Right => Vector2D { x: 1, y: 0 },
        }
    }
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
        let direction = s[0..1].parse::<Direction>().expect("invalid direction");
        let steps = s[1..].parse::<i32>().expect("invalid steps");
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
    fn trace(&self) -> Vec<Vector2D> {
        let mut trace = Vec::new();
        let mut pos = Vector2D::zero();
        for mv in &self.moves {
            for _ in 0..mv.steps {
                pos += mv.direction.step();
                trace.push(pos);
            }
        }
        trace
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_part1(path1: &str, path2: &str) -> i32 {
        let trace1 = path1.parse::<Path>().unwrap().trace();
        let trace2 = path2.parse::<Path>().unwrap().trace();
        let crossings = find_crossings(&trace1, &trace2);
        part1(&crossings)
    }

    fn run_part2(path1: &str, path2: &str) -> usize {
        let trace1 = path1.parse::<Path>().unwrap().trace();
        let trace2 = path2.parse::<Path>().unwrap().trace();
        let crossings = find_crossings(&trace1, &trace2);
        part2(&trace1, &trace2, &crossings)
    }

    #[test]
    fn test_part1() {
        assert_eq!(run_part1("R8,U5,L5,D3", "U7,R6,D4,L4"), 6);
        assert_eq!(
            run_part1(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83",
            ),
            159
        );
        assert_eq!(
            run_part1(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            ),
            135
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(run_part2("R8,U5,L5,D3", "U7,R6,D4,L4"), 30);
        assert_eq!(
            run_part2(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83",
            ),
            610
        );
        assert_eq!(
            run_part2(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            ),
            410
        );
    }
}
