use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

type Pair = (char, char);

fn step(polymer: HashMap<Pair, usize>, rules: &HashMap<Pair, char>) -> HashMap<Pair, usize> {
    let mut npolymer = HashMap::new();
    for ((a, c), v) in polymer {
        if let Some(&b) = rules.get(&(a, c)) {
            *npolymer.entry((a, b)).or_insert(0) += v;
            *npolymer.entry((b, c)).or_insert(0) += v;
        }
    }
    npolymer
}

fn run(template: String, rules: &HashMap<Pair, char>, steps: usize) -> usize {
    let mut polymer = HashMap::new();
    for pair in template.chars().zip(template.chars().skip(1)) {
        *polymer.entry(pair).or_insert(0) += 1;
    }
    let mut freqs = (0..steps)
        .fold(polymer, |f, _| step(f, rules))
        .into_iter()
        .fold(HashMap::new(), |mut freqs, ((a, _), count)| {
            *freqs.entry(a).or_insert(0) += count;
            freqs
        });
    *freqs.entry(template.chars().last().unwrap()).or_insert(0) += 1;
    freqs.values().max().unwrap() - freqs.values().min().unwrap()
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
    println!("part 1: {}", run(template.clone(), &rules, 10));
    println!("part 2: {}", run(template, &rules, 40));
}
