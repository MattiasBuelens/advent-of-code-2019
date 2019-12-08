fn main() {
    let input: Vec<u8> = parse_input(include_str!("input"));
    let width = 25;
    let height = 6;
    println!("Answer to part 1: {}", part1(&input, width, height));
}

fn parse_input(input: &str) -> Vec<u8>{
    return input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).expect("invalid input") as u8)
        .collect();
}

fn part1(input: &Vec<u8>, width: usize, height: usize) -> usize {
    let layers = input.chunks_exact(width * height);
    let min_layer = layers.min_by_key(|layer|{
        layer.iter().filter(|x| **x == 0).count()
    }).expect("expected at least one layer");
    let one_digits = min_layer.iter().filter(|x| **x == 1).count();
    let two_digits = min_layer.iter().filter(|x| **x == 2).count();
    one_digits * two_digits
}