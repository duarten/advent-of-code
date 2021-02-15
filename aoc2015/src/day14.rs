use anyhow::Result;

use itertools::iterate;

use regex::Regex;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Eq, PartialEq)]
struct Reindeer {
    speed: u32,
    sustained: u32,
    rest: u32,
}

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r#"^(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds."#).unwrap();
}

impl FromStr for Reindeer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let g = RE.captures(s).unwrap();
        Ok(Reindeer {
            speed: g.get(2).unwrap().as_str().parse()?,
            sustained: g.get(3).unwrap().as_str().parse()?,
            rest: g.get(4).unwrap().as_str().parse()?,
        })
    }
}

#[derive(Eq, PartialEq)]
struct ReindeerState<'a> {
    reindeer: &'a Reindeer,
    counter: u32,
    resting: bool,
    distance: u32,
    points: u32,
}

impl<'a> Ord for ReindeerState<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl<'a> PartialOrd for ReindeerState<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> ReindeerState<'a> {
    fn new(r: &'a Reindeer) -> Self {
        Self {
            reindeer: r,
            counter: 0,
            resting: false,
            distance: 0,
            points: 0,
        }
    }

    fn tick(&self) -> Self {
        let mut ret = Self {
            counter: self.counter + 1,
            ..*self
        };
        if ret.resting {
            if ret.reindeer.rest == ret.counter {
                ret.counter = 0;
                ret.resting = false;
            }
        } else {
            ret.distance += ret.reindeer.speed;
            if ret.reindeer.sustained == ret.counter {
                ret.counter = 0;
                ret.resting = true;
            }
        }
        ret
    }
}

fn tick_all<'a>(rs: &[ReindeerState<'a>]) -> Vec<ReindeerState<'a>> {
    let mut heap = BinaryHeap::new();
    for r in rs {
        heap.push(r.tick())
    }
    let max = heap.peek().map(|r| r.distance).unwrap_or(0);
    let mut res: Vec<_> = heap.into_iter().collect();
    for r in res.iter_mut() {
        if r.distance == max {
            r.points += 1;
        }
    }
    res
}

fn main() {
    let file = File::open("aoc2015/inputs/day14.input").unwrap();
    let rs: Vec<Reindeer> = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    println!(
        "part 1: {:?}",
        rs.iter()
            .map(|r| iterate(ReindeerState::new(r), ReindeerState::tick)
                .take(2503)
                .last()
                .map(|r| r.distance))
            .max()
    );
    println!(
        "part 2: {:?}",
        iterate(
            rs.iter().map(|r| ReindeerState::new(r)).collect::<Vec<_>>(),
            |rs| tick_all(&rs),
        )
        .take(2503)
        .last()
        .map(|rs| rs.into_iter().map(|r| r.points).max())
    );
}
