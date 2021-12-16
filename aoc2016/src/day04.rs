use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("aoc2016/inputs/day04.input").unwrap();
    let mut sum = 0;
    let mut np_id = 0;
    for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        let parts = line.split('-').collect::<Vec<_>>();
        let last = parts[parts.len() - 1];
        let id = last[0..3].parse::<usize>().unwrap();
        let mut decoded = Vec::new();
        let mut freq = HashMap::new();
        for p in &parts[0..(parts.len() - 1)] {
            for c in p.bytes() {
                decoded.push(b'a' + (((c - b'a') as usize + id) % 26) as u8);
                *freq.entry(c).or_insert(0) += 1;
            }
        }
        let mut freq = freq.into_iter().collect::<Vec<_>>();
        freq.sort_unstable_by(|a, b| a.1.cmp(&b.1).reverse().then(a.0.cmp(&b.0)));
        if freq
            .into_iter()
            .map(|(c, _)| c)
            .take(5)
            .eq(last[4..9].bytes())
        {
            sum += id;
            if std::str::from_utf8(&decoded).unwrap() == "northpoleobjectstorage" {
                np_id = id
            }
        }
    }
    println!("part 1: {}", sum);
    println!("part 2: {}", np_id);
}
