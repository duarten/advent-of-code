use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;

lazy_static::lazy_static! {
    static ref RE_RULE: Regex = Regex::new(r#"^([a-z ]+) bags contain (.*)$"#).unwrap();
    static ref RE_CONT: Regex = Regex::new(r#"(\d) ([a-z ]+) b"#).unwrap();
}

type Bag<'a> = &'a str;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Container<'a> {
    times_contained: usize,
    bag: Bag<'a>,
}

fn do_find_containers<'a>(
    b: Bag<'a>,
    m: &HashMap<Bag<'a>, Vec<Bag<'a>>>,
    ret: &mut HashSet<Bag<'a>>,
) {
    match m.get(b) {
        Some(c) => c.iter().for_each(|c| {
            ret.insert(c);
            do_find_containers(c, m, ret)
        }),
        None => {}
    }
}

fn find_containers<'a>(b: Bag<'a>, m: &HashMap<Bag<'a>, Vec<Bag<'a>>>) -> HashSet<Bag<'a>> {
    let mut ret = HashSet::<Bag<'a>>::new();
    do_find_containers(b, m, &mut ret);
    ret
}

fn count_bags<'a>(b: Bag<'a>, m: &HashMap<Bag, HashSet<Container>>) -> usize {
    1 + match m.get(b) {
        Some(c) => c
            .iter()
            .map(|b| count_bags(b.bag, m) * b.times_contained)
            .sum(),
        None => 0,
    }
}

fn main() {
    let file = File::open("inputs/day07.input").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let (index, rev_index) = lines.iter().fold(
        (
            HashMap::<Bag, HashSet<Container>>::new(),
            HashMap::<Bag, Vec<Bag>>::new(),
        ),
        |(mut index, mut rev_index), l| {
            parse_line(l).into_iter().for_each(|(bag, cnt, contains)| {
                index
                    .entry(bag)
                    .or_insert(HashSet::<Container>::new())
                    .insert(Container {
                        times_contained: cnt,
                        bag: contains,
                    });
                rev_index
                    .entry(contains)
                    .or_insert(Vec::<Bag>::new())
                    .push(bag);
            });
            (index, rev_index)
        },
    );
    println!(
        "Containers for 'shiny gold': {:?}",
        find_containers("shiny gold", &rev_index).len()
    );
    println!(
        "Contained in 'shiny gold': {:?}",
        count_bags("shiny gold", &index) - 1
    );
}

fn parse_line<'a>(rule: &'a str) -> Vec<(Bag<'a>, usize, Bag<'a>)> {
    let captures = RE_RULE.captures(rule).unwrap();
    let container = captures.get(1).unwrap().as_str();
    RE_CONT
        .captures_iter(captures.get(2).unwrap().as_str())
        .map(|cond| {
            (
                container,
                cond.get(1).unwrap().as_str().parse().unwrap(),
                cond.get(2).unwrap().as_str(),
            )
        })
        .collect()
}
