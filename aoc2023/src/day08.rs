use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

use utils::lcm;

fn main() {
    let file = File::open("aoc2023/inputs/day08.input").unwrap();
    let mut lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();
    let mut network = HashMap::new();
    for node in lines.skip(1) {
        let (this, left_right) = node.split_once(" = ").unwrap();
        let (left, right) = left_right[1..left_right.len() - 1]
            .split_once(", ")
            .unwrap();
        network.insert(this.to_owned(), (left.to_owned(), right.to_owned()));
    }
    println!("part 1: {}", solve(&network, &instructions, "AAA"));
    let lcm = network
        .keys()
        .filter(|k| k.chars().nth(2) == Some('A'))
        .map(|n| solve(&network, &instructions, n.as_ref()))
        .reduce(lcm);
    println!("part 2: {}", lcm.unwrap());
}

fn solve<'a>(
    network: &'a HashMap<String, (String, String)>,
    instructions: &[char],
    mut current: &'a str,
) -> usize {
    let mut step = 0;
    while current.chars().nth(2) != Some('Z') {
        let (left, right) = network.get(current).unwrap();
        current = if instructions[step % instructions.len()] == 'L' {
            left.as_str()
        } else {
            right.as_str()
        };
        step += 1;
    }
    step
}
