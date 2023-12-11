use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

use utils::manhattan_distance;

fn main() {
    let lines = io::BufReader::new(File::open("aoc2023/inputs/day11.input").unwrap())
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    println!("part 1: {}", solve(lines.iter(), 2));
    println!("part 2: {}", solve(lines.iter(), 1_000_000));
}

fn solve<'a>(lines: impl Iterator<Item = &'a String>, expansion: usize) -> usize {
    let mut galaxy_col_buckets = HashMap::<_, Vec<_>>::new();
    let mut shift = 0;
    let mut width = 0;
    for (y, line) in lines.enumerate() {
        let mut has_galaxy = false;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxy_col_buckets.entry(x).or_default().push(y + shift);
                has_galaxy = true;
            }
            width = x;
        }
        if !has_galaxy {
            shift += expansion - 1;
        }
    }
    let mut galaxies = Vec::new();
    let mut shift = 0;
    for x in 0..=width {
        if let Some(rows) = galaxy_col_buckets.get(&x) {
            galaxies.extend(rows.iter().map(|y| (x + shift, *y)));
        } else {
            shift += expansion - 1;
        }
    }
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let g1 = galaxies[i];
            let g2 = galaxies[j];
            let dist = manhattan_distance(g1, g2);
            sum += dist;
        }
    }
    sum
}
