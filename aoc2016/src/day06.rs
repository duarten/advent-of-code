use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("aoc2016/inputs/day06.input").unwrap();
    let mut freqs = Vec::new();
    freqs.resize(8, HashMap::new());
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        for (idx, c) in line.chars().enumerate() {
            *freqs[idx].entry(c).or_insert(0) += 1;
        }
    }
    let mut code1 = Vec::new();
    let mut code2 = Vec::new();
    for m in freqs {
        code1.push(*m.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0);
        code2.push(*m.iter().min_by(|a, b| a.1.cmp(b.1)).unwrap().0);
    }
    println!("part 1: {}", code1.into_iter().collect::<String>());
    println!("part 2: {}", code2.into_iter().collect::<String>());
}
