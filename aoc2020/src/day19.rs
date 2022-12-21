use std::{
    collections::HashMap,
    fs,
    str::{Chars, FromStr},
};

use anyhow::anyhow;
use regex::{Captures, Regex};

type Sequence = Vec<usize>;

#[derive(Debug)]
enum Rule {
    Match(char),
    Seq(Sequence),
    Or(Sequence, Sequence),
}

#[derive(Debug)]
struct NumberedRule {
    idx: usize,
    rule: Rule,
}

fn matches<'a>(
    n: usize,
    s: &[usize],
    cs: Chars<'a>,
    rules: &HashMap<usize, NumberedRule>,
    max_depth: usize,
) -> Vec<Chars<'a>> {
    match s.first().and_then(|idx| rules.get(idx)) {
        None => vec![cs],
        Some(r) if r.idx == n && max_depth == 0 => vec![],
        Some(r) => match_internal(
            r,
            cs,
            rules,
            if r.idx == n { max_depth - 1 } else { max_depth },
        )
        .into_iter()
        .flat_map(|cs| matches(n, &s[1..], cs, rules, max_depth))
        .collect(),
    }
}

fn match_internal<'a>(
    r: &NumberedRule,
    mut cs: Chars<'a>,
    rules: &HashMap<usize, NumberedRule>,
    max_depth: usize,
) -> Vec<Chars<'a>> {
    match &r.rule {
        Rule::Match(c) if cs.next() == Some(*c) => vec![cs],
        Rule::Match(_) => vec![],
        Rule::Seq(s) => matches(r.idx, s, cs, rules, max_depth),
        Rule::Or(s1, s2) => vec![s1, s2]
            .into_iter()
            .flat_map(|s| matches(r.idx, s, cs.clone(), rules, max_depth))
            .collect(),
    }
}

impl NumberedRule {
    fn matches(&self, s: &str, rules: &HashMap<usize, NumberedRule>) -> bool {
        let res = match_internal(self, s.chars(), rules, 4);
        res.into_iter().any(|mut cs| cs.next() == None)
    }
}

lazy_static::lazy_static! {
    static ref RE_MATCH: Regex = Regex::new(r#"^(\d+): "([a-z])"$"#).unwrap();
    static ref RE_SEQ: Regex = Regex::new(r#"^(\d+): ([\d ]+)$"#).unwrap();
    static ref RE_OR: Regex = Regex::new(r#"^(\d+): ([\d ]+) \| ([\d ]+)$"#).unwrap();
}

impl FromStr for NumberedRule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_idx = |r: &Captures| r.get(1).unwrap().as_str().parse().unwrap();
        let parse_seq = |r: &Captures, i: usize| {
            r.get(i)
                .unwrap()
                .as_str()
                .split(' ')
                .filter_map(|s| match s.parse() {
                    Ok(num) => Some(num),
                    Err(_) => None,
                })
                .collect::<Vec<usize>>()
        };
        if let Some(g) = RE_MATCH.captures(s) {
            Ok(NumberedRule {
                idx: parse_idx(&g),
                rule: Rule::Match(g.get(2).unwrap().as_str().parse()?),
            })
        } else if let Some(g) = RE_SEQ.captures(s) {
            Ok(NumberedRule {
                idx: parse_idx(&g),
                rule: Rule::Seq(parse_seq(&g, 2)),
            })
        } else if let Some(g) = RE_OR.captures(s) {
            Ok(NumberedRule {
                idx: parse_idx(&g),
                rule: Rule::Or(parse_seq(&g, 2), parse_seq(&g, 3)),
            })
        } else {
            Err(anyhow!("failed to parse rule"))
        }
    }
}

fn main() {
    let input = fs::read_to_string("aoc2020/inputs/day19.input").unwrap();
    let mut sections = input.split("\n\n");
    let mut rules: HashMap<usize, NumberedRule> = sections
        .next()
        .unwrap()
        .lines()
        .map(|l| l.parse::<NumberedRule>().unwrap())
        .map(|nr| (nr.idx, nr))
        .collect();
    let lines: Vec<_> = sections.next().unwrap().lines().collect();
    println!(
        "part 1: {}",
        lines
            .iter()
            .filter(|l| rules.get(&0).unwrap().matches(l, &rules))
            .count()
    );
    rules
        .entry(8)
        .and_modify(|nr| nr.rule = Rule::Or(vec![42], vec![42, 8]));
    rules
        .entry(11)
        .and_modify(|nr| nr.rule = Rule::Or(vec![42, 31], vec![42, 11, 31]));
    println!(
        "part 2: {}",
        lines
            .iter()
            .filter(|l| rules.get(&0).unwrap().matches(l, &rules))
            .count()
    );
}
