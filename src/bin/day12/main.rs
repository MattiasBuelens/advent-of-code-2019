use std::cmp::Ordering;
use std::collections::HashSet;

use regex::Regex;

use advent_of_code_2019::math::lcm_64;
use advent_of_code_2019::vector3d::Vector3D;

fn main() {
    let moons: Vec<Moon> = parse_input(include_str!("input"));
    println!("Answer to part 1: {}", part1(&moons));
    println!("Answer to part 2: {}", part2(&moons));
}

#[derive(Debug, Clone)]
struct Moon {
    position: Vector3D,
    velocity: Vector3D,
}

fn gravity_between(left: i32, right: i32) -> i32 {
    match left.cmp(&right) {
        Ordering::Equal => 0,
        Ordering::Less => 1,
        Ordering::Greater => -1,
    }
}

impl Moon {
    fn new(position: Vector3D) -> Moon {
        Moon {
            position,
            velocity: Vector3D::zero(),
        }
    }

    fn get_gravity(&self, other: &Moon) -> Vector3D {
        let x = gravity_between(self.position.x, other.position.x);
        let y = gravity_between(self.position.y, other.position.y);
        let z = gravity_between(self.position.z, other.position.z);
        Vector3D::new(x, y, z)
    }

    fn apply_acceleration(&mut self, acceleration: Vector3D) {
        self.velocity += acceleration
    }

    fn apply_velocity(&mut self) {
        self.position += self.velocity;
    }

    fn total_energy(&self) -> i32 {
        let potential_energy = self.position.manhattan_distance();
        let kinetic_energy = self.velocity.manhattan_distance();
        potential_energy * kinetic_energy
    }

    fn axis(&self, axis: &Axis) -> MoonAxis {
        match axis {
            Axis::X => MoonAxis::new(self.position.x, self.velocity.x),
            Axis::Y => MoonAxis::new(self.position.y, self.velocity.y),
            Axis::Z => MoonAxis::new(self.position.z, self.velocity.z),
        }
    }
}

enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct MoonAxis {
    position: i32,
    velocity: i32,
}

impl MoonAxis {
    fn new(position: i32, velocity: i32) -> MoonAxis {
        MoonAxis { position, velocity }
    }
}

fn get_moon_axis(moons: &Vec<Moon>, axis: Axis) -> Vec<MoonAxis> {
    moons.iter().map(|moon| moon.axis(&axis)).collect()
}

fn parse_input(input: &str) -> Vec<Moon> {
    return input
        .trim()
        .split('\n')
        .map(parse_vector3d)
        .map(Moon::new)
        .collect();
}

fn parse_vector3d(s: &str) -> Vector3D {
    let re = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
    let captures = re.captures(s).unwrap();
    let x = captures.get(1).unwrap().as_str().parse().unwrap();
    let y = captures.get(2).unwrap().as_str().parse().unwrap();
    let z = captures.get(3).unwrap().as_str().parse().unwrap();
    Vector3D { x, y, z }
}

fn simulate(moons: &mut Vec<Moon>, steps: usize) {
    for _step in 0..steps {
        simulate_step(moons);
    }
}

fn simulate_step(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        let (head, tail) = moons.split_at_mut(i + 1);
        let left = head.last_mut().unwrap();
        for right in tail {
            let gravity = left.get_gravity(&right);
            left.apply_acceleration(gravity);
            right.apply_acceleration(-gravity);
        }
    }
    for moon in moons.iter_mut() {
        moon.apply_velocity();
    }
}

fn total_energy(moons: &Vec<Moon>) -> i32 {
    moons.iter().map(Moon::total_energy).sum()
}

fn part1(moons: &Vec<Moon>) -> i32 {
    let mut moons = moons.clone();
    simulate(&mut moons, 1000);
    total_energy(&moons)
}

fn part2(moons: &Vec<Moon>) -> i64 {
    let mut moons = moons.clone();

    // each axis (x, y, z) is independent, so look for repeats in the state of each moon
    // along each separate axis
    let mut seen_x: HashSet<Vec<MoonAxis>> = HashSet::new();
    let mut seen_y: HashSet<Vec<MoonAxis>> = HashSet::new();
    let mut seen_z: HashSet<Vec<MoonAxis>> = HashSet::new();

    let mut step: i64 = 0;
    let mut repeat_x = 0;
    let mut repeat_y = 0;
    let mut repeat_z = 0;
    while repeat_x == 0 || repeat_y == 0 || repeat_z == 0 {
        if repeat_x == 0 {
            let x = get_moon_axis(&moons, Axis::X);
            if seen_x.contains(&x) {
                repeat_x = step;
            } else {
                seen_x.insert(x);
            }
        }
        if repeat_y == 0 {
            let y = get_moon_axis(&moons, Axis::Y);
            if seen_y.contains(&y) {
                repeat_y = step;
            } else {
                seen_y.insert(y);
            }
        }
        if repeat_z == 0 {
            let z = get_moon_axis(&moons, Axis::Z);
            if seen_z.contains(&z) {
                repeat_z = step;
            } else {
                seen_z.insert(z);
            }
        }
        simulate_step(&mut moons);
        step += 1;
    }

    // the first repeat is the least common multiple of the first repeat along each axis
    lcm_64(repeat_x, lcm_64(repeat_y, repeat_z))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let mut moons: Vec<Moon> = parse_input(include_str!("example1"));
        simulate(&mut moons, 10);
        assert_eq!(total_energy(&moons), 179);
    }

    #[test]
    fn test_part1_example2() {
        let mut moons: Vec<Moon> = parse_input(include_str!("example2"));
        simulate(&mut moons, 100);
        assert_eq!(total_energy(&moons), 1940);
    }

    #[test]
    fn test_part2_example1() {
        let moons: Vec<Moon> = parse_input(include_str!("example1"));
        assert_eq!(part2(&moons), 2772);
    }

    #[test]
    fn test_part2_example2() {
        let moons: Vec<Moon> = parse_input(include_str!("example2"));
        assert_eq!(part2(&moons), 4_686_774_924);
    }
}
