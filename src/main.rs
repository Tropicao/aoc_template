use std::fs;
mod implementation;
use implementation::*;

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt not found");
    println!("Part 1 answer : {}", part_1(&input));
    println!("Part 2 answer : {}", part_2(&input));
}
