use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug)]
struct Policy {
    letter: char,
    min: usize,
    max: usize,
    password: String,
}

impl Policy {
    fn is_valid1(&self) -> bool {
        let cnt = self.password.chars().filter(|&c| c == self.letter).count();
        cnt <= self.max && cnt >= self.min
    }

    fn matches(&self, pos: usize) -> bool {
        pos <= self.password.len() && self.password.chars().nth(pos - 1).unwrap() == self.letter
    }

    fn is_valid2(&self) -> bool {
        self.matches(self.min) != self.matches(self.max)
    }
}

impl FromStr for Policy {
    type Err = std::boxed::Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        let min_max: Vec<usize> = parts[0]
            .split('-')
            .map(|s| usize::from_str(s).unwrap())
            .collect();
        Ok(Policy {
            min: min_max[0],
            max: min_max[1],
            letter: char::from_str(&parts[1][..1]).unwrap(),
            password: parts[2].to_owned(),
        })
    }
}

fn main() {
    let file = File::open("aoc2020/inputs/day02.input").unwrap();
    let xs: Vec<Policy> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    let valid = xs.iter().filter(|&p| p.is_valid1()).count();
    println!("policy 1: {:?}", valid);
    let valid = xs.iter().filter(|&p| p.is_valid2()).count();
    println!("policy 2: {:?}", valid);
}
