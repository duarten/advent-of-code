use std::collections;
use std::fs;

#[derive(Default)]
struct TurnsSeens {
    positions: [usize; 2],
    i: usize,
}

fn next(prev: usize, seen: &mut collections::HashMap<usize, TurnsSeens>) -> usize {
    let ts = seen.entry(prev).or_insert_with(TurnsSeens::default);
    let n = ts.positions.len();
    if ts.i > 1 {
        ts.positions[(ts.i + 1) % n] - ts.positions[ts.i % n]
    } else {
        0
    }
}

fn record(x: usize, idx: usize, seen: &mut collections::HashMap<usize, TurnsSeens>) {
    let ts = seen.entry(x).or_insert_with(TurnsSeens::default);
    ts.positions[ts.i % ts.positions.len()] = idx;
    ts.i += 1;
}

fn main() {
    let input: Vec<_> = fs::read_to_string("aoc2020/inputs/day15.input")
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let run = |limit| {
        let mut seen = collections::HashMap::<usize, TurnsSeens>::new();
        input
            .iter()
            .enumerate()
            .for_each(|(i, n)| record(*n, i, &mut seen));
        let mut prev = input[input.len() - 1];
        for i in input.len()..limit {
            let next = next(prev, &mut seen);
            record(next, i, &mut seen);
            prev = next;
        }
        prev
    };
    println!("part 1: {}", run(2020));
    println!("part 2: {}", run(30000000));
}
