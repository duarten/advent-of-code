use std::{
    fs::File,
    io::{self, BufRead},
};

fn check(chars: Vec<char>) -> Result<usize, usize> {
    let mut expected = Vec::new();
    for c in chars {
        if matches!(c, '(' | '[' | '{' | '<') {
            expected.push(match c {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => unreachable!(),
            });
        } else {
            let closing = expected.remove(expected.len() - 1);
            if closing != c {
                return Err(match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                });
            }
        }
    }
    Ok(expected.into_iter().rev().fold(0, |acc, c| {
        acc * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            }
    }))
}

fn main() {
    let file = File::open("aoc2021/inputs/day10.input").unwrap();
    let (corrupted, mut incomplete) = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .map(check)
        .fold((0, vec![]), |(mut corrupted, mut incomplete), r| {
            match r {
                Ok(c) => incomplete.push(c),
                Err(c) => corrupted += c,
            }
            (corrupted, incomplete)
        });
    incomplete.sort_unstable();
    println!("part 1: {}", corrupted);
    println!("part 2: {}", incomplete[incomplete.len() / 2]);
}
