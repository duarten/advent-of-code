use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

type Person = String;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Pair(Person, Person);

fn do_visit(
    to_visit: Vec<&String>,
    pairs: &HashMap<Pair, i32>,
    prev: Option<String>,
    first: Option<String>,
) -> i32 {
    (0..to_visit.len())
        .map(|i| {
            let cost = prev
                .as_ref()
                .map(|p| {
                    pairs
                        .get(&Pair(to_visit[i].clone(), p.clone()))
                        .and_then(|h| Some(h + pairs.get(&Pair(p.clone(), to_visit[i].clone()))?))
                        .unwrap_or(0)
                })
                .unwrap_or(0);
            let mut n = to_visit.clone();
            n.swap_remove(i);
            cost + (if !n.is_empty() {
                do_visit(
                    n,
                    pairs,
                    Some(to_visit[i].clone()),
                    first.clone().or_else(|| Some(to_visit[i].clone())),
                )
            } else {
                first
                    .as_ref()
                    .and_then(|f| {
                        pairs
                            .get(&Pair(to_visit[0].clone(), f.clone()))
                            .and_then(|h| {
                                Some(h + pairs.get(&Pair(f.clone(), to_visit[0].clone()))?)
                            })
                    })
                    .unwrap_or(0)
            })
        })
        .max()
        .unwrap_or(0)
}

fn visit(pairs: &HashMap<Pair, i32>) -> i32 {
    do_visit(
        pairs
            .keys()
            .map(|p| &p.0)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect(),
        pairs,
        None,
        None,
    )
}

lazy_static::lazy_static! {
    static ref RE_HAP: Regex = Regex::new(r#"^([\w]+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)."#).unwrap();
}

fn main() {
    let file = File::open("aoc2015/inputs/day13.input").unwrap();
    let mut pairs = HashMap::<Pair, i32>::new();
    for l in BufReader::new(file).lines().map(Result::unwrap) {
        if let Some(g) = RE_HAP.captures(&l) {
            let p1 = g.get(1).unwrap().as_str().to_owned();
            let p2 = g.get(4).unwrap().as_str().to_owned();
            let mut happiness: i32 = g.get(3).unwrap().as_str().parse().unwrap();
            if g.get(2).unwrap().as_str() == "lose" {
                happiness = -happiness;
            }
            pairs.insert(Pair(p1, p2), happiness);
        }
    }
    println!("part 1: {}", visit(&pairs));
    let me = "me".to_owned();
    pairs.extend(
        pairs
            .keys()
            .map(|p| &p.0)
            .collect::<HashSet<_>>()
            .into_iter()
            .flat_map(|p| {
                vec![
                    (Pair(me.clone(), p.clone()), 0),
                    (Pair(p.clone(), me.clone()), 0),
                ]
            })
            .collect::<Vec<_>>(),
    );
    println!("part 2: {}", visit(&pairs));
}
