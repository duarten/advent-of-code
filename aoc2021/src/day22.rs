use std::{
    cmp::{max, min},
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    ops::RangeInclusive,
};

use regex::Regex;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Cube(
    RangeInclusive<i64>,
    RangeInclusive<i64>,
    RangeInclusive<i64>,
);

fn intersect_range(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> RangeInclusive<i64> {
    max(*r1.start(), *r2.start())..=min(*r1.end(), *r2.end())
}

impl Cube {
    fn vol(&self) -> i64 {
        (self.0.end() - self.0.start() + 1)
            * (self.1.end() - self.1.start() + 1)
            * (self.2.end() - self.2.start() + 1)
    }

    fn intersect(&self, other: &Cube) -> Option<Cube> {
        let c = Cube(
            intersect_range(&self.0, &other.0),
            intersect_range(&self.1, &other.1),
            intersect_range(&self.2, &other.2),
        );
        (c.0.start() <= c.0.end() && c.1.start() <= c.1.end() && c.2.start() <= c.2.end())
            .then_some(c)
    }
}

fn main() {
    let file = File::open("aoc2021/inputs/day22.input").unwrap();
    let mut cubes = HashMap::new();
    for l in io::BufReader::new(file).lines().flatten() {
        let r = Regex::new(r#"(-?\d+)..(-?\d+)"#).unwrap();
        let ranges = r
            .captures_iter(&l)
            .map(|g| {
                g.get(1).unwrap().as_str().parse().unwrap()
                    ..=g.get(2).unwrap().as_str().parse().unwrap()
            })
            .collect::<Vec<_>>();
        let nc = Cube(ranges[0].clone(), ranges[1].clone(), ranges[2].clone());
        let changes = cubes
            .iter()
            .filter_map(|(c, n)| nc.intersect(c).map(|c| (c, *n)))
            .collect::<Vec<_>>();
        for (c, cnt) in changes {
            *cubes.entry(c).or_insert(0) -= cnt;
        }
        if l.starts_with("on") {
            *cubes.entry(nc).or_insert(0) += 1;
        }
    }
    let vol = cubes
        .iter()
        .filter(|(c, _)| {
            [&c.0, &c.1, &c.2]
                .into_iter()
                .all(|c| *c.start() > -50 && *c.end() < 50)
        })
        .map(|(c, &n)| c.vol() * n)
        .sum::<i64>();
    println!("part 1: {}", vol);
    let vol = cubes.iter().map(|(c, &n)| c.vol() * n).sum::<i64>();
    println!("part 2: {}", vol);
}
