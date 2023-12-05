use std::{
    collections::HashSet,
    convert::Infallible,
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

#[derive(Debug, Clone)]
struct Card {
    winning: HashSet<usize>,
    chosen: Vec<usize>,
}

impl Card {
    fn score(&self) -> usize {
        let matches = self.matches();
        if matches == 0 {
            0
        } else if matches == 1 {
            1
        } else {
            2_usize.pow((self.matches() - 1) as u32)
        }
    }

    fn matches(&self) -> usize {
        self.chosen
            .iter()
            .map(|n| self.winning.contains(n) as usize)
            .sum()
    }
}

impl FromStr for Card {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning, chosen) = numbers.split_once(" | ").unwrap();
        let winning = winning
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<HashSet<_>>();
        let chosen = chosen
            .split_ascii_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Ok(Self { winning, chosen })
    }
}

fn main() {
    let file = File::open("aoc2023/inputs/day04.input").unwrap();
    let cards = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<Card>().unwrap())
        .collect::<Vec<_>>();
    println!("part 1: {}", cards.iter().map(Card::score).sum::<usize>());
    let mut copies = vec![1_usize; cards.len()];
    for (idx, c) in cards.iter().enumerate() {
        let matches = c.matches();
        for i in 1..=matches {
            if idx + i < cards.len() {
                copies[idx + i] += copies[idx];
            }
        }
    }
    println!("part 2: {}", copies.into_iter().sum::<usize>());
}
