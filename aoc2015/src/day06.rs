use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Clone, Default, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn square(self, other: Coord) -> Vec<Coord> {
        let mut r = vec![];
        for y in self.y..=other.y {
            for x in self.x..=other.x {
                r.push(Coord::new(x, y));
            }
        }
        r
    }
}

type Grid = HashSet<Coord>;
type Brightness = HashMap<Coord, usize>;

fn increase(b: &mut Brightness, coords: &[Coord], val: usize) {
    coords.iter().for_each(|c| {
        b.entry(c.clone()).and_modify(|b| *b += val).or_insert(val);
    });
}

fn turn_on(g: &mut Grid, b: &mut Brightness, coords: Vec<Coord>) {
    increase(b, &coords, 1);
    g.extend(coords);
}

fn toggle(g: &mut Grid, b: &mut Brightness, coords: Vec<Coord>) {
    increase(b, &coords, 2);
    coords.into_iter().for_each(|c| {
        if !g.remove(&c) {
            g.insert(c);
        }
    })
}

fn turn_off(g: &mut Grid, b: &mut Brightness, coords: Vec<Coord>) {
    coords.iter().for_each(|c| {
        b.entry(c.clone())
            .and_modify(|b| *b = if *b > 0 { *b - 1 } else { 0 });
    });
    coords.into_iter().for_each(|c| {
        g.remove(&c);
    })
}

lazy_static::lazy_static! {
    static ref RE_RULE: Regex = Regex::new(r#"^([\w\s]+) (\d+),(\d+) through (\d+),(\d+)$"#).unwrap();
}

fn main() {
    let file = File::open("aoc2015/inputs/day06.input").unwrap();
    let mut g = Grid::new();
    let mut b = Brightness::new();
    BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .for_each(|l| {
            let c = RE_RULE.captures(&l).unwrap();
            let f = match c.get(1).unwrap().as_str() {
                "turn on" => turn_on,
                "turn off" => turn_off,
                "toggle" => toggle,
                _ => panic!(),
            };
            let val = |g: usize| c.get(g).unwrap().as_str().parse().unwrap();
            let coords = Coord::new(val(2), val(3)).square(Coord::new(val(4), val(5)));
            f(&mut g, &mut b, coords);
        });
    println!("part 1: {}", g.len());
    println!("part 2: {}", b.values().sum::<usize>());
}
