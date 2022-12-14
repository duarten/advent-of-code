use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

use itertools::{Either, Itertools};
use nom::{
    bytes::complete::tag, character::complete::u32, combinator::map, multi::separated_list0,
    sequence::tuple, IResult,
};

type Offset = (u32, u32);

fn line(input: &str) -> IResult<&str, Vec<Offset>> {
    let offset = map(tuple((u32, tag(","), u32)), |(x, _, y)| (x, y));
    separated_list0(tag(" -> "), offset)(input)
}

fn build_range(from: u32, to: u32) -> impl Iterator<Item = u32> {
    if from < to {
        Either::Left(from..=to)
    } else {
        Either::Right((to..=from).rev())
    }
}

fn fill(from: Offset, to: Offset) -> impl Iterator<Item = Offset> {
    if from.0 == to.0 {
        Either::Left(build_range(from.1, to.1).map(move |y| (from.0, y)))
    } else {
        Either::Right(build_range(from.0, to.0).map(move |x| (x, from.1)))
    }
}

fn calculate_next(
    terrain: &HashSet<Offset>,
    sand: &mut HashSet<Offset>,
    source: Offset,
    offset: Offset,
) -> Offset {
    let down = (offset.0, offset.1 + 1);
    let left = (offset.0 - 1, offset.1 + 1);
    let right = (offset.0 + 1, offset.1 + 1);
    if let Some(next) = [down, left, right]
        .into_iter()
        .find(|&next| !(terrain.contains(&next) || sand.contains(&next)))
    {
        next
    } else {
        sand.insert(offset);
        source
    }
}

fn main() {
    let file = File::open("aoc2022/inputs/day14.input").unwrap();
    let terrain = io::BufReader::new(file)
        .lines()
        .flat_map(|l| {
            line(&l.unwrap())
                .unwrap()
                .1
                .into_iter()
                .tuple_windows::<(_, _)>()
                .flat_map(|(from, to)| fill(from, to))
        })
        .collect::<HashSet<_>>();
    let max_y = terrain.iter().map(|(_, y)| y).max().unwrap();
    let mut sand = HashSet::new();
    let source = (500, 0);
    let mut offset = source;
    while offset.1 < *max_y {
        offset = calculate_next(&terrain, &mut sand, source, offset);
    }
    println!("part 1: {}", sand.len());
    offset = source;
    while !sand.contains(&source) {
        if offset.1 == *max_y + 1 {
            sand.insert(offset);
            offset = source;
        } else {
            offset = calculate_next(&terrain, &mut sand, source, offset);
        }
    }
    println!("part 2: {}", sand.len());
}
