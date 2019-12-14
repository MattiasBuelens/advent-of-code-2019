use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

use advent_of_code_2019::input::parse_list;

fn main() {
    let input: Vec<Reaction> = parse_list(include_str!("input"), '\n');
    println!("Answer to part 1: {}", part1(&input));
    println!("Answer to part 2: {}", part2(&input));
}

#[derive(Debug, Clone)]
struct Quantity {
    amount: i32,
    chemical: String,
}

impl FromStr for Quantity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        Ok(Quantity {
            amount: parts[0].parse().unwrap(),
            chemical: parts[1].to_string(),
        })
    }
}

impl Display for Quantity {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{} {}", self.amount, self.chemical)
    }
}

#[derive(Debug, Clone)]
struct Reaction {
    inputs: Vec<Quantity>,
    output: Quantity,
}

impl FromStr for Reaction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" => ").collect();
        Ok(Reaction {
            inputs: parts[0].split(", ").map(|x| x.parse().unwrap()).collect(),
            output: parts[1].parse().unwrap(),
        })
    }
}

impl Display for Reaction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.inputs[0].fmt(f)?;
        for input in self.inputs.iter().skip(1) {
            write!(f, ", ")?;
            input.fmt(f)?;
        }
        write!(f, " => ")?;
        self.output.fmt(f)?;
        Ok(())
    }
}

fn part1(input: &Vec<Reaction>) -> i32 {
    let mut reactions = reactions_by_output(input.clone());
    let mut stock: HashMap<String, i32> = HashMap::new();
    let mut ore = 0;
    produce_one(&"FUEL".to_string(), &mut reactions, &mut stock, &mut ore);
    ore
}

fn reactions_by_output(input: Vec<Reaction>) -> HashMap<String, Reaction> {
    let mut reactions: HashMap<String, Reaction> = HashMap::new();
    for reaction in input {
        assert!(!reactions.contains_key(&reaction.output.chemical));
        reactions.insert(reaction.output.chemical.clone(), reaction);
    }
    reactions
}

fn produce_one(
    chemical: &String,
    reactions: &HashMap<String, Reaction>,
    stock: &mut HashMap<String, i32>,
    ore: &mut i32,
) {
    if chemical.as_str() == "ORE" {
        // Produce ore
        *stock.entry(chemical.clone()).or_default() += 1;
        *ore += 1;
    } else {
        let reaction = reactions.get(chemical).unwrap();
        // Consume inputs
        for input in &reaction.inputs {
            for _ in 0..input.amount {
                if !has_stock(&input.chemical, stock) {
                    produce_one(&input.chemical, reactions, stock, ore);
                }
                consume_stock(&input.chemical, stock);
            }
        }
        // Produce outputs
        *stock.entry(reaction.output.chemical.clone()).or_default() += reaction.output.amount;
    }
}

fn has_stock(chemical: &String, stock: &HashMap<String, i32>) -> bool {
    *stock.get(chemical).unwrap_or(&0) > 0
}

fn consume_stock(chemical: &String, stock: &mut HashMap<String, i32>) {
    assert!(has_stock(chemical, stock));
    *stock.get_mut(chemical).unwrap() -= 1;
}

fn part2(input: &Vec<Reaction>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {}
}
