use std::{
    fs::File,
    io::{self, BufRead},
};

use cached::proc_macro::cached;
use itertools::iproduct;

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

fn play1(mut p1: Player, mut p2: Player) -> usize {
    let mut die = (1..=100).cycle().enumerate();
    let mut roll = || die.next().unwrap().1;
    while p1.roll(roll() + roll() + roll()) < 1000 && p2.roll(roll() + roll() + roll()) < 1000 {}
    std::cmp::min(p1.score, p2.score) * die.next().unwrap().0
}

#[cached]
fn play2(p1: Player, p2: Player) -> (usize, usize) {
    if p1.score >= 21 {
        return (1, 0);
    }
    if p2.score >= 21 {
        return (0, 1);
    }
    let mut p1wins = 0;
    let mut p2wins = 0;
    for d in iproduct!(1..=3, 1..=3, 1..=3).map(|(d1, d2, d3)| d1 + d2 + d3) {
        let mut p1 = p1.clone();
        p1.roll(d);
        let r = play2(p2.clone(), p1);
        p1wins += r.1;
        p2wins += r.0;
    }
    (p1wins, p2wins)
}

fn main() {
    let file = File::open("aoc2021/inputs/day21.input").unwrap();
    let input = io::BufReader::new(file)
        .lines()
        .flatten()
        .map(|f| Player::new(f.split_whitespace().last().unwrap().parse().unwrap()))
        .collect::<Vec<_>>();
    println!("part 1: {}", play1(input[0].clone(), input[1].clone()));
    let wins = play2(input[0].clone(), input[1].clone());
    println!("part 2: {}", std::cmp::max(wins.0, wins.1));
}
