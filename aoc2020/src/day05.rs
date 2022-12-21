use std::{
    fs::File,
    io::{self, BufRead},
    ops::Add,
};

fn parse_id(guide: &str) -> usize {
    let row = guide[..7]
        .chars()
        .fold(0, |acc, g| (acc << 1) + (g == 'B') as usize);
    let col = guide[7..]
        .chars()
        .fold(0, |acc, g| (acc << 1) + (g == 'R') as usize);
    row * 8 + col
}

fn main() {
    let file = File::open("aoc2020/inputs/day05.input").unwrap();
    let ids = io::BufReader::new(file)
        .lines()
        .map(|l| parse_id(&l.unwrap()));
    let (highest, smallest, cnt) = ids.fold((0, usize::MAX, 0), |(max, min, sum), x| {
        (usize::max(max, x), usize::min(min, x), sum + x)
    });
    let missing = (smallest..=highest).fold(0, usize::add) - cnt;
    println!("highest seat: {}; missing: {}", highest, missing);
}
