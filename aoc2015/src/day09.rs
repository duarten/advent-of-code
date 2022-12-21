use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

struct Connection {
    to: String,
    distance: usize,
}

struct Path<'a> {
    next: &'a str,
    visited: HashSet<&'a str>,
    cost: usize,
}

impl<'a> Path<'a> {
    fn new(next: &'a str, visited: HashSet<&'a str>, cost: usize) -> Self {
        Self {
            next,
            visited,
            cost,
        }
    }
}

fn min_distance(connections: &HashMap<String, Vec<Connection>>) -> (usize, usize) {
    let mut to_visit: Vec<_> = connections
        .keys()
        .map(|next| Path::new(next, HashSet::<&str>::new(), 0))
        .collect();
    let mut costs = Vec::<usize>::new();
    while let Some(Path {
        next,
        cost,
        mut visited,
    }) = to_visit.pop()
    {
        visited.insert(next);
        if visited.len() == connections.len() {
            costs.push(cost);
            continue;
        }
        for c in connections[next].iter() {
            if !visited.contains(c.to.as_str()) {
                to_visit.push(Path::new(&c.to, visited.clone(), cost + c.distance));
            }
        }
    }
    (
        *costs.iter().min().unwrap(),
        costs.into_iter().max().unwrap(),
    )
}

lazy_static::lazy_static! {
    static ref RE_DIST: Regex = Regex::new(r#"^(\w+) to (\w+) = (\d+)$"#).unwrap();
}

fn main() {
    let file = File::open("aoc2015/inputs/day09.input").unwrap();
    let mut connections = HashMap::<String, Vec<Connection>>::new();
    BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .for_each(|l| {
            if let Some(g) = RE_DIST.captures(&l) {
                let from = g.get(1).unwrap().as_str().to_owned();
                let to = g.get(2).unwrap().as_str().to_owned();
                let distance: usize = g.get(3).unwrap().as_str().parse().unwrap();
                connections
                    .entry(from.clone())
                    .or_insert_with(Vec::new)
                    .push(Connection {
                        to: to.clone(),
                        distance,
                    });
                connections
                    .entry(to)
                    .or_insert_with(Vec::new)
                    .push(Connection { to: from, distance });
            } else {
                panic!();
            }
        });
    let (min, max) = min_distance(&connections);
    println!("part 1: {}", min);
    println!("part 2: {}", max);
}
