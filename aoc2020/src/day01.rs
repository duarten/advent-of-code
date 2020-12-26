use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn find_entries(entries: &HashSet<i32>) -> Option<i32> {
    entries
        .iter()
        .find_map(|x| entries.get(&(2020 - x)).map(|y| y * x))
}

fn find_entries_trip(entries: &HashSet<i32>) -> Option<i32> {
    entries.iter().find_map(|x| {
        entries
            .iter()
            .filter(|&y| y != x)
            .find_map(|y| entries.get(&(2020 - x - y)).map(|z| x * y * z))
    })
}

fn main() {
    let file = File::open("aoc2020/inputs/day01.input").unwrap();
    let xs: HashSet<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    let r = find_entries(&xs);
    println!("{:?}", r);
    let r = find_entries_trip(&xs);
    println!("{:?}", r);
}
