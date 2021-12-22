use std::fs::{self};

fn decompress(data: &str) -> (usize, usize) {
    let mut cntv1 = 0;
    let mut cntv2 = 0;
    let mut idx = 0;
    let bytes = data.as_bytes();
    while idx < data.len() {
        match bytes[idx] {
            b'(' => {
                let end = bytes[idx..].iter().position(|&c| c == b')').unwrap();
                let (n, r) = data[(idx + 1)..(idx + end)].split_once('x').unwrap();
                let n = n.parse::<usize>().unwrap();
                let r = r.parse::<usize>().unwrap();
                cntv1 += r * n;
                cntv2 += r * decompress(&data[(idx + end + 1)..(idx + end + 1 + n)]).1;
                idx = idx + end + 1 + n;
            }
            _ => {
                idx += 1;
                cntv1 += 1;
                cntv2 += 1;
            }
        }
    }
    (cntv1, cntv2)
}

fn main() {
    let file = fs::read_to_string("aoc2016/inputs/day09.input").unwrap();
    let (cntv1, cntv2) = decompress(&file);
    println!("part 1: {}", cntv1);
    println!("part 2: {}", cntv2);
}
