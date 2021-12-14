use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone)]
struct Polymer {
    inner: HashMap<(char, char), usize>,
    outer: (char, char),
}

fn step(polymer: Polymer, rules: &HashMap<(char, char), char>) -> Polymer {
    let mut inner = HashMap::new();
    for (pair, count) in polymer.inner {
        if let Some(&insert) = rules.get(&pair) {
            *inner.entry((pair.0, insert)).or_insert(0) += count;
            *inner.entry((insert, pair.1)).or_insert(0) += count;
        }
    }
    Polymer {
        inner,
        outer: polymer.outer,
    }
}

fn cycle(polymer: Polymer, rules: &HashMap<(char, char), char>, iter: usize) -> Polymer {
    (0..iter).fold(polymer, |p, _| step(p, rules))
}

fn value(polymer: &Polymer) -> usize {
    let mut frequencies = HashMap::<char, usize>::new();
    for ((left, right), count) in polymer.inner.iter() {
        *frequencies.entry(*left).or_insert(0) += count;
        *frequencies.entry(*right).or_insert(0) += count;
    }
    *frequencies.entry(polymer.outer.0).or_insert(0) += 1;
    *frequencies.entry(polymer.outer.1).or_insert(0) += 1;
    let max = frequencies.values().max().unwrap();
    let min = frequencies.values().min().unwrap();
    (max - min) / 2
}

fn main() {
    let file = File::open("aoc2021/inputs/day14.input").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let iter = lines.by_ref();
    let template = iter.next().unwrap().unwrap();
    let rules = iter
        .skip(1)
        .map(|l| {
            let line = l.unwrap();
            let (from, to) = line.split_once(" -> ").unwrap();
            (
                (from.chars().next().unwrap(), from.chars().nth(1).unwrap()),
                to.chars().next().unwrap(),
            )
        })
        .collect::<HashMap<_, _>>();
    let mut polymer = Polymer {
        inner: HashMap::new(),
        outer: (
            template.chars().next().unwrap(),
            template.chars().nth(template.len() - 1).unwrap(),
        ),
    };
    for i in 0..(template.len() - 1) {
        *polymer
            .inner
            .entry((
                template.chars().nth(i).unwrap(),
                template.chars().nth(i + 1).unwrap(),
            ))
            .or_insert(0) += 1;
    }
    let polymer = cycle(polymer, &rules, 10);
    println!("part 1: {}", value(&polymer));
    let polymer = cycle(polymer, &rules, 30);
    println!("part 2: {}", value(&polymer));
}
