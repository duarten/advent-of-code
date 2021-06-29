use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

type DistFn = dyn Fn(&str, usize, usize) -> bool;

fn dist1(_k: &str, v: usize, target: usize) -> bool {
    target == v
}

fn dist2(k: &str, v: usize, target: usize) -> bool {
    match k {
        "cat" | "trees" => v > target,
        "pomeranians" | "goldfish" => v < target,
        _ => target == v,
    }
}

fn dist(target: &HashMap<String, usize>, sue: &HashMap<String, usize>, f: &DistFn) -> usize {
    target
        .iter()
        .map(|(k, t)| sue.get(k).map(|v| f(k.as_str(), *v, *t)).unwrap_or(false) as usize)
        .sum()
}

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r#"(\w+: \d+)"#).unwrap();
}

fn main() {
    let file = File::open("aoc2015/inputs/day16.input").unwrap();
    let sues = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            RE.captures_iter(&l)
                .map(|m| {
                    let mut splits = m.get(0).unwrap().as_str().split(": ");
                    (
                        splits.next().unwrap().to_owned(),
                        splits.next().unwrap().parse::<usize>().unwrap(),
                    )
                })
                .collect::<HashMap<String, usize>>()
        })
        .collect::<Vec<_>>();

    let target: HashMap<String, usize> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .iter()
    .cloned()
    .map(|(s, u)| (s.to_owned(), u))
    .collect();
    for (pt, d) in [dist1, dist2].iter().enumerate() {
        println!(
            "part {}: {}",
            pt,
            1 + sues
                .iter()
                .enumerate()
                .max_by_key(|(_, s)| dist(&target, s, d))
                .unwrap()
                .0
        );
    }
}
