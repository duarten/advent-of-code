use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

type WireId = String;

type Wires = HashMap<WireId, u16>;

#[derive(Clone)]
enum WireOrValue {
    Wire(WireId),
    Value(u16),
}

impl WireOrValue {
    fn resolve(&self, wires: &Wires) -> Option<u16> {
        match self {
            WireOrValue::Wire(w) => wires.get(w).copied(),
            WireOrValue::Value(v) => Some(*v),
        }
    }
}

trait Instruction {
    fn exec(&self, w: &mut Wires) -> Option<()>;
}

#[derive(Clone)]
struct SignalData {
    src: WireOrValue,
    to: WireId,
}

impl Instruction for SignalData {
    fn exec(&self, w: &mut Wires) -> Option<()> {
        self.src.resolve(w).map(|v| {
            w.insert(self.to.clone(), v);
        })
    }
}

#[derive(Clone)]
struct NotData {
    negate: WireOrValue,
    to: WireId,
}

impl Instruction for NotData {
    fn exec(&self, w: &mut Wires) -> Option<()> {
        self.negate.resolve(w).map(|v| {
            w.insert(self.to.clone(), !v);
        })
    }
}

#[derive(Clone)]
enum BinaryOp {
    And,
    Or,
    Lshift,
    Rshift,
}

#[derive(Clone)]
struct BinaryInstructionData {
    l: WireOrValue,
    r: WireOrValue,
    op: BinaryOp,
    to: WireId,
}

impl Instruction for BinaryInstructionData {
    fn exec(&self, w: &mut Wires) -> Option<()> {
        self.l.resolve(w).zip(self.r.resolve(w)).map(|(l, r)| {
            let r = match self.op {
                BinaryOp::And => l & r,
                BinaryOp::Or => l | r,
                BinaryOp::Lshift => l << r,
                BinaryOp::Rshift => l >> r,
            };
            w.insert(self.to.clone(), r);
        })
    }
}

#[derive(Clone)]
enum Instructions {
    Signal(SignalData),
    Not(NotData),
    Binary(BinaryInstructionData),
}

impl Instructions {
    fn exec(&self, w: &mut Wires) -> Option<()> {
        match self {
            Instructions::Signal(d) => d.exec(w),
            Instructions::Not(d) => d.exec(w),
            Instructions::Binary(d) => d.exec(w),
        }
    }
}

fn eval(mut insts: Vec<Instructions>) -> u16 {
    let mut wires = Wires::new();
    while !insts.is_empty() {
        insts.retain(|inst| matches!(inst.exec(&mut wires), None))
    }
    wires["a"]
}

lazy_static::lazy_static! {
    static ref RE_SIGNAL: Regex = Regex::new(r#"^(\S+) -> (\w+)$"#).unwrap();
    static ref RE_NOT: Regex = Regex::new(r#"^NOT (\S+) -> (\w+)$"#).unwrap();
    static ref RE_BINARY: Regex = Regex::new(r#"^(\S+) (\w+) (\S+) -> (\w+)$"#).unwrap();
}

fn main() {
    let file = File::open("aoc2015/inputs/day07.input").unwrap();
    let mut instructions: Vec<_> = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            let operand = |o: &str| match o.parse::<u16>() {
                Ok(val) => WireOrValue::Value(val),
                Err(_) => WireOrValue::Wire(o.to_owned()),
            };
            if let Some(g) = RE_SIGNAL.captures(&l) {
                Instructions::Signal(SignalData {
                    src: operand(g.get(1).unwrap().as_str()),
                    to: g.get(2).unwrap().as_str().to_owned(),
                })
            } else if let Some(g) = RE_NOT.captures(&l) {
                Instructions::Not(NotData {
                    negate: operand(g.get(1).unwrap().as_str()),
                    to: g.get(2).unwrap().as_str().to_owned(),
                })
            } else {
                let g = RE_BINARY.captures(&l).unwrap();
                Instructions::Binary(BinaryInstructionData {
                    l: operand(g.get(1).unwrap().as_str()),
                    r: operand(g.get(3).unwrap().as_str()),
                    op: match g.get(2).unwrap().as_str() {
                        "AND" => BinaryOp::And,
                        "OR" => BinaryOp::Or,
                        "LSHIFT" => BinaryOp::Lshift,
                        "RSHIFT" => BinaryOp::Rshift,
                        _ => panic!(),
                    },
                    to: g.get(4).unwrap().as_str().to_owned(),
                })
            }
        })
        .collect();
    let pt1 = eval(instructions.clone());
    println!("part 1: {}", pt1);
    let pos = instructions
        .iter()
        .position(|inst| matches!(inst, Instructions::Signal(SignalData { to: b, .. }) if b == "b"))
        .unwrap();
    instructions.swap_remove(pos);
    instructions.push(Instructions::Signal(SignalData {
        src: WireOrValue::Value(pt1),
        to: "b".to_owned(),
    }));
    println!("part 2: {}", eval(instructions));
}
