use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;

fn parse_id(guide: String) -> Option<usize> {
    let mut row = 0;
    let mut col = 0;
    for g in guide.bytes() {
        match g {
            b'F' => row <<= 1,
            b'B' => row = (row << 1) + 1,
            b'L' => col <<= 1,
            b'R' => col = (col << 1) + 1,
            _ => return None,
        }
    }
    Some(row * 8 + col)
}

fn main() {
    let file = File::open("inputs/day05.input").unwrap();
    let ids = io::BufReader::new(file)
        .lines()
        .filter_map(|l| parse_id(l.unwrap()));
    let (highest, smallest, cnt) = ids.fold((0, usize::MAX, 0), |(max, min, sum), x| {
        (usize::max(max, x), usize::min(min, x), sum + x)
    });
    let missing = (smallest..=highest).fold(0, usize::add) - cnt;
    println!("highest seat: {}; missing: {}", highest, missing);
}
