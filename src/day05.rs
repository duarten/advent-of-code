use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;

fn find_position(mut pos: &[usize], guide: &[i32]) -> usize {
    for g in guide {
        let m = pos.len() / 2;
        if *g > 0 {
            pos = &pos[m..];
        } else {
            pos = &pos[..m];
        }
    }
    pos[0]
}

fn main() {
    let file = File::open("inputs/day05.input").unwrap();
    let lines = io::BufReader::new(file).lines().map(|l| {
        l.unwrap()
            .chars()
            .map(|c| match c {
                'F' | 'L' => -1,
                'B' | 'R' => 1,
                _ => panic!(),
            })
            .collect::<Vec<i32>>()
    });
    let rows: Vec<usize> = (0..128).collect();
    let cols: Vec<usize> = (0..8).collect();
    let positions = lines.map(|s| {
        (
            find_position(&rows, &s[0..7]),
            find_position(&cols, &s[7..]),
        )
    });
    let ids = positions.map(|(r, c)| r * 8 + c);
    let (highest, smallest, cnt) = ids.fold((0, usize::MAX, 0), |(max, min, sum), x| {
        (usize::max(max, x), usize::min(min, x), sum + x)
    });
    let missing = (smallest..=highest).fold(0, usize::add) - cnt;
    println!("highest seat: {}; missing: {}", highest, missing);
}
