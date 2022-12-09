use std::{
    collections::HashSet,
    fs::{self},
};

fn unique(it: &[char], count: usize) -> usize {
    count
        + it.windows(count)
            .enumerate()
            .find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == count)
            .unwrap()
            .0
}

fn main() {
    let input = fs::read_to_string("aoc2022/inputs/day06.input")
        .unwrap()
        .chars()
        .collect::<Vec<_>>();
    println!("part 1: {}", unique(&input, 4));
    println!("part 2: {}", unique(&input, 14));
}
