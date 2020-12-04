use std::fs::File;
use std::io::{self, BufRead};

fn check_slope(trees: &Vec<String>, right: usize, down: usize) -> usize {
    trees
        .iter()
        .enumerate()
        .filter(|(i, ts)| {
            i % down == 0 && ts.chars().nth(((i / down) * right) % ts.len()).unwrap() == '#'
        })
        .count()
}

fn main() {
    let file = File::open("inputs/day03.input").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    let res = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(r, d)| check_slope(&lines, *r, *d))
        .fold(1, |acc, x| acc * x);
    println!("{}", res)
}
