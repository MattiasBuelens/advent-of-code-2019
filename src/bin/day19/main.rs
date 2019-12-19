use std::collections::HashSet;

use advent_of_code_2019::input::parse_list;
use advent_of_code_2019::intcode::{Machine, ProgramMachine};
use advent_of_code_2019::vector2d::Vector2D;

fn main() {
    let program: Vec<i64> = parse_list(include_str!("input"), ',');
    println!("Answer to part 1: {}", part1(&program));
    println!("Answer to part 2: {}", part2(&program));
}

type Beam = HashSet<Vector2D>;

fn part1(program: &Vec<i64>) -> usize {
    let mut beam: Beam = HashSet::new();
    for y in 0..50 {
        for x in 0..50 {
            let pos = Vector2D::new(x, y);
            if in_beam(&program, &pos) {
                beam.insert(pos);
            }
        }
    }
    beam.len()
}

fn in_beam(program: &Vec<i64>, pos: &Vector2D) -> bool {
    let mut machine = ProgramMachine::new(program.clone(), vec![]);
    machine.add_input(pos.x as i64);
    machine.add_input(pos.y as i64);
    match machine.run_to_output() {
        Some(1) => true,
        Some(0) => false,
        output => panic!("unexpected output {:?}", output),
    }
}

fn print_beam(beam: &Beam) {
    if beam.is_empty() {
        return;
    }
    let min_x = beam.iter().min_by_key(|pos| pos.x).unwrap().x;
    let min_y = beam.iter().min_by_key(|pos| pos.y).unwrap().y;
    let max_x = beam.iter().max_by_key(|pos| pos.x).unwrap().x;
    let max_y = beam.iter().max_by_key(|pos| pos.y).unwrap().y;
    for y in min_y..=max_y {
        let mut line = String::new();
        for x in min_x..=max_x {
            let pos = Vector2D::new(x, y);
            if beam.contains(&pos) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
}

fn part2(program: &Vec<i64>) -> i32 {
    let size = 100;
    let mut beam: Beam = HashSet::new();
    let mut min_x = 0;
    let mut min_y = 0;
    let mut corner_br = None;
    'outer: for limit in 1.. {
        for x in min_x..=limit {
            let pos = Vector2D::new(x, limit);
            if in_beam(&program, &pos) {
                break;
            } else {
                min_x += 1;
            }
        }
        for x in min_x..=limit {
            let pos = Vector2D::new(x, limit);
            if in_beam(&program, &pos) {
                beam.insert(pos);
                if fits_square(&beam, size, &pos) {
                    corner_br = Some(pos);
                    break 'outer;
                }
            }
        }
        for y in min_y..limit {
            let pos = Vector2D::new(limit, y);
            if in_beam(&program, &pos) {
                break;
            } else {
                min_y += 1;
            }
        }
        for y in min_y..limit {
            let pos = Vector2D::new(limit, y);
            if in_beam(&program, &pos) {
                beam.insert(pos);
                if fits_square(&beam, size, &pos) {
                    corner_br = Some(pos);
                    break 'outer;
                }
            }
        }
    }
    // print_beam(&beam);

    let corner_br = corner_br.unwrap();
    let corner_tl = corner_br - Vector2D::new(size - 1, size - 1);

    corner_tl.x * 10_000 + corner_tl.y
}

fn fits_square(beam: &Beam, size: i32, corner: &Vector2D) -> bool {
    beam.contains(corner)
        && beam.contains(&(*corner - Vector2D::new(0, size - 1)))
        && beam.contains(&(*corner - Vector2D::new(size - 1, 0)))
        && beam.contains(&(*corner - Vector2D::new(size - 1, size - 1)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {}
}
