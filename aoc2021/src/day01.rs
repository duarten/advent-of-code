use std::{
    fs::File,
    io::{self, BufRead},
};

fn count_increases(entries: &[usize]) -> usize {
    entries.windows(2).filter(|w| w[1] > w[0]).count()
}

fn count_increases_groups(entries: &[usize]) -> usize {
    let groups = entries
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<_>>();
    count_increases(&groups)
}

fn main() {
    let file = File::open("aoc2021/inputs/day01.input").unwrap();
    let xs = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect::<Vec<_>>();
    println!("part 1: {:?}", count_increases(&xs));
    println!("part 2: {:?}", count_increases_groups(&xs));
}
