use anyhow::{anyhow, Result};
use regex::Regex;
use std::fs;
use std::io::{self, BufRead};
use std::str;

#[derive(Debug)]
enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

lazy_static::lazy_static! {
    static ref RE_RULE: Regex = Regex::new(r#"^(\w+) (\d+)$"#).unwrap();
}

impl str::FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = RE_RULE.captures(s).unwrap();
        let value = c.get(2).unwrap().as_str().parse().unwrap();
        match c.get(1).unwrap().as_str() {
            "forward" => Ok(Command::Forward(value)),
            "up" => Ok(Command::Up(value)),
            "down" => Ok(Command::Down(value)),
            _ => Err(anyhow!("couldn't parse {}", s)),
        }
    }
}

#[derive(Default)]
struct CommandResult {
    pos: usize,
    depth: usize,
    aim: usize,
}

impl CommandResult {
    fn value(self) -> usize {
        self.pos * self.depth
    }
}

fn cycle(commands: &[Command]) -> CommandResult {
    commands.iter().fold(CommandResult::default(), |mut r, c| {
        match c {
            Command::Forward(value) => r.pos += value,
            Command::Up(value) => r.depth -= value,
            Command::Down(value) => r.depth += value,
        };
        r
    })
}

fn cycle_v2(commands: &[Command]) -> CommandResult {
    commands.iter().fold(CommandResult::default(), |mut r, c| {
        match c {
            Command::Forward(value) => {
                r.pos += value;
                r.depth += r.aim * value;
            }
            Command::Up(value) => r.aim -= value,
            Command::Down(value) => r.aim += value,
        };
        r
    })
}

fn main() {
    let file = fs::File::open("aoc2021/inputs/day02.input").unwrap();
    let commands: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    println!("part 1: {}", cycle(&commands).value());
    println!("part 2: {}", cycle_v2(&commands).value());
}
