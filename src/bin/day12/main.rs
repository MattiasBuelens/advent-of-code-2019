use std::cmp::Ordering;

use regex::Regex;

use advent_of_code_2019::vector3d::Vector3D;

fn main() {
    let moons: Vec<Moon> = parse_input(include_str!("input"));
    println!("Answer to part 1: {}", part1(&moons));
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
        for i in 0..moons.len() {
            for j in (i + 1)..moons.len() {
                let gravity = moons[i].get_gravity(&moons[j]);
                moons[i].apply_acceleration(gravity);
                moons[j].apply_acceleration(-gravity);
            }
        }
        for moon in moons.iter_mut() {
            moon.apply_velocity();
        }
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
}
