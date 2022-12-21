use std::fs::{self};

use utils::bitvec::{char_to_bits, BitVec};

fn parse_literal(literal: &mut impl Iterator<Item = u8>) -> usize {
    let mut decoded = Vec::<u8>::new();
    while let Some(1) = literal.next() {
        decoded.extend(literal.take(4));
    }
    decoded.extend(literal.take(4));
    decoded.into_iter().to_number()
}

fn parse(
    packet: &mut impl Iterator<Item = u8>,
    rem: Option<usize>,
    versions: &mut Vec<usize>,
    values: &mut Vec<usize>,
) {
    let version = packet.take(3).to_number();
    versions.push(version);
    let type_id = packet.take(3).to_number();
    if type_id == 4 {
        values.push(parse_literal(packet));
    } else {
        let mut inner = Vec::with_capacity(2);
        match packet.next() {
            Some(0) => {
                let length = packet.take(15).to_number();
                let nested = packet.take(length).collect::<Vec<_>>();
                let mut iter = nested.iter().cloned();
                parse(iter.by_ref(), None, versions, &mut inner);
            }
            Some(1) => {
                let length = packet.take(11).to_number();
                parse(packet, Some(length), versions, &mut inner);
            }
            None => return,
            _ => unreachable!(),
        }
        values.push(match type_id {
            0 => inner.into_iter().sum(),
            1 => inner.into_iter().product(),
            2 => inner.into_iter().min().unwrap(),
            3 => inner.into_iter().max().unwrap(),
            5 => (inner[0] > inner[1]) as usize,
            6 => (inner[0] < inner[1]) as usize,
            7 => (inner[0] == inner[1]) as usize,
            _ => unreachable!(),
        })
    }
    match rem {
        None => parse(packet, None, versions, values),
        Some(x) if x > 1 => parse(packet, Some(x - 1), versions, values),
        _ => {}
    };
}

fn main() {
    let input = fs::read_to_string("aoc2021/inputs/day16.input").unwrap();
    let mut versions = Vec::new();
    let mut values = Vec::new();
    let mut iter = input.chars().flat_map(char_to_bits);
    parse(iter.by_ref(), None, &mut versions, &mut values);
    println!("part 1: {}", versions.into_iter().sum::<usize>());
    println!("part 2: {}", values[0]);
}
