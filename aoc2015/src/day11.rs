use itertools::iterate;

use std::collections::HashSet;
use std::fs;

fn next(mut prev: Vec<u8>) -> Vec<u8> {
    for i in 0..8 {
        let idx = prev.len() - 1 - i;
        let c = (((prev[idx] - b'a') + 1) % (b'z' - b'a' + 1)) + b'a';
        prev[idx] = c;
        if c != b'a' {
            break;
        }
    }
    prev
}

fn is_valid(pass: &[u8]) -> bool {
    pass.iter().all(|c| !matches!(c, b'i' | b'o' | b'l'))
        && pass
            .windows(3)
            .any(|w| w[1] == w[0] + 1 && w[2] == w[1] + 1)
        && pass
            .windows(2)
            .filter(|w| w[0] == w[1])
            .map(|w| {
                let mut tmp = String::new();
                tmp.push(w[0] as char);
                tmp.push(w[1] as char);
                tmp
            })
            .collect::<HashSet<_>>()
            .len()
            >= 2
}

fn next_password(input: Vec<u8>) -> Vec<u8> {
    iterate(input, |v| next(v.to_vec()))
        .find(|v| is_valid(&v))
        .unwrap()
}

fn to_string(v: &[u8]) -> String {
    v.iter()
        .map(|c| (*c as char).to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn main() {
    let input: Vec<_> = fs::read_to_string("aoc2015/inputs/day11.input")
        .unwrap()
        .chars()
        .map(|c| c as u8)
        .collect();
    let p1 = next_password(input);
    println!("part 1: {}", to_string(&p1));
    println!("part 1: {}", to_string(&next_password(next(p1))));
}
