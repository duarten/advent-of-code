use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

type Offset = (isize, isize);

fn adjust(head: Offset, mut tail: Offset) -> Offset {
    let (diff_x, diff_y) = (head.0 - tail.0, head.1 - tail.1);
    if diff_x.abs() < 2 && diff_y.abs() < 2 {
        return tail;
    }
    if diff_x > 0 {
        tail.0 += (diff_x + 1) / 2;
    } else {
        tail.0 += (diff_x - 1) / 2;
    }
    if diff_y > 0 {
        tail.1 += (diff_y + 1) / 2;
    } else {
        tail.1 += (diff_y - 1) / 2;
    }
    tail
}

fn main() {
    let file = File::open("aoc2022/inputs/day09.input").unwrap();
    let mut visited_1 = HashSet::new();
    let mut visited_2 = HashSet::new();
    let mut knots = [(isize::MAX / 2, isize::MAX / 2); 10];
    visited_1.insert(knots[1]);
    visited_2.insert(knots[knots.len() - 1]);
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let split = line.split(' ').collect::<Vec<_>>();
        for _ in 0..split[1].parse::<usize>().unwrap() {
            match split[0] {
                "R" => {
                    knots[0].0 += 1;
                }
                "L" => {
                    knots[0].0 -= 1;
                }
                "U" => {
                    knots[0].1 += 1;
                }
                "D" => {
                    knots[0].1 -= 1;
                }
                _ => unreachable!(),
            }
            for i in 1..knots.len() {
                knots[i] = adjust(knots[i - 1], knots[i]);
            }
            visited_1.insert(knots[1]);
            visited_2.insert(knots[knots.len() - 1]);
        }
    }
    println!("part 1: {}", visited_1.len());
    println!("part 2: {}", visited_2.len());
}
