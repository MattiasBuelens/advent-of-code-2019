use std::collections::HashSet;
use std::f64::consts::PI;

use advent_of_code_2019::vector2d::Vector2D;

fn main() {
    let grid: Grid = parse_input(include_str!("input"));
    println!("Answer to part 1: {}", part1(&grid));
    println!("Answer to part 2: {}", part2(&grid));
}

type Grid = HashSet<Vector2D>;

fn parse_input(input: &str) -> Grid {
    let mut grid = HashSet::new();
    let mut y = 0;
    for line in input.trim().lines() {
        let mut x = 0;
        for cell in line.chars() {
            if cell == '#' {
                grid.insert(Vector2D::new(x, y));
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

fn get_visible_asteroids(center: &Vector2D, grid: &Grid) -> HashSet<Vector2D> {
    let center = *center;
    let mut visible: HashSet<Vector2D> = HashSet::new();
    for other in grid {
        let other = *other;
        if center == other {
            continue;
        }
        let delta = other - center;
        let div = gcd(delta.x, delta.y);
        let step = Vector2D::new(delta.x / div, delta.y / div);
        let mut pos = center + step;
        while !grid.contains(&pos) {
            pos += step;
        }
        visible.insert(pos);
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
        let mut targets: Vec<Vector2D> = get_visible_asteroids(&station, &grid)
            .iter()
            .cloned()
            .collect();
        assert!(!targets.is_empty(), "ran out of targets");
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

fn get_angle(station: &Vector2D, pos: &Vector2D) -> f64 {
    let x = (pos.x - station.x) as f64;
    let y = (pos.y - station.y) as f64;
    let angle = x.atan2(-y);
    if angle < 0.0 {
        angle + (2.0 * PI)
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
        let zero = Vector2D::new(0, 0);
        assert_eq!(get_angle(&zero, &(Vector2D::new(0, -1))), 0.0);
        assert_eq!(get_angle(&zero, &(Vector2D::new(1, 0))), FRAC_PI_2);
        assert_eq!(get_angle(&zero, &(Vector2D::new(0, 1))), PI);
        assert_eq!(get_angle(&zero, &(Vector2D::new(-1, 0))), PI + FRAC_PI_2);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(include_str!("example1"))), 8);
        assert_eq!(part1(&parse_input(include_str!("example2"))), 33);
        assert_eq!(part1(&parse_input(include_str!("example3"))), 35);
        assert_eq!(part1(&parse_input(include_str!("example4"))), 41);
        assert_eq!(part1(&parse_input(include_str!("example5"))), 210);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(include_str!("example5"))), 802);
    }
}
