use std::fs::File;
use std::io::{self, BufRead};

fn shape_score(shape: &str, base: char) -> usize {
    (shape.as_bytes().first().unwrap() - base as u8 + 1) as usize
}

fn main() {
    let file = File::open("aoc2022/inputs/day02.input").unwrap();
    let mut total_score_pt1 = 0;
    let mut total_score_pt2 = 0;
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let shapes = line.split_once(' ').unwrap();
        total_score_pt1 += match shapes {
            ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
            ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
            _ => 0,
        } + shape_score(shapes.1, 'X');
        total_score_pt2 += match shapes {
            (shape, "X") => match shape_score(shape, 'A') {
                1 => 3,
                x => x - 1,
            },
            (shape, "Y") => shape_score(shape, 'A'),
            (shape, "Z") => match shape_score(shape, 'A') {
                3 => 1,
                x => x + 1,
            },
            _ => 0,
        } + (shapes.1.as_bytes().first().unwrap() - b'X') as usize * 3;
    }
    println!("part 1: {:?}", total_score_pt1);
    println!("part 2: {:?}", total_score_pt2);
}
