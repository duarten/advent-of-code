use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

use regex::Regex;

enum Target {
    Bot(usize),
    Output(usize),
}

impl FromStr for Target {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target, value) = s.split_once(' ').unwrap();
        let value = value.parse::<usize>()?;
        Ok(match target {
            "bot" => Target::Bot(value),
            "output" => Target::Output(value),
            _ => unreachable!(),
        })
    }
}

struct Instruction {
    bot: usize,
    low: Target,
    high: Target,
}

lazy_static::lazy_static! {
    static ref RE_INIT: Regex = Regex::new(r#"^value (\d+) goes to bot (\d+)$"#).unwrap();
    static ref RE_INST: Regex = Regex::new(r#"^bot (\d+) gives low to (.+) and high to (.+)$"#).unwrap();
}

fn main() {
    let file = File::open("aoc2016/inputs/day10.input").unwrap();
    let mut bots = HashMap::new();
    let mut insts = Vec::new();
    for l in io::BufReader::new(file).lines().map(Result::unwrap) {
        if let Some(captures) = RE_INIT.captures(&l) {
            let value = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            bots.entry(captures.get(2).unwrap().as_str().parse::<usize>().unwrap())
                .or_insert_with(Vec::new)
                .push(value);
        } else if let Some(captures) = RE_INST.captures(&l) {
            insts.push(Instruction {
                bot: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                low: captures.get(2).unwrap().as_str().parse::<Target>().unwrap(),
                high: captures.get(3).unwrap().as_str().parse::<Target>().unwrap(),
            });
        }
    }
    let mut outputs = HashMap::new();
    while bots.values().map(|v| v.len()).sum::<usize>() > 0 {
        for Instruction { bot, low, high } in &insts {
            let entry = bots.entry(*bot).or_insert_with(Vec::new);
            if entry.len() < 2 {
                continue;
            }
            entry.sort_unstable();
            let highv = entry.pop().unwrap();
            let lowv = entry.pop().unwrap();
            if lowv == 17 && highv == 61 {
                println!("part 1: {}", bot);
            }
            for (v, t) in [lowv, highv].into_iter().zip([&low, &high]) {
                (match t {
                    Target::Bot(bot) => bots.entry(*bot),
                    Target::Output(output) => outputs.entry(*output),
                })
                .or_insert_with(Vec::new)
                .push(v);
            }
        }
    }
    let p2: usize = [0, 1, 2]
        .into_iter()
        .flat_map(|i| outputs.get(&i).unwrap())
        .product();
    println!("part 2: {}", p2);
}
