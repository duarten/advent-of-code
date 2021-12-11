use std::{
    fs::File,
    io::{self, BufRead},
    iter::repeat,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
struct Octos(Vec<Vec<usize>>);

type Offset = (usize, usize);

impl Index<Offset> for Octos {
    type Output = usize;

    fn index(&self, index: Offset) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl IndexMut<Offset> for Octos {
    fn index_mut(&mut self, index: Offset) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

impl Octos {
    fn offsets(&self) -> Vec<Offset> {
        (0..self.0.len())
            .flat_map(|row| repeat(row).zip(0..self.0[0].len()))
            .collect()
    }

    fn at(&self, index: Offset) -> Option<usize> {
        (index.0 < self.0.len() && index.1 < self.0[0].len()).then(|| self[index])
    }

    fn len(&self) -> usize {
        self.0.len() * self.0[0].len()
    }
}

fn neighbors(ocots: &Octos, offset: Offset) -> Vec<Offset> {
    [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]
    .into_iter()
    .filter_map(|step: (i32, i32)| {
        let new_offset = (
            offset.0.wrapping_add(step.0 as usize),
            offset.1.wrapping_add(step.1 as usize),
        );
        ocots.at(new_offset).and(Some(new_offset))
    })
    .collect()
}

fn step(octos: &mut Octos) -> usize {
    let mut flashing = Vec::<Offset>::default();
    for pos in octos.offsets() {
        octos[pos] += 1;
        if octos[pos] > 9 {
            flashing.push(pos);
        }
    }
    while !flashing.is_empty() {
        for npos in neighbors(octos, flashing.swap_remove(0)) {
            octos[npos] += 1;
            if octos[npos] == 10 {
                flashing.push(npos);
            }
        }
    }
    let mut flashes = 0;
    for pos in octos.offsets() {
        if octos[pos] > 9 {
            flashes += 1;
            octos[pos] = 0;
        }
    }
    flashes
}

fn count_flashes(octos: &mut Octos, iterations: usize) -> usize {
    (0..iterations).map(|_| step(octos)).sum()
}

fn all_flash(octos: &mut Octos) -> usize {
    (0..).take_while(|_| step(octos) != octos.len()).count() + 101
}

fn main() {
    let file = File::open("aoc2021/inputs/day11.input").unwrap();
    let input = io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut octos = Octos(input);
    println!("part 1: {}", count_flashes(&mut octos, 100));
    println!("part 2: {}", all_flash(&mut octos));
}
