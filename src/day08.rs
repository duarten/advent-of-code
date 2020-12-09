use anyhow::{anyhow, Result};
use std::collections;
use std::fmt;
use std::fs;
use std::io::{self, BufRead};
use std::str;

#[derive(Debug)]
enum ExecError {
    InfiniteLoop(i64),
    InvalidPc(usize),
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecError::InfiniteLoop(x) => write!(f, "Infinite loop with acc {}", x),
            ExecError::InvalidPc(x) => write!(f, "Invalid PC {}", x),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let offset: i64 = s[4..].parse()?;
        match &s[..3] {
            "acc" => Ok(Instruction::Acc(offset)),
            "jmp" => Ok(Instruction::Jmp(offset)),
            "nop" => Ok(Instruction::Nop(offset)),
            _ => Err(anyhow!("couldn't parse {}", s)),
        }
    }
}

#[derive(Debug, Default)]
struct VM {
    pc: usize,
    acc: i64,
    executed: collections::HashSet<usize>,
}

enum State {
    Continue,
    Done,
}

impl VM {
    fn execute(&mut self, program: &[Instruction]) -> Result<State, ExecError> {
        let pc = self.pc;
        if !self.executed.insert(pc) {
            return Err(ExecError::InfiniteLoop(self.acc));
        }
        let inst = program.get(pc);
        self.pc += 1;
        match inst {
            Some(Instruction::Acc(x)) => self.acc += x,
            Some(Instruction::Jmp(x)) => self.pc = pc.wrapping_add(*x as usize),
            Some(Instruction::Nop(_)) => {}
            None => return Err(ExecError::InvalidPc(pc)),
        }
        Ok(if self.pc == program.len() {
            State::Done
        } else {
            State::Continue
        })
    }

    fn cycle(mut self, program: &[Instruction]) -> Result<Self, ExecError> {
        let mut res = Ok(State::Continue);
        while let Ok(State::Continue) = res {
            res = self.execute(program)
        }
        match res {
            Ok(_) => Ok(self),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    let file = fs::File::open("inputs/day08.input").unwrap();
    let mut program: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    println!("{}", VM::default().cycle(&program).err().unwrap());

    for inst in 0..program.len() {
        let replace = match program[inst] {
            Instruction::Nop(offset) => Instruction::Jmp(offset),
            Instruction::Jmp(offset) => Instruction::Nop(offset),
            _ => continue,
        };
        let old = std::mem::replace(&mut program[inst], replace);
        if let Ok(VM { acc, .. }) = VM::default().cycle(&program) {
            println!("acc: {}", acc);
            break;
        }
        program[inst] = old;
    }
}
