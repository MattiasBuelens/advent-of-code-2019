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
    amount: i64,
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

fn part1(input: &Vec<Reaction>) -> i64 {
    let reactions = reactions_by_output(input.clone());
    ore_needed_for_fuel(1, &reactions)
}

fn ore_needed_for_fuel(fuel_amount: i64, reactions: &HashMap<String, Reaction>) -> i64 {
    let mut stock: HashMap<String, i64> = HashMap::new();
    let mut ore = 0;
    produce(
        &"FUEL".to_string(),
        fuel_amount,
        &reactions,
        &mut stock,
        &mut ore,
    );
    ore
}

fn reactions_by_output(input: Vec<Reaction>) -> HashMap<String, Reaction> {
    let mut reactions: HashMap<String, Reaction> = HashMap::new();
    for reaction in input {
        debug_assert!(!reactions.contains_key(&reaction.output.chemical));
        reactions.insert(reaction.output.chemical.clone(), reaction);
    }
    reactions
}

fn produce(
    chemical: &String,
    amount: i64,
    reactions: &HashMap<String, Reaction>,
    stock: &mut HashMap<String, i64>,
    ore: &mut i64,
) {
    if chemical.as_str() == "ORE" {
        // Produce ore
        *stock.entry(chemical.clone()).or_default() += amount;
        *ore += amount;
    } else {
        let reaction = reactions.get(chemical).unwrap();
        // Repeat reaction to produce at least $amount outputs
        let mut repeats = amount / reaction.output.amount;
        if amount % reaction.output.amount != 0 {
            repeats += 1;
        };
        // Consume inputs
        for input in &reaction.inputs {
            let needed = input.amount * repeats;
            let in_stock = *stock.get(&input.chemical.clone()).unwrap_or(&0);
            if needed > in_stock {
                produce(&input.chemical, needed - in_stock, reactions, stock, ore);
            }
            consume_stock(&input.chemical, needed, stock);
        }
        // Produce outputs
        *stock.entry(reaction.output.chemical.clone()).or_default() +=
            reaction.output.amount * repeats;
    }
}

fn consume_stock(chemical: &String, amount: i64, stock: &mut HashMap<String, i64>) {
    let stock_amount = stock.get_mut(chemical).unwrap();
    debug_assert!(*stock_amount >= amount);
    *stock_amount -= amount;
}

fn part2(input: &Vec<Reaction>) -> i64 {
    let reactions = reactions_by_output(input.clone());
    let ore_available: i64 = 1_000_000_000_000;

    // Find upper bound
    let mut upper_bound: i64 = 1;
    while ore_needed_for_fuel(upper_bound, &reactions) < ore_available {
        upper_bound *= 2;
    }

    // Binary search for fuel amount that is just below available ore amount
    let mut low = upper_bound / 2;
    let mut high = upper_bound;
    while low <= high {
        let mid = low + (high - low) / 2;
        let ore_needed = ore_needed_for_fuel(mid, &reactions);
        if ore_needed > ore_available {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    debug_assert_eq!(low, high + 1);

    let max_fuel = high;
    debug_assert!(ore_needed_for_fuel(max_fuel, &reactions) < ore_available);
    debug_assert!(ore_needed_for_fuel(max_fuel + 1, &reactions) > ore_available);
    max_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input: Vec<Reaction> = parse_list(include_str!("example1"), '\n');
        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn test_part1_example2() {
        let input: Vec<Reaction> = parse_list(include_str!("example2"), '\n');
        assert_eq!(part1(&input), 165);
    }

    #[test]
    fn test_part1_example3() {
        let input: Vec<Reaction> = parse_list(include_str!("example3"), '\n');
        assert_eq!(part1(&input), 13312);
    }

    #[test]
    fn test_part1_example4() {
        let input: Vec<Reaction> = parse_list(include_str!("example4"), '\n');
        assert_eq!(part1(&input), 180697);
    }

    #[test]
    fn test_part1_example5() {
        let input: Vec<Reaction> = parse_list(include_str!("example5"), '\n');
        assert_eq!(part1(&input), 2210736);
    }

    #[test]
    fn test_part2_example3() {
        let input: Vec<Reaction> = parse_list(include_str!("example3"), '\n');
        assert_eq!(part2(&input), 82892753);
    }

    #[test]
    fn test_part2_example4() {
        let input: Vec<Reaction> = parse_list(include_str!("example4"), '\n');
        assert_eq!(part2(&input), 5586022);
    }

    #[test]
    fn test_part2_example5() {
        let input: Vec<Reaction> = parse_list(include_str!("example5"), '\n');
        assert_eq!(part2(&input), 460664);
    }
}
