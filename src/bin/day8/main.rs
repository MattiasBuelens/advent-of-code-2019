fn main() {
    let input: Vec<u8> = parse_input(include_str!("input"));
    let width = 25;
    let height = 6;
    println!("Answer to part 1: {}", part1(&input, width, height));
    println!("Answer to part 2:");
    print_image(&part2(&input, width, height));
}

fn parse_input(input: &str) -> Vec<u8> {
    return input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).expect("invalid input") as u8)
        .collect();
}

fn part1(input: &Vec<u8>, width: usize, height: usize) -> usize {
    let layers = input.chunks_exact(width * height);
    let min_layer = layers
        .min_by_key(|layer| layer.iter().filter(|x| **x == 0).count())
        .expect("expected at least one layer");
    let one_digits = min_layer.iter().filter(|x| **x == 1).count();
    let two_digits = min_layer.iter().filter(|x| **x == 2).count();
    one_digits * two_digits
}

fn part2(input: &Vec<u8>, width: usize, height: usize) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = Vec::with_capacity(height);
    for y in 0..height {
        let mut row: Vec<char> = Vec::with_capacity(width);
        for x in 0..width {
            let layer = input
                .chunks_exact(width * height)
                .find(|layer| layer[y * width + x] != 2)
                .expect(&format!("missing color at {},{}", x, y));
            row.push(if layer[y * width + x] == 1 { '#' } else { ' ' });
        }
        output.push(row);
    }
    output
}

fn print_image(image: &Vec<Vec<char>>) {
    for line in image {
        println!("{}", line.iter().collect::<String>())
    }
}
