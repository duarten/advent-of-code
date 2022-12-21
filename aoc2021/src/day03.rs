use std::{
    cmp::Ordering,
    fs::File,
    io::{self, BufRead},
};

use utils::bitvec::BitVec;

const DIGITS: usize = 12;
const MASK: usize = (1 << DIGITS) - 1;

fn ones(pos: usize, values: &[Vec<u8>]) -> Ordering {
    let ones = values.iter().map(|v| v[pos] as usize).sum::<usize>();
    ones.cmp(&(values.len() / 2))
}

fn gamma(values: &[Vec<u8>]) -> usize {
    (0..DIGITS)
        .map(|idx| match ones(idx, values) {
            Ordering::Less => 0_u8,
            _ => 1_u8,
        })
        .to_number()
}

fn filter_criteria(mut values: Vec<Vec<u8>>, criteria: impl Fn(usize, &[Vec<u8>]) -> u8) -> usize {
    let mut idx = 0;
    while values.len() > 1 {
        let c = criteria(idx, &values);
        values.retain(|v| v[idx] == c);
        idx += 1;
    }
    values.swap_remove(0).into_iter().to_number()
}

fn main() {
    let file = File::open("aoc2021/inputs/day03.input").unwrap();
    let xs = io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    let gamma = gamma(&xs);
    let epsilon = !gamma & MASK;
    println!("part 1: {:?}", gamma * epsilon);

    let o = filter_criteria(xs.clone(), |idx, iter| match ones(idx, iter) {
        Ordering::Less => 0,
        _ => 1,
    });
    let co2 = filter_criteria(xs, |idx, iter| match ones(idx, iter) {
        Ordering::Less => 1,
        _ => 0,
    });
    println!("part 2: {:?}", o * co2);
}
