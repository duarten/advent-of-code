use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("aoc2020/inputs/day21.input").unwrap();
    let mut foods = HashMap::<String, usize>::new();
    let mut allergens = HashMap::<String, Vec<String>>::new();
    for line in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        let parts: Vec<&str> = line.split('(').collect();
        let fs: Vec<&str> = parts[0].split(' ').filter(|s| !s.is_empty()).collect();
        for f in fs.iter() {
            foods
                .entry(f.to_string())
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        if parts.len() > 1 {
            for a in parts[1][9..(parts[1].len() - 1)].split(", ") {
                allergens
                    .entry(a.to_owned())
                    .and_modify(|afs| {
                        let v = afs
                            .iter_mut()
                            .filter(|a| fs.iter().any(|x| x == a))
                            .map(|a| a.to_owned())
                            .collect::<Vec<String>>();
                        *afs = v;
                    })
                    .or_insert_with(|| fs.iter().map(|f| f.to_string()).collect());
            }
        }
    }
    let s: usize = foods
        .iter()
        .filter(|(f, _)| !allergens.iter().any(|(_, fs)| fs.contains(f)))
        .map(|(_, s)| s)
        .sum();
    println!("part 1: {}", s);
    let mut allergens: Vec<_> = allergens.into_iter().map(|(k, v)| (k, v)).collect();
    for i in 0..allergens.len() {
        allergens.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
        if allergens[i].1.len() > 1 {
            panic!("{:?}", allergens[i]);
        }
        for j in (i + 1)..allergens.len() {
            if let Some(idx) = allergens[j].1.iter().position(|f| *f == allergens[i].1[0]) {
                allergens[j].1.remove(idx);
            }
        }
    }
    allergens.sort_by(|a, b| a.0.cmp(&b.0));
    println!(
        "part 2: {}",
        allergens
            .into_iter()
            .map(|mut a| a.1.pop().unwrap())
            .collect::<Vec<_>>()
            .join(",")
    );
}
