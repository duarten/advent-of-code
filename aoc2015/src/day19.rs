use std::{
    collections::HashSet,
    fs,
    io::{self, BufRead},
};

use anyhow::Result;
use itertools::{iterate, Itertools};

fn all(target: &str, transformations: &[(String, String)]) -> usize {
    let mut set = HashSet::<String>::new();
    for pos in 0..target.len() {
        for (k, v) in transformations.iter() {
            if target[pos..].starts_with(k) {
                let chars = target[0..pos]
                    .chars()
                    .chain(v.chars())
                    .chain(target[pos + k.len()..].chars());
                set.insert(chars.collect::<String>());
            }
        }
    }
    set.len()
}

fn reverse_iterate(target: &str, transformations: &[(String, String)]) -> Option<String> {
    for (k, v) in transformations.iter() {
        if let Some(pos) = target.find(v) {
            let chars = target[0..pos]
                .chars()
                .chain(k.chars())
                .chain(target[pos + v.len()..].chars());
            return Some(chars.collect::<String>());
        }
    }
    None
}

fn reverse(target: String, transformations: &[(String, String)]) -> usize {
    iterate(Some(target), |v_opt| {
        v_opt
            .as_ref()
            .and_then(|v| reverse_iterate(v, transformations))
    })
    .flatten()
    .take_while(|v| v != "e")
    .count()
}

fn main() -> Result<()> {
    let file = fs::File::open("aoc2015/inputs/day19.input").unwrap();
    let mut lines = io::BufReader::new(file).lines().flatten();
    let transformations = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .filter_map(|l| {
            l.split(" => ")
                .map(|x| x.to_owned())
                .collect_tuple::<(String, String)>()
        })
        .collect::<Vec<_>>();
    let target = lines.last().unwrap();
    println!("part 1: {}", all(&target, &transformations));
    println!("part 2: {}", reverse(target, &transformations));
    Ok(())
}
