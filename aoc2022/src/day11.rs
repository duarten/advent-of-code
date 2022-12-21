use std::{collections::VecDeque, convert::Infallible, fs, str::FromStr};

#[derive(Clone)]
enum Operand {
    Literal(usize),
    Old,
}

impl FromStr for Operand {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.parse().map(Operand::Literal).unwrap_or(Operand::Old))
    }
}

#[derive(Clone)]
enum Op {
    Add(Operand),
    Mul(Operand),
}

impl FromStr for Op {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, value) = s.split_once(' ').unwrap();
        match op {
            "+" => Ok(Self::Add(value.parse().unwrap())),
            "*" => Ok(Self::Mul(value.parse().unwrap())),
            _ => unreachable!(),
        }
    }
}

impl Op {
    fn apply(&self, value: usize) -> usize {
        match self {
            Self::Add(Operand::Literal(literal)) => value + literal,
            Self::Add(Operand::Old) => value + value,
            Self::Mul(Operand::Literal(literal)) => value * literal,
            Self::Mul(Operand::Old) => value * value,
        }
    }
}

type Index = usize;

#[derive(Clone)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Op,
    test: usize,
    on_true: Index,
    on_false: Index,
    inspected: usize,
}

impl FromStr for Monkey {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next();
        Ok(Monkey {
            items: lines.next().unwrap()["  Starting items: ".len()..]
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect(),
            operation: lines.next().unwrap()["  Operation: new = old ".len()..]
                .parse()
                .unwrap(),
            test: lines.next().unwrap()["  Test: divisible by ".len()..]
                .parse()
                .unwrap(),
            on_true: lines.next().unwrap()["    If true: throw to monkey ".len()..]
                .parse()
                .unwrap(),
            on_false: lines.next().unwrap()["    If false: throw to monkey ".len()..]
                .parse()
                .unwrap(),
            inspected: 0,
        })
    }
}

fn turn(monkeys: &mut [Monkey], current: usize, decrease: impl Fn(usize) -> usize) {
    while let Some(worry) = monkeys[current].items.pop_front() {
        monkeys[current].inspected += 1;
        let worry = decrease(monkeys[current].operation.apply(worry));
        let target = if worry % monkeys[current].test as usize == 0 {
            monkeys[current].on_true
        } else {
            monkeys[current].on_false
        };
        monkeys[target].items.push_back(worry);
    }
}

fn cycle(
    mut monkeys: Vec<Monkey>,
    rounds: usize,
    decrease: impl Fn(usize) -> usize + Copy,
) -> usize {
    for _ in 0..rounds {
        (0..monkeys.len()).for_each(|m| turn(&mut monkeys, m, decrease));
    }
    monkeys.sort_unstable_by(|m1, m2| m2.inspected.cmp(&m1.inspected));
    monkeys[0].inspected * monkeys[1].inspected
}

fn main() {
    let file = fs::read_to_string("aoc2022/inputs/day11.input").unwrap();
    let monkeys = file
        .split_terminator("\n\n")
        .flat_map(Monkey::from_str)
        .collect::<Vec<_>>();
    let lcm = monkeys.iter().map(|m| m.test).product::<usize>();
    println!("part 1: {}", cycle(monkeys.clone(), 20, |w| w / 3));
    println!("part 2: {}", cycle(monkeys, 10_000, |w| w % lcm));
}
