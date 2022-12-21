use std::{collections, fs, ops, str::FromStr};

use anyhow::{anyhow, Result};
use regex::Regex;

#[derive(Debug)]
struct Ticket {
    values: Vec<usize>,
}

impl FromStr for Ticket {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Result<Vec<_>> = s
            .split(',')
            .map(|p| match p.parse() {
                Err(_) => Err(anyhow!("Error parsing number")),
                Ok(x) => Ok(x),
            })
            .collect();
        Ok(Ticket { values: values? })
    }
}

#[derive(Debug)]
struct Rule {
    name: String,
    r1: ops::RangeInclusive<usize>,
    r2: ops::RangeInclusive<usize>,
}

impl Rule {
    fn is_valid(&self, v: &usize) -> bool {
        self.r1.contains(v) || self.r2.contains(v)
    }
}

lazy_static::lazy_static! {
    static ref RE_RULE: Regex = Regex::new(r#"^([\w\s]+): (\d+-\d+) or (\d+-\d+)$"#).unwrap();
}

fn parse_range(r: &str) -> ops::RangeInclusive<usize> {
    let mut p = r.split('-');
    ops::RangeInclusive::new(
        p.next().unwrap().parse().unwrap(),
        p.next().unwrap().parse().unwrap(),
    )
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE_RULE.captures(s).unwrap();
        Ok(Rule {
            name: captures.get(1).unwrap().as_str().to_owned(),
            r1: parse_range(captures.get(2).unwrap().as_str()),
            r2: parse_range(captures.get(3).unwrap().as_str()),
        })
    }
}

#[derive(Default)]
struct CandidateRules<'a> {
    field: usize,
    rules: Vec<&'a Rule>,
}

fn field_positions(ts: &[Ticket], rs: &[Rule]) -> collections::HashMap<String, usize> {
    let n = ts[0].values.len();
    let mut ret = collections::HashMap::<String, usize>::new();
    let mut candidates = Vec::<CandidateRules>::with_capacity(n);
    for i in 0..n {
        let applied_rules: Vec<&Rule> = rs
            .iter()
            .filter(|r| ts.iter().all(|t| r.is_valid(&t.values[i])))
            .collect();
        candidates.push(CandidateRules {
            field: i,
            rules: applied_rules,
        });
    }
    candidates.sort_by(|a, b| a.rules.len().cmp(&b.rules.len()));
    for c in candidates {
        for r in c.rules {
            if ret.get(&r.name).is_none() {
                ret.insert(r.name.clone(), c.field);
            }
        }
    }
    ret
}

fn main() {
    let input = fs::read_to_string("aoc2020/inputs/day16.input").unwrap();
    let mut sections = input.split("\n\n");
    let rules: Vec<Rule> = sections
        .next()
        .unwrap()
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();
    let my_ticket: Ticket = sections
        .next()
        .unwrap()
        .lines()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let other_tickets: Vec<_> = sections
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|t| t.parse())
        .collect::<Result<Vec<Ticket>>>()
        .unwrap();
    println!(
        "part 1: {:?}",
        other_tickets
            .iter()
            .flat_map(|t| t
                .values
                .iter()
                .filter(|v| rules.iter().all(|r| !r.is_valid(v))))
            .sum::<usize>()
    );
    let valid_tickets: Vec<_> = other_tickets
        .into_iter()
        .filter(|t| t.values.iter().all(|v| rules.iter().any(|r| r.is_valid(v))))
        .collect();
    let pos = field_positions(&valid_tickets, &rules);
    let res: usize = pos
        .into_iter()
        .filter_map(|(n, p)| {
            if n.starts_with("departure") {
                Some(my_ticket.values[p])
            } else {
                None
            }
        })
        .product();
    println!("part 2: {}", res);
}
