use std::{
    collections,
    fs,
    io::{self, BufRead},
    str::FromStr,
};

use anyhow::Result;
use regex::Regex;

#[derive(Clone, Debug, Default)]
struct Mask {
    clear: u64,
    set: u64,
}

#[derive(PartialEq)]
enum Bit {
    Zero,
    One,
}

impl Mask {
    fn apply(&self, val: u64) -> u64 {
        (val & !self.clear) | self.set
    }

    fn set(&mut self, b: Bit, pos: usize) {
        let v = (1 << 35) >> pos;
        if b == Bit::Zero {
            self.clear |= v
        } else {
            self.set |= v
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Mask { mask: Mask, floating: Vec<Mask> },
    Mem { pos: u64, value: u64 },
}

lazy_static::lazy_static! {
    static ref RE_MEM: Regex = Regex::new(r#"^mem\[(\d+)\] = (\d+)$"#).unwrap();
}

fn parse_mask(s: &str) -> Instruction {
    let mut mask = Mask::default();
    let mut floating = vec![Mask::default()];
    for (i, b) in s.bytes().enumerate() {
        if b == b'0' {
            mask.set(Bit::Zero, i)
        } else if b == b'1' {
            mask.set(Bit::One, i)
        } else {
            let mut to_set = floating.clone();
            floating.iter_mut().for_each(|m| m.set(Bit::Zero, i));
            to_set.iter_mut().for_each(|m| m.set(Bit::One, i));
            floating.extend(to_set.into_iter());
        }
    }
    Instruction::Mask { mask, floating }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            return Ok(parse_mask(&s[7..]));
        }
        let captures = RE_MEM.captures(s).unwrap();
        Ok(Instruction::Mem {
            pos: captures.get(1).unwrap().as_str().parse()?,
            value: captures.get(2).unwrap().as_str().parse()?,
        })
    }
}

#[derive(Default)]
struct State {
    mask: Mask,
    floating: Vec<Mask>,
    mem_v1: collections::HashMap<u64, u64>,
    mem_v2: collections::HashMap<u64, u64>,
}

impl State {
    fn apply(&mut self, instrs: Vec<Instruction>) {
        for inst in instrs {
            match inst {
                Instruction::Mask { mask, floating } => {
                    self.mask = mask;
                    self.floating = floating;
                }
                Instruction::Mem { pos, value } => {
                    self.mem_v1.insert(pos, self.mask.apply(value));
                    let new_pos = pos | self.mask.set;
                    for m in self.floating.iter() {
                        self.mem_v2.insert(m.apply(new_pos), value);
                    }
                }
            }
        }
    }
}

fn main() {
    let file = fs::File::open("aoc2020/inputs/day14.input").unwrap();
    let instructions: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    let mut s = State::default();
    s.apply(instructions);
    println!("part 1: {}", s.mem_v1.values().sum::<u64>());
    println!("part 2: {}", s.mem_v2.values().sum::<u64>());
}
