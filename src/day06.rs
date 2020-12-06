use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Add;

fn main() {
    let file = File::open("inputs/day06.input").unwrap();
    let mut group_any = 0u32;
    let mut groups_any = vec![];
    let mut group_all = !0u32;
    let mut groups_all = vec![];
    for line in io::BufReader::new(file).lines() {
        let bytes = line.unwrap().into_bytes();
        if bytes.is_empty() {
            groups_any.push(std::mem::replace(&mut group_any, 0).count_ones());
            groups_all.push(std::mem::replace(&mut group_all, !0).count_ones());
            continue;
        }
        let p = bytes.into_iter().fold(0, |acc, x| acc | 1 << (x - b'a'));
        group_any |= p;
        group_all &= p;
    }
    groups_any.push(group_any.count_ones());
    groups_all.push(group_all.count_ones());
    println!(
        "any: {}, all: {}",
        groups_any.into_iter().fold(0, u32::add),
        groups_all.into_iter().fold(0, u32::add)
    );
}
