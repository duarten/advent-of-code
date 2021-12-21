use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead},
};

use regex::Regex;

struct Screen([[u8; 50]; 6]);

impl Screen {
    fn new() -> Self {
        Self([[0; 50]; 6])
    }

    fn rect(&mut self, cols: usize, rows: usize) {
        for col in 0..cols {
            for row in 0..rows {
                self.0[row][col] = 1;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, k: usize) {
        let mut vec = VecDeque::from(self.0[row]);
        vec.rotate_right(k);
        let (s1, s2) = vec.as_slices();
        self.0[row][..s1.len()].clone_from_slice(s1);
        self.0[row][s1.len()..].clone_from_slice(s2);
    }

    fn rotate_col(&mut self, col: usize, k: usize) {
        let mut vec = VecDeque::from(self.0.map(|r| r[col]));
        vec.rotate_right(k);
        for (i, item) in vec.into_iter().enumerate() {
            self.0[i][col] = item;
        }
    }

    fn lit(&self) -> usize {
        self.0.iter().flatten().map(|&p| p as usize).sum()
    }

    fn display(&self) {
        for row in 0..self.0.len() {
            for col in 0..self.0[0].len() {
                if self.0[row][col] == 1 {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

lazy_static::lazy_static! {
    static ref RE_RECT: Regex = Regex::new(r#"^rect (\d+)x(\d+)$"#).unwrap();
    static ref RE_ROT_ROW: Regex = Regex::new(r#"^rotate row y=(\d+) by (\d+)$"#).unwrap();
    static ref RE_ROT_COL: Regex = Regex::new(r#"^rotate column x=(\d+) by (\d+)$"#).unwrap();
}

fn main() {
    let file = File::open("aoc2016/inputs/day08.input").unwrap();
    let mut s = Screen::new();
    let operands = |r: &Regex, s: &str| -> Option<(usize, usize)> {
        r.captures(s).map(|g| {
            (
                g.get(1).unwrap().as_str().parse().unwrap(),
                g.get(2).unwrap().as_str().parse().unwrap(),
            )
        })
    };
    for l in io::BufReader::new(file).lines().map(Result::unwrap) {
        if let Some((cols, rows)) = operands(&RE_RECT, &l) {
            s.rect(cols, rows);
        } else if let Some((row, k)) = operands(&RE_ROT_ROW, &l) {
            s.rotate_row(row, k);
        } else if let Some((col, k)) = operands(&RE_ROT_COL, &l) {
            s.rotate_col(col, k);
        }
    }
    println!("part 1: {}", s.lit());
    println!("part 2:");
    s.display();
}
