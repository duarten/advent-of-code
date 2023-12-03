use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead},
};

use itertools::Itertools;

fn apply_pt1(stacks: &mut [VecDeque<u8>], amount: usize, from: usize, to: usize) {
    for _ in 0..amount {
        let c = stacks[from].pop_front().unwrap();
        stacks[to].push_front(c);
    }
}

fn apply_pt2(stacks: &mut [VecDeque<u8>], amount: usize, from: usize, to: usize) {
    let mut tmp = VecDeque::with_capacity(amount);
    for _ in 0..amount {
        tmp.push_front(stacks[from].pop_front().unwrap());
    }
    for x in tmp {
        stacks[to].push_front(x);
    }
}

fn main() {
    let file = File::open("aoc2022/inputs/day05.input").unwrap();
    let mut stacks1 = Vec::<VecDeque<u8>>::new();
    stacks1.resize_with(9, Default::default);
    let mut lines = io::BufReader::new(file).lines().map(Result::unwrap);
    for line in lines.by_ref().take_while(|l| !l.starts_with(' ')) {
        for (idx, item) in line.into_bytes().chunks(4).enumerate() {
            let value = item[1];
            if value.is_ascii_uppercase() {
                stacks1[idx].push_back(value);
            }
        }
    }
    lines.by_ref().next();
    let mut stacks2 = stacks1.clone();
    for line in lines.by_ref() {
        let split = line.split(' ').collect::<Vec<_>>();
        let amount = split[1].parse::<usize>().unwrap();
        let from = split[3].parse::<usize>().unwrap() - 1;
        let to = split[5].parse::<usize>().unwrap() - 1;
        apply_pt1(&mut stacks1, amount, from, to);
        apply_pt2(&mut stacks2, amount, from, to);
    }
    println!(
        "part 1: {}",
        stacks1.iter().map(|v| *v.front().unwrap() as char).join("")
    );
    println!(
        "part 2: {}",
        stacks2.iter().map(|v| *v.front().unwrap() as char).join("")
    );
}
