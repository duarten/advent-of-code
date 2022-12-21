use std::{collections::HashSet, fs};

#[derive(Clone, Default, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

fn main() {
    let mut pt1 = HashSet::<Coord>::new();
    let mut pt2 = HashSet::<Coord>::new();
    let mut all = Coord::default();
    let mut santa = Coord::default();
    let mut robosanta = Coord::default();
    pt1.insert(all.clone());
    pt2.insert(all.clone());
    for (i, c) in fs::read_to_string("aoc2015/inputs/day03.input")
        .unwrap()
        .chars()
        .enumerate()
    {
        let o = if i % 2 == 0 {
            &mut robosanta
        } else {
            &mut santa
        };
        for coord in [&mut all, o].iter_mut() {
            match c {
                '>' => coord.x += 1,
                '<' => coord.x -= 1,
                '^' => coord.y += 1,
                'v' => coord.y -= 1,
                _ => panic!(),
            }
        }
        pt1.insert(all.clone());
        pt2.insert(o.clone());
    }
    println!("part 1: {}", pt1.len());
    println!("part 2: {}", pt2.len());
}
