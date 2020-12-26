use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
enum Op {
    Add,
    Mul,
}

#[derive(Clone)]
enum Exp {
    Num(u64),
    Binary(Box<Exp>, Box<Exp>, Op),
}

impl Exp {
    fn eval(&self) -> u64 {
        match self {
            Exp::Num(x) => *x,
            Exp::Binary(l, r, Op::Add) => l.eval() + r.eval(),
            Exp::Binary(l, r, Op::Mul) => l.eval() * r.eval(),
        }
    }

    fn plus_first(&self) -> Exp {
        match self {
            Exp::Binary(l, r, Op::Add) => match l.plus_first() {
                Exp::Binary(subl, subr, Op::Mul) => Exp::Binary(
                    subl,
                    Box::new(Exp::Binary(subr, Box::new(r.plus_first()), Op::Add)),
                    Op::Mul,
                ),
                n => Exp::Binary(Box::new(n), Box::new(r.plus_first()), Op::Add),
            },
            Exp::Binary(l, r, op) => {
                Exp::Binary(Box::new(l.plus_first()), Box::new(r.plus_first()), *op)
            }
            n => n.clone(),
        }
    }
}

impl FromStr for Exp {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut groups = Vec::<(Exp, Op)>::new();
        Ok(s.chars()
            .fold((Exp::Num(0), Op::Add), |(exp, op), c| match c {
                ' ' => (exp, op),
                '(' => {
                    groups.push((exp, op));
                    (Exp::Num(0), Op::Add)
                }
                ')' => {
                    let (prev_exp, prev_op) = groups.pop().unwrap();
                    (Exp::Binary(Box::new(prev_exp), Box::new(exp), prev_op), op)
                }
                '+' => (exp, Op::Add),
                '*' => (exp, Op::Mul),
                x => (
                    Exp::Binary(
                        Box::new(exp),
                        Box::new(Exp::Num(x.to_digit(10).unwrap().into())),
                        op,
                    ),
                    op,
                ),
            })
            .0)
    }
}

fn main() {
    let file = File::open("aoc2020/inputs/day18.input").unwrap();
    let exps: Vec<Exp> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    println!("part 1: {}", exps.iter().map(|exp| exp.eval()).sum::<u64>());
    println!(
        "part 2: {}",
        exps.iter().map(|exp| exp.plus_first().eval()).sum::<u64>()
    );
}
