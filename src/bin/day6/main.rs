use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn main() {
    let input = parse_input();
    println!("Answer to part 1: {}", input.total_orbits());
}

#[derive(Debug)]
struct Orbit(String, String);

impl FromStr for Orbit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(')').collect();
        Ok(Orbit(parts[0].into(), parts[1].into()))
    }
}

fn parse_input() -> OrbitMap {
    let orbits = include_str!("input")
        .trim()
        .split('\n')
        .map(|x| x.parse().expect("expected number"))
        .collect();
    OrbitMap::from_orbits(orbits)
}

#[derive(Debug)]
struct OrbitMap {
    objects: HashSet<String>,
    map: HashMap<String, String>,
}

impl OrbitMap {
    fn from_orbits(orbits: Vec<Orbit>) -> OrbitMap {
        let mut objects: HashSet<String> = HashSet::new();
        let mut map: HashMap<String, String> = HashMap::new();
        for orbit in orbits {
            objects.insert(orbit.0.clone());
            objects.insert(orbit.1.clone());
            map.insert(orbit.1, orbit.0);
        }
        OrbitMap { objects, map }
    }

    fn total_orbits(&self) -> usize {
        self.objects
            .iter()
            .map(|object| self.count_orbits_of(object))
            .sum()
    }

    fn count_orbits_of(&self, object: &String) -> usize {
        match self.map.get(object) {
            None => 0,
            Some(center) => 1 + self.count_orbits_of(center),
        }
    }
}
