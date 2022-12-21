use std::{
    fs,
    io::{self, BufRead},
    str,
};

use anyhow::{anyhow, Result};
use itertools::iterate;

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
}

impl str::FromStr for Register {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            _ => Err(anyhow!("couldn't parse {}", s)),
        }
    }
}

type Offset = i32;

#[derive(Debug)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(Offset),
    Jie(Register, Offset),
    Jio(Register, Offset),
}

impl str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..3] {
            "hlf" => Ok(Instruction::Hlf(s[4..].parse()?)),
            "tpl" => Ok(Instruction::Tpl(s[4..].parse()?)),
            "inc" => Ok(Instruction::Inc(s[4..].parse()?)),
            "jmp" => Ok(Instruction::Jmp(s[4..].parse()?)),
            "jie" => Ok(Instruction::Jie(s[4..5].parse()?, s[7..].parse()?)),
            "jio" => Ok(Instruction::Jio(s[4..5].parse()?, s[7..].parse()?)),
            _ => Err(anyhow!("couldn't parse {}", s)),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct VM {
    a: usize,
    b: usize,
    pc: usize,
}

impl VM {
    fn modify(&mut self, reg: Register, f: impl Fn(usize) -> usize) -> Offset {
        match reg {
            Register::A => self.a = f(self.a),
            Register::B => self.b = f(self.b),
        }
        1
    }

    fn value(&self, reg: Register) -> usize {
        match reg {
            Register::A => self.a,
            Register::B => self.b,
        }
    }
}

fn execute(vm: &VM, program: &[Instruction]) -> Option<VM> {
    program.get(vm.pc).map(|inst| {
        let mut new_vm = vm.clone();
        let offset = match inst {
            Instruction::Hlf(r) => new_vm.modify(*r, |v| v / 2),
            Instruction::Tpl(r) => new_vm.modify(*r, |v| v * 3),
            Instruction::Inc(r) => new_vm.modify(*r, |v| v + 1),
            Instruction::Jmp(offset) => *offset,
            Instruction::Jie(r, offset) if vm.value(*r) % 2 == 0 => *offset,
            Instruction::Jio(r, offset) if vm.value(*r) == 1 => *offset,
            _ => 1,
        };
        new_vm.pc = vm.pc.wrapping_add(offset as usize);
        new_vm
    })
}

fn cycle(program: &[Instruction], initial: VM) -> VM {
    iterate(Some(initial), |vm| {
        vm.as_ref().and_then(|vm| execute(vm, program))
    })
    .take_while(|x| x.is_some())
    .last()
    .flatten()
    .unwrap()
}

fn main() {
    let file = fs::File::open("aoc2015/inputs/day23.input").unwrap();
    let program: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    println!("part 1: {}", cycle(&program, VM::default()).b);
    println!(
        "part 2: {}",
        cycle(
            &program,
            VM {
                a: 1,
                ..VM::default()
            }
        )
        .b
    );
}
