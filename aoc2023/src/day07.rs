use std::{
    cmp::{Ordering, Reverse},
    collections::HashMap,
    convert::Infallible,
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

fn ranks(cards: [char; 13]) -> HashMap<char, usize> {
    cards
        .into_iter()
        .enumerate()
        .map(|(idx, a)| (a, idx))
        .collect()
}

lazy_static::lazy_static! {
    static ref RANKS_PT1: HashMap<char, usize> = ranks([
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ]);

    static ref RANKS_PT2: HashMap<char, usize> = ranks([
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ]);
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand(Vec<char>);

impl Hand {
    fn calc_type(&self, joker: bool) -> Type {
        let mut counts = HashMap::<_, usize>::new();
        let mut jokers = 0;
        for card in self.0.iter() {
            if joker && *card == 'J' {
                jokers += 1;
                continue;
            }
            *counts.entry(*card).or_default() += 1;
        }
        let mut counts = counts.into_values().collect::<Vec<_>>();
        counts.sort_unstable_by_key(|c| Reverse(*c));
        match counts.first().unwrap_or(&0) + jokers {
            5 => Type::FiveOfAKind,
            4 => Type::FourOfAKind,
            3 => {
                if counts.len() == 2 {
                    Type::FullHouse
                } else {
                    Type::ThreeOfAKind
                }
            }
            2 => {
                if counts.len() == 3 {
                    Type::TwoPairs
                } else {
                    Type::OnePair
                }
            }
            1 => Type::HighCard,
            _ => unreachable!(),
        }
    }

    fn cmp(&self, other: &Self, joker: bool) -> Ordering {
        self.calc_type(joker)
            .cmp(&other.calc_type(joker))
            .then_with(|| {
                let rank = |card| {
                    if joker {
                        RANKS_PT2[card]
                    } else {
                        RANKS_PT1[card]
                    }
                };
                for (card, other_card) in self.0.iter().zip(other.0.iter()) {
                    let res = rank(card).cmp(&rank(other_card));
                    if res != Ordering::Equal {
                        return res;
                    }
                }
                Ordering::Equal
            })
    }
}

impl FromStr for Hand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand(s.chars().collect::<Vec<_>>()))
    }
}

fn winnings(input: &[(Hand, usize)]) -> usize {
    input
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, (_, bid))| acc + (idx + 1) * bid)
}

fn main() {
    let file = File::open("aoc2023/inputs/day07.input").unwrap();
    let mut input = io::BufReader::new(file)
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let (hand, bid) = l.split_once(' ').unwrap();
            (hand.parse::<Hand>().unwrap(), bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    input.sort_unstable_by(|(a, _), (b, _)| a.cmp(b, false));
    println!("part 1: {}", winnings(&input));
    input.sort_unstable_by(|(a, _), (b, _)| a.cmp(b, true));
    println!("part 2: {}", winnings(&input));
}
