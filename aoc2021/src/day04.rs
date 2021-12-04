use std::{
    fs::File,
    io::{self, BufRead},
};

struct Board(Vec<Vec<Option<usize>>>);

impl Board {
    fn bingo(&mut self, called: usize) -> bool {
        for r in 0..5 {
            let mut marked_r = 0;
            let mut marked_c = 0;
            for c in 0..5 {
                if self.0[r][c] == Some(called) {
                    self.0[r][c] = None;
                }
                marked_r += self.0[r][c].is_none() as usize;
                marked_c += self.0[c][r].is_none() as usize;
                if marked_c == 5 || marked_r == 5 {
                    return true;
                }
            }
        }
        false
    }

    fn score(self) -> usize {
        self.0.into_iter().flatten().flatten().sum::<usize>()
    }
}

fn main() {
    let file = File::open("aoc2021/inputs/day04.input").unwrap();
    let mut lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let called = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();
    let mut boards = lines
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|c| Board(c.to_owned()))
        .collect::<Vec<_>>();

    let mut winners = Vec::with_capacity(boards.len());
    for c in called {
        let mut b = 0;
        while b < boards.len() {
            if boards[b].bingo(c) {
                winners.push(boards.swap_remove(b).score() * c);
            } else {
                b += 1;
            }
        }
    }
    println!("part 1: {}", winners[0]);
    println!("part 2: {}", winners[winners.len() - 1]);
}
