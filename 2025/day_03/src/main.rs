use std::fs::read_to_string;

use day_03::{part1, part2};

fn main() {
    let input = read_to_string("input.txt").unwrap();
    println!("Testing part 1");
    let result = part1(&input).unwrap();
    println!("The output for Part 1 is: {result}");
    println!("Testing part 2");
    let result = part2(&input).unwrap();
    println!("The output for Part 2 is: {result}");
}
