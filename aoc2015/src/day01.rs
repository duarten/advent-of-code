use std::fs;

fn main() {
    let (r1, r2) = fs::read_to_string("aoc2015/inputs/day01.input")
        .unwrap()
        .chars()
        .enumerate()
        .fold((0, usize::MAX), |(acc, b), (i, c)| match c {
            '(' => (acc + 1, b),
            ')' if acc == 0 => (acc - 1, std::cmp::min(b, i + 1)),
            ')' => (acc - 1, b),
            _ => panic!(),
        });
    println!("part 1: {}", r1);
    println!("part 2: {}", r2);
}
