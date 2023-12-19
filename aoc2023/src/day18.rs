use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("aoc2023/inputs/day18.input").unwrap();
    let mut instructions1 = Vec::new();
    let mut instructions2 = Vec::new();
    for l in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        let mut split = l.split_ascii_whitespace();
        let dir = split.next().unwrap();
        let steps = split.next().unwrap().parse::<i64>().unwrap();
        instructions1.push((dir.to_owned(), steps));
        let hex = split.next().unwrap();
        let steps = i64::from_str_radix(&hex[2..=6], 16).unwrap();
        let dir = match &hex[7..8].parse::<usize>().unwrap() {
            0 => "R",
            1 => "D",
            2 => "L",
            3 => "U",
            _ => unreachable!(),
        };
        instructions2.push((dir.to_owned(), steps));
    }
    println!("part 1: {:?}", solve(instructions1.into_iter()));
    println!("part 2: {:?}", solve(instructions2.into_iter()));
}

fn solve(instructions: impl Iterator<Item = (String, i64)>) -> i64 {
    let mut cur_dig = (0, 0);
    let mut exterior = 0;
    let mut area = 0;
    for (dir, steps) in instructions {
        let next = match dir.as_str() {
            "R" => (cur_dig.0 + steps, cur_dig.1),
            "L" => (cur_dig.0 - steps, cur_dig.1),
            "D" => (cur_dig.0, cur_dig.1 + steps),
            "U" => (cur_dig.0, cur_dig.1 - steps),
            _ => unreachable!(),
        };
        area += cur_dig.0 * next.1 - cur_dig.1 * next.0;
        exterior += steps;
        cur_dig = next;
    }
    area = area.abs() / 2;
    let interior = area + 1 - exterior / 2;
    interior + exterior
}
