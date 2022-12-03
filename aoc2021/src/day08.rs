use std::{
    fs::File,
    io::{self, BufRead},
};

fn contains_all(search: &str, container: &str) -> bool {
    search.chars().all(|c| container.contains(c))
}

fn decode(signals: Vec<String>, value: Vec<String>) -> usize {
    let mut decoded = [""; 10];
    for s in signals.iter() {
        match s.len() {
            2 => decoded[1] = s,
            4 => decoded[4] = s,
            3 => decoded[7] = s,
            7 => decoded[8] = s,
            _ => {}
        }
    }
    for s in signals.iter().filter(|s| s.len() == 6) {
        if !contains_all(decoded[1], s) {
            decoded[6] = s;
        } else if contains_all(decoded[4], s) {
            decoded[9] = s;
        } else {
            decoded[0] = s;
        }
    }
    for s in signals.iter().filter(|s| s.len() == 5) {
        if contains_all(decoded[1], s) {
            decoded[3] = s;
        } else if contains_all(s, decoded[6]) {
            decoded[5] = s;
        } else {
            decoded[2] = s;
        }
    }
    let mut result = 0;
    let mut mag = 1000;
    for v in value {
        result += mag * decoded.iter().position(|&d| v == d).unwrap();
        mag /= 10
    }
    result
}

fn normalize(s: &str) -> Vec<String> {
    s.split(' ')
        .map(|s| {
            let mut chars: Vec<char> = s.chars().collect::<Vec<_>>();
            chars.sort_unstable();
            String::from_iter(chars)
        })
        .collect::<Vec<_>>()
}

fn main() {
    let file = File::open("aoc2021/inputs/day08.input").unwrap();
    let input = io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .split_once(" | ")
                .map(|(s, d)| (normalize(s), normalize(d)))
                .unwrap()
        })
        .collect::<Vec<_>>();

    let p1 = input
        .iter()
        .flat_map(|(_, digits)| digits)
        .filter(|digit| matches!(digit.len(), 2 | 4 | 3 | 7))
        .count();
    println!("part 1: {}", p1);

    let p2 = input.into_iter().map(|(k, v)| decode(k, v)).sum::<usize>();
    println!("part 2: {}", p2);
}
