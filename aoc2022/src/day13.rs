use std::{
    fs::{self},
    iter::once,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u8,
    combinator::map,
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

#[derive(Clone, Debug)]
enum Packet {
    List(Vec<Packet>),
    Literal(u8),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Literal(l), Packet::Literal(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => l.cmp(r),
            (Packet::Literal(l), Packet::List(r)) => vec![Packet::Literal(*l)].cmp(r),
            (Packet::List(l), Packet::Literal(r)) => l.cmp(&vec![Packet::Literal(*r)]),
        }
    }
}

fn packet(input: &str) -> IResult<&str, Packet> {
    let list = delimited(tag("["), separated_list0(tag(","), packet), tag("]"));
    alt((map(list, Packet::List), map(u8, Packet::Literal)))(input)
}

fn main() {
    let file = fs::read_to_string("aoc2022/inputs/day13.input").unwrap();
    let mut pairs = file
        .split_terminator("\n\n")
        .map(|l| {
            let (p1, p2) = l.split_once('\n').unwrap();
            (packet(p1).unwrap().1, packet(p2).unwrap().1)
        })
        .collect::<Vec<_>>();
    let in_order = pairs
        .iter()
        .enumerate()
        .flat_map(|(idx, (p1, p2))| (p1 <= p2).then_some(idx + 1))
        .sum::<usize>();
    let div1 = Packet::List(vec![Packet::List(vec![Packet::Literal(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Literal(6)])]);
    pairs.push((div1.clone(), div2.clone()));
    let mut pairs = pairs
        .into_iter()
        .flat_map(|(p1, p2)| once(p1).chain(once(p2)))
        .collect::<Vec<_>>();
    pairs.sort_unstable();
    let idx1 = pairs.iter().position(|p| p == &div1).unwrap() + 1;
    let idx2 = pairs.iter().position(|p| p == &div2).unwrap() + 1;
    println!("part 1: {}", in_order);
    println!("part 2: {}", idx1 * idx2);
}
