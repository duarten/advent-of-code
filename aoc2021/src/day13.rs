use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Dot {
    x: usize,
    y: usize,
}

impl Dot {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn fold(dots: &mut HashSet<Dot>, selector: impl Fn(&Dot) -> Option<(Dot, Dot)>) {
    for (remove, add) in dots.iter().filter_map(selector).collect::<Vec<_>>() {
        dots.remove(&remove);
        dots.insert(add);
    }
}

fn display(dots: HashSet<Dot>) {
    let max_x = dots.iter().map(|d| d.x).max().unwrap();
    let max_y = dots.iter().map(|d| d.y).max().unwrap();
    for y in 0..=max_y {
        for x in 0..=max_x {
            match dots.get(&Dot { x, y }) {
                Some(_) => print!("#"),
                None => print!(" "),
            }
        }
        println!();
    }
}

fn main() {
    let file = File::open("aoc2021/inputs/day13.input").unwrap();
    let mut dots = HashSet::new();
    let mut lines = io::BufReader::new(file).lines();
    let iter = lines.by_ref();
    let mut first = None;
    for line in iter.map(Result::unwrap).take_while(|l| !l.is_empty()) {
        let dot = line.split_once(',').unwrap();
        dots.insert(Dot::new(dot.0.parse().unwrap(), dot.1.parse().unwrap()));
    }
    for line in iter.map(Result::unwrap) {
        let (dir, value) = line.split(' ').last().unwrap().split_once('=').unwrap();
        let value = value.parse::<usize>().unwrap();
        if dir == "y" {
            fold(&mut dots, |&dot| {
                (dot.y > value).then(|| (dot, Dot::new(dot.x, value - (dot.y - value))))
            });
        } else if dir == "x" {
            fold(&mut dots, |&dot| {
                (dot.x > value).then(|| (dot, Dot::new(value - (dot.x - value), dot.y)))
            });
        }
        if first.is_none() {
            first = Some(dots.len());
        }
    }
    println!("part 1: {}", first.unwrap());
    println!("part 2:");
    display(dots);
}
