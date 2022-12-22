use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, i64},
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug)]
enum Job {
    Num(i64),
    Sum(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

impl Job {
    fn apply(&self, monkeys: &HashMap<String, Job>) -> i64 {
        match self {
            Job::Num(n) => *n,
            Job::Sum(m1, m2) => monkeys[m1].apply(monkeys) + monkeys[m2].apply(monkeys),
            Job::Sub(m1, m2) => monkeys[m1].apply(monkeys) - monkeys[m2].apply(monkeys),
            Job::Mul(m1, m2) => monkeys[m1].apply(monkeys) * monkeys[m2].apply(monkeys),
            Job::Div(m1, m2) => monkeys[m1].apply(monkeys) / monkeys[m2].apply(monkeys),
        }
    }

    fn path_to<'a>(
        &self,
        monkeys: &'a HashMap<String, Job>,
        target: &str,
        path: &mut HashSet<String>,
    ) -> bool {
        match self {
            Job::Num(_) => false,
            Job::Sum(m1, m2) | Job::Sub(m1, m2) | Job::Mul(m1, m2) | Job::Div(m1, m2) => {
                for m in [m1, m2] {
                    if m == target || monkeys[m].path_to(monkeys, target, path) {
                        path.insert(m.clone());
                        return true;
                    }
                }
                false
            }
        }
    }
}

fn path_to<'a>(monkeys: &'a HashMap<String, Job>, start: &str, key: &str) -> HashSet<String> {
    let mut result = HashSet::new();
    monkeys[start].path_to(monkeys, key, &mut result);
    result
}

fn solve(
    monkeys: &HashMap<String, Job>,
    current: &str,
    target: &str,
    path: &HashSet<String>,
    result: i64,
) -> i64 {
    if current == target {
        return result;
    }
    if !path.contains(current) {
        return monkeys[current].apply(monkeys);
    }
    let (result, next) = match &monkeys[current] {
        Job::Num(result) => return *result,
        Job::Sum(m1, m2) => {
            if path.contains(m1) {
                (result - monkeys[m2].apply(monkeys), m1)
            } else {
                (result - monkeys[m1].apply(monkeys), m2)
            }
        }
        Job::Sub(m1, m2) => {
            if path.contains(m1) {
                (result + monkeys[m2].apply(monkeys), m1)
            } else {
                (monkeys[m1].apply(monkeys) - result, m2)
            }
        }
        Job::Mul(m1, m2) => {
            if path.contains(m1) {
                (result / monkeys[m2].apply(monkeys), m1)
            } else {
                (result / monkeys[m1].apply(monkeys), m2)
            }
        }
        Job::Div(m1, m2) => {
            if path.contains(m1) {
                (result * monkeys[m2].apply(monkeys), m1)
            } else {
                (monkeys[m1].apply(monkeys) / result, m2)
            }
        }
    };
    solve(monkeys, next, target, path, result)
}

fn monkey(input: &str) -> IResult<&str, (String, Job)> {
    map(
        tuple((
            map(alpha1, |s: &str| s.to_owned()),
            tag(": "),
            alt((
                map(i64, Job::Num),
                map(
                    tuple((
                        map(alpha1, |s: &str| s.to_owned()),
                        delimited(tag(" "), anychar, tag(" ")),
                        map(alpha1, |s: &str| s.to_owned()),
                    )),
                    |(m1, op, m2)| match op {
                        '-' => Job::Sub(m1, m2),
                        '+' => Job::Sum(m1, m2),
                        '*' => Job::Mul(m1, m2),
                        '/' => Job::Div(m1, m2),
                        _ => panic!("Unknown op: {}", op),
                    },
                ),
            )),
        )),
        |(m, _, job)| (m, job),
    )(input)
}

fn main() {
    let file = File::open("aoc2022/inputs/day21.input").unwrap();
    let monkeys = BufReader::new(file)
        .lines()
        .map(|l| monkey(&l.unwrap()).unwrap().1)
        .collect::<HashMap<_, _>>();

    println!("part 1: {}", monkeys["root"].apply(&monkeys));

    let path = path_to(&monkeys, "root", "humn");
    let (result, start) = match &monkeys["root"] {
        Job::Sum(m1, m2) | Job::Sub(m1, m2) | Job::Mul(m1, m2) | Job::Div(m1, m2) => {
            if !path.contains(m1) {
                (monkeys[m1].apply(&monkeys), m2)
            } else {
                (monkeys[m2].apply(&monkeys), m1)
            }
        }
        Job::Num(_) => unreachable!(),
    };
    println!("part 2: {}", solve(&monkeys, start, "humn", &path, result));
}
