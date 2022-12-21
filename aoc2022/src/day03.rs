use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn priorities(c: &str) -> HashSet<u8> {
    c.bytes()
        .map(|c| {
            if c >= b'a' {
                c - b'a' + 1
            } else {
                c - b'A' + 27
            }
        })
        .collect::<HashSet<_>>()
}

fn main() {
    let file = File::open("aoc2022/inputs/day03.input").unwrap();
    let mut sum = 0;
    let mut badge_opt: Option<HashSet<u8>> = None;
    let mut badge_sum = 0;
    for (idx, line) in io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .enumerate()
    {
        let ps = priorities(&line);
        let (c1, c2) = line.split_at(line.len() / 2);
        let (p1, p2) = (priorities(c1), priorities(c2));
        sum += *p1.intersection(&p2).last().unwrap() as usize;
        if let Some(badge) = badge_opt {
            let int = badge.intersection(&ps).cloned().collect::<HashSet<_>>();
            if (idx + 1) % 3 == 0 {
                badge_sum += int.into_iter().last().unwrap() as usize;
                badge_opt = None;
            } else {
                badge_opt = Some(int);
            }
        } else {
            badge_opt = Some(ps);
        }
    }
    println!("part 1: {:?}", sum);
    println!("part 2: {:?}", badge_sum);
}
