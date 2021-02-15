use itertools::Itertools;

use std::fs;

fn next(prev: Vec<u8>) -> Vec<u8> {
    prev.into_iter()
        .group_by(|x| *x)
        .into_iter()
        .flat_map(|(k, g)| vec![g.count() as u8, k])
        .collect()
}

fn main() {
    let input: Vec<_> = fs::read_to_string("aoc2015/inputs/day10.input")
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    let run = |limit| (0..limit).fold(input.clone(), |acc, _| next(acc)).len();
    println!("part 1: {}", run(40));
    println!("part 2: {}", run(50));
}
