use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Player {
    pawn: usize,
    score: usize,
}

impl Player {
    fn new(starting: usize) -> Self {
        Self {
            pawn: starting,
            score: 0,
        }
    }

    fn roll(&mut self, roll: usize) -> usize {
        self.pawn = (self.pawn + roll - 1) % 10 + 1;
        self.score += self.pawn;
        self.score
    }
}

fn play1(s1: usize, s2: usize) -> usize {
    let mut p1 = Player::new(s1);
    let mut p2 = Player::new(s2);
    let mut die = 1;
    let mut rolls = 0;
    let mut roll = || {
        rolls += 1;
        let ndie = die % 100 + 1;
        std::mem::replace(&mut die, ndie)
    };
    while p1.roll(roll() + roll() + roll()) < 1000 && p2.roll(roll() + roll() + roll()) < 1000 {}
    std::cmp::min(p1.score, p2.score) * rolls
}

fn dice() -> HashMap<usize, usize> {
    let mut vs = HashMap::new();
    for r1 in 1..=3 {
        for r2 in 1..=3 {
            for r3 in 1..=3 {
                *vs.entry(r1 + r2 + r3).or_insert(0) += 1;
            }
        }
    }
    vs
}

fn play2(s1: usize, s2: usize) -> usize {
    let dice = dice();
    let mut p1wins = 0;
    let mut p2wins = 0;
    let mut universes = HashMap::new();
    universes.insert((Player::new(s1), Player::new(s2)), 1);
    while !universes.is_empty() {
        for ((p1, p2), cnt) in std::mem::take(&mut universes) {
            for (&d1, &dcnt1) in dice.iter() {
                let mut p1 = p1.clone();
                if p1.roll(d1) >= 21 {
                    p1wins += cnt * dcnt1;
                    continue;
                }
                for (&d2, &dcnt2) in dice.iter() {
                    let mut p2 = p2.clone();
                    if p2.roll(d2) >= 21 {
                        p2wins += cnt * dcnt1 * dcnt2;
                        continue;
                    }
                    *universes.entry((p1.clone(), p2)).or_insert(0) += cnt * dcnt1 * dcnt2;
                }
            }
        }
    }
    std::cmp::max(p1wins, p2wins)
}

fn main() {
    let file = File::open("aoc2021/inputs/day21.input").unwrap();
    let input = io::BufReader::new(file)
        .lines()
        .flatten()
        .map(|f| f.split_whitespace().last().unwrap().parse().unwrap())
        .collect::<Vec<_>>();
    println!("part 1: {}", play1(input[0], input[1]));
    println!("part 2: {}", play2(input[0], input[1]));
}
