use std::{
    fs::File,
    io::{self, BufRead},
    ops::RangeInclusive,
};

fn make_range(r: &str) -> RangeInclusive<usize> {
    let (start, end) = r.split_once('-').unwrap();
    start.parse().unwrap()..=end.parse().unwrap()
}

fn subsumes<T: PartialOrd>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool {
    r1.contains(r2.start()) && r1.contains(r2.end())
}

fn overlaps<T: PartialOrd>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool {
    r1.start() <= r2.end() && r2.start() <= r1.end()
}

fn main() {
    let file = File::open("aoc2022/inputs/day04.input").unwrap();
    let mut subsumed = 0;
    let mut overlapped = 0;
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let (p1, p2) = line.split_once(',').unwrap();
        let (r1, r2) = (make_range(p1), make_range(p2));
        subsumed += (subsumes(&r1, &r2) || subsumes(&r2, &r1)) as usize;
        overlapped += overlaps(&r1, &r2) as usize;
    }
    println!("part 1: {}", subsumed);
    println!("part 2: {}", overlapped);
}
