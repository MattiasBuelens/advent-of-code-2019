use std::collections::HashSet;

fn main() {
    let input: Grid = parse_input(include_str!("input"));
    println!("Answer to part 1: {}", part1(&input));
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Position {
    x: i32,
    y: i32,
}

type Grid = HashSet<Position>;

fn parse_input(input: &str) -> Grid {
    let mut grid = HashSet::new();
    let mut y = 0;
    for line in input.trim().lines() {
        let mut x = 0;
        for cell in line.chars() {
            if cell == '#' {
                grid.insert(Position { x, y });
            }
            x += 1;
        }
        y += 1;
    }
    grid
}

fn part1(input: &Grid) -> usize {
    input
        .iter()
        .map(|pos| get_visible_asteroids(&pos, input).len())
        .max()
        .expect("expected at least one asteroid")
}

fn get_visible_asteroids(center: &Position, grid: &Grid) -> HashSet<Position> {
    let mut visible: HashSet<Position> = HashSet::new();
    'outer: for other in grid {
        if center == other {
            continue;
        }
        let dx = other.x - center.x;
        let dy = other.y - center.y;
        let div = gcd(dx, dy);
        let step_x = dx / div;
        let step_y = dy / div;
        let mut x = center.x + step_x;
        let mut y = center.y + step_y;
        //        println!("{:?} {:?} {} {}", center, other, step_x, step_y);
        while x != other.x || y != other.y {
            let pos = Position { x, y };
            //            println!("{:?}", pos);
            if grid.contains(&pos) {
                //                println!("insert pos {:?}", pos);
                visible.insert(pos);
                continue 'outer;
            }
            x += step_x;
            y += step_y;
        }
        //        println!("insert other {:?}", other);
        visible.insert(other.clone());
    }
    visible
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0 {
        let old_a = a;
        a = b % a;
        b = old_a;
    }
    b.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // TODO
    }
}
