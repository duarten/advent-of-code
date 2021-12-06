use std::{
    fs::File,
    io::{self, BufRead},
};

fn run_simulation(mut fishes: [usize; 9], limit: usize) -> usize {
    for _ in 0..limit {
        for (pos, cnt) in std::mem::take(&mut fishes).into_iter().enumerate() {
            if pos == 0 {
                fishes[6] += cnt;
                fishes[8] += cnt;
            } else {
                fishes[pos - 1] += cnt;
            }
        }
    }
    fishes.into_iter().sum()
}

fn main() {
    let file = File::open("aoc2021/inputs/day06.input").unwrap();
    let mut fishes = [0; 9];
    for f in io::BufReader::new(file)
        .lines()
        .last()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|f| f.parse::<usize>().unwrap())
    {
        fishes[f] += 1;
    }
    println!("part 1: {}", run_simulation(fishes, 80));
    println!("part 2: {}", run_simulation(fishes, 256));
}
