use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_nice1(s: &str) -> bool {
    s.as_bytes().windows(2).any(|w| w[0] == w[1])
        && ["ab", "cd", "pq", "xy"].iter().all(|w| !s.contains(w))
        && s.chars()
            .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
            .count()
            >= 3
}

fn is_nice2(s: &str) -> bool {
    s.as_bytes().windows(3).any(|w| w[0] == w[2])
        && s.as_bytes().windows(2).any(|w| {
            let mut tmp = String::new();
            tmp.push(w[0] as char);
            tmp.push(w[1] as char);
            s.matches(&tmp).count() >= 2
        })
}

fn main() {
    let file = File::open("aoc2015/inputs/day05.input").unwrap();
    let lines: Vec<_> = BufReader::new(file).lines().map(Result::unwrap).collect();
    println!("part 1: {}", lines.iter().filter(|l| is_nice1(l)).count());
    println!("part 2: {}", lines.iter().filter(|l| is_nice2(l)).count());
}
