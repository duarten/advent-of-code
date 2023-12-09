use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("aoc2023/inputs/day09.input").unwrap();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        let history = line
            .split_ascii_whitespace()
            .map(|n| n.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let (first, last) = predict(&history);
        sum1 += last;
        sum2 += first;
    }
    println!("part 1: {}", sum1);
    println!("part 2: {}", sum2);
}

fn predict(history: &[isize]) -> (isize, isize) {
    let mut delta = Vec::new();
    for (idx, value) in history[1..].iter().enumerate() {
        delta.push(value - history[idx]);
    }
    if delta.iter().all(|&d| d == 0) {
        return (*history.first().unwrap(), *history.last().unwrap());
    }
    let (first, last) = predict(&delta);
    (
        history.first().unwrap() - first,
        last + history.last().unwrap(),
    )
}
