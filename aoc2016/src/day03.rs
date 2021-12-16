use std::{
    fs::File,
    io::{self, BufRead},
};

fn possible(ts: &[usize]) -> bool {
    ts[0] + ts[1] > ts[2] && ts[0] + ts[2] > ts[1] && ts[1] + ts[2] > ts[0]
}

fn main() {
    let file = File::open("aoc2016/inputs/day03.input").unwrap();
    let input = io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();
    println!("part 1: {}", input.iter().filter(|ts| possible(ts)).count());
    let input = input
        .iter()
        .map(|t| t[0])
        .chain(input.iter().map(|t| t[1]))
        .chain(input.iter().map(|t| t[2]))
        .collect::<Vec<usize>>();
    println!(
        "part 2: {}",
        input.chunks(3).filter(|ts| possible(ts)).count()
    );
}
