fn main() {
    let input: Vec<Color> = parse_input(include_str!("input"));
    let width = 25;
    let height = 6;
    println!("Answer to part 1: {}", part1(&input, width, height));
    println!("Answer to part 2:");
    print_image(&part2(&input, width, height));
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Color {
    BLACK,
    WHITE,
    TRANSPARENT,
}

impl Color {
    fn parse(digit: u32) -> Color {
        match digit {
            0 => Color::BLACK,
            1 => Color::WHITE,
            2 => Color::TRANSPARENT,
            _ => panic!("invalid color {}", digit),
        }
    }

    fn print(&self) -> char {
        match *self {
            Color::WHITE => '#',
            _ => ' ',
        }
    }
}

fn parse_input(input: &str) -> Vec<Color> {
    return input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).expect("invalid input"))
        .map(Color::parse)
        .collect();
}

fn part1(input: &Vec<Color>, width: usize, height: usize) -> usize {
    let layers = input.chunks_exact(width * height);
    let min_layer = layers
        .min_by_key(|layer| layer.iter().filter(|&x| x == &Color::BLACK).count())
        .expect("expected at least one layer");
    let one_digits = min_layer.iter().filter(|&x| x == &Color::WHITE).count();
    let two_digits = min_layer
        .iter()
        .filter(|&x| x == &Color::TRANSPARENT)
        .count();
    one_digits * two_digits
}

fn part2(input: &Vec<Color>, width: usize, height: usize) -> Vec<Vec<Color>> {
    let mut output: Vec<Vec<Color>> = Vec::with_capacity(height);
    for y in 0..height {
        let mut row: Vec<Color> = Vec::with_capacity(width);
        for x in 0..width {
            let color = input
                .chunks_exact(width * height)
                .map(|layer| layer[y * width + x])
                .find(|&color| color != Color::TRANSPARENT)
                .expect(&format!("missing color at {},{}", x, y));
            row.push(color);
        }
        output.push(row);
    }
    output
}

fn print_image(image: &Vec<Vec<Color>>) {
    for line in image {
        println!("{}", line.iter().map(Color::print).collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse_input("0222112222120000"), 2, 2),
            vec![
                vec![Color::BLACK, Color::WHITE],
                vec![Color::WHITE, Color::BLACK]
            ]
        );
    }
}
