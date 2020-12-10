use std::fs;
use std::io::{self, BufRead};

fn count_differences(jolts: &[u64]) -> (u64, u64) {
    let mut one = 0u64;
    let mut three = 0u64;
    for i in 0..(jolts.len() - 1) {
        let diff = jolts[i + 1] - jolts[i];
        if diff == 1 {
            one += 1
        } else if diff == 3 {
            three += 1
        }
    }
    (one, three)
}

fn count_arrangements(jolts: &[u64]) -> usize {
    let mut paths = vec![0; jolts.len()];
    paths[0] = 1;
    for i in 1..jolts.len() {
        paths[i] = jolts[..i]
            .iter()
            .enumerate()
            .rev()
            .take_while(|(_, j)| jolts[i] - **j <= 3)
            .map(|(i, _)| paths[i])
            .sum();
    }
    paths[jolts.len() - 1]
}

fn main() {
    let file = fs::File::open("inputs/day10.input").unwrap();
    let mut jolts = vec![0];
    jolts.extend(
        io::BufReader::new(file)
            .lines()
            .map(|l| l.unwrap().parse::<u64>().unwrap()),
    );
    jolts.sort_unstable();
    jolts.push(jolts.last().unwrap() + 3);
    let (one, three) = count_differences(&jolts);
    println!("diff: {} * {} = {}", one, three, one * three);
    println!("arrangements = {}", count_arrangements(&jolts));
}
