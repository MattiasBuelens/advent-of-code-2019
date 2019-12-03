use std::str::FromStr;

fn main() {
    let input = parse_input();
    println!("{:?}", input);
}

fn parse_input() -> Vec<Path> {
    return include_str!("input")
        .trim()
        .split('\n')
        .map(|x| x.parse().expect("expected number"))
        .collect();
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
