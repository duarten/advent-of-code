use std::{
    fs::File,
    io::{self, BufRead},
};

use utils::bitvec::BitVec;

type Offset = (i64, i64);

struct Image(Vec<Vec<u8>>, u8);

impl Image {
    fn at(&self, (row, col): Offset) -> u8 {
        if row < 0 || row >= self.0.len() as i64 || col < 0 || col >= self.0[0].len() as i64 {
            self.1
        } else {
            self.0[row as usize][col as usize]
        }
    }

    fn bright(&self) -> usize {
        self.0.iter().flatten().map(|&p| p as usize).sum()
    }
}

fn section(img: &Image, (row, col): Offset) -> [u8; 9] {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 0),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .map(|(dr, dc)| img.at((row + dr, col + dc)))
}

fn enhance(img: Image, algo: &[u8]) -> Image {
    let mut output = Vec::new();
    let margin = 3;
    for r in (-margin)..(img.0.len() as i64 + margin) {
        let row = ((-margin)..(img.0[0].len() as i64 + margin))
            .map(|c| algo[section(&img, (r, c)).into_iter().to_number()])
            .collect();
        output.push(row);
    }
    Image(output, (img.1 + 1) & 1)
}

fn cycle(img: Image, algo: &[u8], n: usize) -> Image {
    (0..n).fold(img, |acc, _| enhance(acc, algo))
}

fn value(c: char) -> u8 {
    (c == '#') as u8
}

fn main() {
    let file = File::open("aoc2021/inputs/day20.input").unwrap();
    let mut input = io::BufReader::new(file).lines().flatten();
    let algo = input.next().unwrap().chars().map(value).collect::<Vec<_>>();
    let img = input
        .skip(1)
        .map(|x| x.chars().map(value).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!(
        "part 1: {}",
        cycle(Image(img.clone(), 0), &algo, 2).bright()
    );
    println!("part 2: {}", cycle(Image(img, 0), &algo, 50).bright());
}
