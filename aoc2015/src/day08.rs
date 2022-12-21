use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

struct StringInfo {
    diff: usize,
    encode: usize,
}

impl FromStr for StringInfo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mem = 0;
        let mut escaping = false;
        let mut hex = 0;
        let mut encode = 2;
        for c in s.chars() {
            mem += escaping as usize;
            match c {
                '"' => {
                    encode += 1;
                    escaping = false;
                }
                '\\' => {
                    encode += 1;
                    escaping = !escaping;
                }
                'x' if escaping => {
                    escaping = false;
                    hex = 1;
                }
                d if d.is_ascii_hexdigit() && hex > 0 => {
                    hex = (hex + 1) % 3;
                }
                _ => {
                    mem += 1;
                }
            };
        }
        Ok(Self {
            encode,
            diff: s.len() - mem,
        })
    }
}

fn main() {
    let file = File::open("aoc2015/inputs/day08.input").unwrap();
    let (pt1, pt2) = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<StringInfo>().unwrap())
        .fold((0, 0), |(pt1, pt2), i| (pt1 + i.diff, pt2 + i.encode));
    println!("part 1: {}", pt1);
    println!("part 2: {}", pt2);
}
