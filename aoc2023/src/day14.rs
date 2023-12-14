use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

use utils::rotate;

fn main() {
    let file = File::open("aoc2023/inputs/day14.input").unwrap();
    let mut platform = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let load1 = {
        let mut platform = platform.clone();
        tilt_north(&mut platform);
        total_load(&platform)
    };
    println!("part 1: {}", load1);
    let mut history = HashMap::new();
    for idx in 1..1000000000 {
        cycle(&mut platform);
        if let Some(seen_at) = history.insert(total_load(&platform), idx) {
            if (1000000000 - idx) % (idx - seen_at) == 0 {
                break;
            }
        }
    }
    println!("part 2: {}", total_load(&platform));
}

fn cycle(platform: &mut Vec<Vec<char>>) {
    for _ in 0..4 {
        tilt_north(platform);
        *platform = rotate(platform);
    }
}

fn total_load(platform: &[Vec<char>]) -> usize {
    platform
        .iter()
        .enumerate()
        .map(|(idx, row)| (platform.len() - idx) * row.iter().filter(|&&c| c == 'O').count())
        .sum()
}

fn tilt_north(platform: &mut Vec<Vec<char>>) {
    for col in 0..platform[0].len() {
        let mut next = 0;
        for row in 0..platform.len() {
            match platform[row][col] {
                'O' => {
                    if next != row {
                        platform[next][col] = 'O';
                        platform[row][col] = '.';
                    }
                    next += 1;
                }
                '#' => next = row + 1,
                _ => {}
            }
        }
    }
}
