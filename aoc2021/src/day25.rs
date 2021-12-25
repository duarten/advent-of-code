use std::{
    fs::File,
    io::{self, BufRead},
    ops::{Index, IndexMut},
};

use itertools::Itertools;

type Offset = (usize, usize);

#[derive(PartialEq, Eq, Clone)]
enum Direction {
    East,
    South,
}

#[derive(PartialEq, Eq, Clone)]
struct Cucumber(Direction);

impl Cucumber {
    fn move_from(&self, c: Offset) -> Offset {
        match self.0 {
            Direction::East => (c.0, c.1 + 1),
            Direction::South => (c.0 + 1, c.1),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Cucumbers(Vec<Vec<Option<Cucumber>>>);

impl Index<Offset> for Cucumbers {
    type Output = Option<Cucumber>;

    fn index(&self, (r, c): Offset) -> &Self::Output {
        &self.0[r % self.0.len()][c % self.0[0].len()]
    }
}

impl IndexMut<Offset> for Cucumbers {
    fn index_mut(&mut self, (r, c): Offset) -> &mut Self::Output {
        let rows = self.0.len();
        let cols = self.0[0].len();
        &mut self.0[r % rows][c % cols]
    }
}

fn herd_changes(cs: &mut Cucumbers, target_dir: Direction) -> bool {
    let mut herd = Vec::new();
    for (row, col) in (0..cs.0.len()).cartesian_product(0..cs.0[0].len()) {
        if let Some(c @ Cucumber(dir)) = &cs[(row, col)] {
            if cs[c.move_from((row, col))].is_none() && *dir == target_dir {
                herd.push((row, col));
            }
        }
    }
    for of in herd.iter() {
        if let Some(c) = std::mem::take(&mut cs[*of]) {
            cs[c.move_from(*of)] = Some(c.clone());
        }
    }
    !herd.is_empty()
}

fn cycle(mut cs: Cucumbers) -> usize {
    let mut step = 1;
    loop {
        let moved_east = herd_changes(&mut cs, Direction::East);
        let moved_south = herd_changes(&mut cs, Direction::South);
        if !moved_east && !moved_south {
            break;
        }
        step += 1;
    }
    step
}

fn main() {
    let file = File::open("aoc2021/inputs/day25.input").unwrap();
    let cucumbers = io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| match c {
                    '.' => None,
                    '>' => Some(Cucumber(Direction::East)),
                    'v' => Some(Cucumber(Direction::South)),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("part 1: {}", cycle(Cucumbers(cucumbers)));
}
