use std::fs::read_to_string;

use day_01::{part1, part2};

fn main() {
    println!("Testing part 1");
    let input = read_to_string("input.txt").unwrap();
    let zero_count = part1(&input).unwrap();
    println!("The output for Part 1 is: {zero_count}");

    println!("Testing part 2");
    let zero_count = part2(&input).unwrap();
    println!("The output for Part 2 is: {zero_count}");
}
