use std::collections::HashSet;
use std::f64::consts::PI;

fn main() {
    let grid: Grid = parse_input(include_str!("input"));
    println!("Answer to part 1: {}", part1(&grid));
    println!("Answer to part 2: {}", part2(&grid));
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

fn part1(grid: &Grid) -> usize {
    grid.iter()
        .map(|pos| get_visible_asteroids(&pos, grid).len())
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
        while x != other.x || y != other.y {
            let pos = Position { x, y };
            if grid.contains(&pos) {
                visible.insert(pos);
                continue 'outer;
            }
            x += step_x;
            y += step_y;
        }
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

fn part2(grid: &Grid) -> i32 {
    let mut grid = grid.clone();
    let station = grid
        .iter()
        .max_by_key(|pos| get_visible_asteroids(&pos, &grid).len())
        .unwrap()
        .clone();
    let mut destroyed = 0usize;
    loop {
        let mut targets: Vec<Position> = get_visible_asteroids(&station, &grid)
            .iter()
            .cloned()
            .collect();
        targets.sort_by(|a, b| {
            get_angle(&station, a)
                .partial_cmp(&get_angle(&station, b))
                .unwrap()
        });
        for target in targets {
            grid.remove(&target);
            destroyed += 1;
            if destroyed == 200 {
                return target.x * 100 + target.y;
            }
        }
    }
}

fn get_angle(station: &Position, pos: &Position) -> f64 {
    let x = (pos.x - station.x) as f64;
    let y = (pos.y - station.y) as f64;
    let angle = x.atan2(-y);
    if angle < 0f64 {
        angle + (2f64 * PI)
    } else {
        angle
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::*;

    use super::*;

    #[test]
    fn test_get_angle() {
        let zero = Position { x: 0, y: 0 };
        assert_eq!(get_angle(&zero, &(Position { x: 0, y: -1 })), 0f64);
        assert_eq!(get_angle(&zero, &(Position { x: 1, y: 0 })), FRAC_PI_2);
        assert_eq!(get_angle(&zero, &(Position { x: 0, y: 1 })), PI);
        assert_eq!(
            get_angle(&zero, &(Position { x: -1, y: 0 })),
            PI + FRAC_PI_2
        );
    }

    #[test]
    fn test_part1() {
        // TODO
    }
}
