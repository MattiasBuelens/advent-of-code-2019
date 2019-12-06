use std::collections::HashMap;

fn main() {
    let input = parse_input();
    println!("Answer to part 1: {}", input.total_orbits());
    println!(
        "Answer to part 2: {}",
        input.transfers_between("YOU", "SAN")
    );
}

type Orbit = (String, String);

fn parse_orbit(s: &str) -> Orbit {
    let parts: Vec<&str> = s.split(')').collect();
    (parts[0].into(), parts[1].into())
}

fn parse_input() -> OrbitMap {
    let orbits = include_str!("input")
        .trim()
        .split('\n')
        .map(parse_orbit)
        .collect();
    OrbitMap::from_orbits(orbits)
}

#[derive(Debug)]
struct OrbitMap {
    map: HashMap<String, String>,
}

impl OrbitMap {
    fn from_orbits(orbits: Vec<Orbit>) -> OrbitMap {
        let map: HashMap<String, String> = orbits
            .iter()
            .cloned()
            .map(|(center, satellite)| (satellite, center))
            .collect();
        OrbitMap { map }
    }

    fn total_orbits(&self) -> usize {
        self.map
            .keys()
            .map(|object| self.count_orbits_of(object))
            .sum()
    }

    fn count_orbits_of(&self, object: &String) -> usize {
        match self.map.get(object) {
            None => 0,
            Some(center) => 1 + self.count_orbits_of(center),
        }
    }

    fn transfers_between(&self, start: &str, target: &str) -> usize {
        let start_ancestors = self.get_ancestors(String::from(start));
        let target_ancestors = self.get_ancestors(String::from(target));
        let common_ancestor_index = start_ancestors
            .iter()
            .position(|ancestor| target_ancestors.contains(ancestor))
            .expect("expected a common ancestor");
        let common_ancestor = &start_ancestors[common_ancestor_index];
        let start_to_common = common_ancestor_index;
        let common_to_target = target_ancestors
            .iter()
            .position(|x| x == common_ancestor)
            .unwrap();
        start_to_common + common_to_target
    }

    fn get_ancestors(&self, mut object: String) -> Vec<String> {
        let mut ancestors: Vec<String> = Vec::new();
        while let Some(parent) = self.map.get(&object) {
            ancestors.push(parent.clone());
            object = parent.clone();
        }
        ancestors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ];
        let orbits: Vec<Orbit> = input.iter().map(|x| parse_orbit(x)).collect();
        let map = OrbitMap::from_orbits(orbits);

        assert_eq!(map.total_orbits(), 42);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];
        let orbits: Vec<Orbit> = input.iter().map(|x| parse_orbit(x)).collect();
        let map = OrbitMap::from_orbits(orbits);

        assert_eq!(map.transfers_between("YOU", "SAN"), 4);
    }
}
