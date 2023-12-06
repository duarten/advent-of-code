use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("aoc2023/inputs/day06.input").unwrap();
    let mut input = io::BufReader::new(file).lines().map(|l| {
        let l = l.unwrap();
        let (_, values) = l.split_once(": ").unwrap();
        values
            .split_ascii_whitespace()
            .flat_map(|x| x.parse::<usize>())
            .collect::<Vec<_>>()
    });
    let times = input.next().unwrap();
    let records = input.next().unwrap();
    let races = times.iter().zip(records.iter()).map(|(t, r)| race(*t, *r));
    println!("part 1: {}", races.product::<usize>());
    println!("part 2: {}", race(join(times), join(records)));
}

fn join(values: Vec<usize>) -> usize {
    let joined = values.into_iter().fold(String::new(), |mut acc, x| {
        acc.push_str(&x.to_string());
        acc
    });
    joined.parse::<usize>().unwrap()
}

fn race(time: usize, record: usize) -> usize {
    (1..time).map(|p| ((time - p) * p > record) as usize).sum()
}
