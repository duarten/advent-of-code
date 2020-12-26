use anyhow::Result;
use std::collections::{HashSet, VecDeque};
use std::fs;
use std::str::FromStr;

type Card = usize;

#[derive(Clone, Debug)]
struct Deck {
    player: usize,
    cards: VecDeque<Card>,
}

impl Deck {
    fn len(&self) -> usize {
        self.cards.len()
    }

    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn draw(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    fn keep(&mut self, mine: Card, other: Card) {
        self.cards.push_back(mine);
        self.cards.push_back(other);
    }

    fn score(&self) -> usize {
        self.cards
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, c)| acc + c * (i + 1))
    }

    fn copy_limit(&self, limit: usize) -> Self {
        Self {
            cards: self.cards.iter().copied().take(limit).collect(),
            player: self.player,
        }
    }
}

impl FromStr for Deck {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Deck {
            cards: s.lines().skip(1).map(|s| s.parse().unwrap()).collect(),
            player: 0,
        })
    }
}

fn combat(mut d1: Deck, mut d2: Deck) -> Deck {
    while !d1.is_empty() && !d2.is_empty() {
        let c1 = d1.draw().unwrap();
        let c2 = d2.draw().unwrap();
        if c1 > c2 {
            d1.keep(c1, c2);
        } else {
            d2.keep(c2, c1);
        }
    }
    if d1.is_empty() {
        d2
    } else {
        d1
    }
}

fn recursive_combat(mut d1: Deck, mut d2: Deck) -> Deck {
    let mut memo = HashSet::<(usize, usize)>::new();
    while !d1.is_empty() && !d2.is_empty() {
        if !memo.insert((d1.score(), d2.score())) {
            return d1;
        }
        let c1 = d1.draw().unwrap();
        let c2 = d2.draw().unwrap();
        let c1_wins = if d1.len() >= c1 && d2.len() >= c2 {
            recursive_combat(d1.copy_limit(c1), d2.copy_limit(c2)).player == 1
        } else {
            c1 > c2
        };
        if c1_wins {
            d1.keep(c1, c2);
        } else {
            d2.keep(c2, c1);
        }
    }
    if d1.is_empty() {
        d2
    } else {
        d1
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("aoc2020/inputs/day22.input").unwrap();
    let mut decks = input
        .split("\n\n")
        .map(Deck::from_str)
        .collect::<Result<Vec<_>>>()?;
    let (l, r) = decks.split_at_mut(1);
    l[0].player = 1;
    r[0].player = 2;
    println!("part 1: {}", combat(l[0].clone(), r[0].clone()).score());
    println!(
        "part 2: {}",
        recursive_combat(l[0].clone(), r[0].clone()).score()
    );
    Ok(())
}
