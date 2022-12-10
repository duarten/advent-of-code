use std::convert::Infallible;
use std::fs;
use std::io::{self, BufRead};
use std::str;

enum Instruction {
    Add(isize),
    Noop,
}

impl str::FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..4] {
            "addx" => Ok(Instruction::Add(s[5..].parse().unwrap())),
            "noop" => Ok(Instruction::Noop),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
struct VM {
    x: isize,
    cycle: usize,
}

impl VM {
    fn apply(&mut self, inst: Instruction) -> Vec<VM> {
        self.cycle += 1;
        let mut res = vec![*self];
        if let Instruction::Add(n) = inst {
            self.cycle += 1;
            res.push(*self);
            self.x += n;
        }
        res
    }
}

fn main() {
    let file = fs::File::open("aoc2022/inputs/day10.input").unwrap();
    let mut vm = VM { cycle: 0, x: 1 };
    let program = io::BufReader::new(file)
        .lines()
        .flat_map(|l| vm.apply(l.unwrap().parse().unwrap()));
    let mut signal_strength = 0;
    let mut pixels = Vec::with_capacity(240);
    for VM { cycle, x } in program {
        if cycle % 40 == 20 {
            signal_strength += cycle as isize * x;
        }
        pixels.push(if x.abs_diff((cycle as isize - 1) % 40) < 2 {
            '#'
        } else {
            '.'
        });
    }
    println!("part 1: {}", signal_strength);
    println!("part 2:");
    for (idx, pixel) in pixels.into_iter().enumerate() {
        print!("{pixel}");
        if idx % 40 == 39 {
            println!();
        }
    }
}
