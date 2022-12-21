use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    str,
};

use anyhow::{bail, Result};
use utils::reverse;

type Reg = usize;

enum Operand {
    Reg(Reg),
    Num(i64),
}

enum Instruction {
    Inp(Reg),
    Add(Reg, Operand),
    Mul(Reg, Operand),
    Div(Reg, Operand),
    Mod(Reg, Operand),
    Eql(Reg, Operand),
}

impl str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inst = &s[0..3];
        let parse_reg = |s: &str| (s.as_bytes()[0] - b'w') as usize;
        let reg = parse_reg(&s[4..5]);
        if inst == "inp" {
            return Ok(Instruction::Inp(reg));
        }
        let op = s[6..]
            .parse()
            .map(Operand::Num)
            .unwrap_or_else(|_| Operand::Reg(parse_reg(&s[6..])));
        match &s[..3] {
            "add" => Ok(Instruction::Add(reg, op)),
            "mul" => Ok(Instruction::Mul(reg, op)),
            "div" => Ok(Instruction::Div(reg, op)),
            "mod" => Ok(Instruction::Mod(reg, op)),
            "eql" => Ok(Instruction::Eql(reg, op)),
            _ => bail!("couldn't parse {}", s),
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq, Hash)]
struct Alu {
    pc: usize,
    regs: [i64; 4],
}

impl Alu {
    fn resolve(&self, op: &Operand) -> i64 {
        match op {
            Operand::Reg(var) => self.regs[*var],
            Operand::Num(n) => *n,
        }
    }

    fn apply(&mut self, prog: &[Instruction], input: i64) {
        match &prog[self.pc] {
            Instruction::Inp(reg) => self.regs[*reg] = input,
            Instruction::Add(reg, op) => self.regs[*reg] += self.resolve(op),
            Instruction::Mul(reg, op) => self.regs[*reg] *= self.resolve(op),
            Instruction::Div(reg, op) => self.regs[*reg] /= self.resolve(op),
            Instruction::Mod(reg, op) => self.regs[*reg] %= self.resolve(op),
            Instruction::Eql(reg, op) => {
                self.regs[*reg] = (self.regs[*reg] == self.resolve(op)) as i64;
            }
        }
        self.pc += 1;
    }
}

fn solve(
    prog: &[Instruction],
    alu: Alu,
    cutoff: &mut HashSet<Alu>,
    monad: impl Iterator<Item = i64> + Clone,
) -> Option<i64> {
    if cutoff.contains(&alu) {
        return None;
    }
    for input in monad.clone() {
        let mut alu = alu.clone();
        alu.apply(prog, input);
        while alu.pc < prog.len() {
            if matches!(prog[alu.pc], Instruction::Inp(_)) {
                if let Some(answer) = solve(prog, alu.clone(), cutoff, monad.clone()) {
                    return Some(answer * 10 + input);
                }
                break;
            }
            alu.apply(prog, input);
        }
        if alu.pc == prog.len() && alu.regs[(b'z' - b'w') as usize] == 0 {
            return Some(input);
        }
    }
    cutoff.insert(alu);
    None
}

fn main() {
    let file = File::open("aoc2021/inputs/day24.input").unwrap();
    let prog: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    let mut visited = HashSet::new();
    let p1 = solve(&prog, Alu::default(), &mut visited, (1..=9).rev()).unwrap();
    println!("part 1: {}", reverse(p1 as usize));
    let p2 = solve(&prog, Alu::default(), &mut visited, 1..=9).unwrap();
    println!("part 2: {}", reverse(p2 as usize));
}
