use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("aoc2022/inputs/day01.input").unwrap();
    let mut sums = Vec::new();
    let mut current = 0;
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        if line.is_empty() {
            sums.push(current);
            current = 0;
        } else {
            current += line.parse::<usize>().unwrap();
        }
    }
    sums.push(current);
    sums.sort_unstable_by(|a, b| b.cmp(a));
    println!("part 1: {:?}", sums.first().unwrap());
    println!("part 2: {:?}", sums.into_iter().take(3).sum::<usize>());
}
