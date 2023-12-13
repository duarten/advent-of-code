use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn solve(springs: &[u8], damaged: &[usize], cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(count) = cache.get(&(springs.len(), damaged.len())) {
        return *count;
    }
    let mut count = 0;
    let run = damaged[0];
    let total_damaged = damaged.iter().sum::<usize>();
    for idx in 0..=springs.len() - total_damaged {
        if idx > 0 && springs[idx - 1] == b'#' {
            break;
        }
        if springs[idx..idx + run].iter().all(|s| *s != b'.') {
            if damaged.len() == 1 {
                count += springs[idx + run..].iter().all(|s| *s != b'#') as usize;
            } else if idx + run == springs.len() {
                break;
            } else if springs[idx + run] != b'#' && springs.len() > idx + total_damaged {
                count += solve(&springs[idx + run + 1..], &damaged[1..], cache);
            }
        }
    }
    cache.insert((springs.len(), damaged.len()), count);
    count
}

fn main() {
    let file = File::open("aoc2023/inputs/day12.input").unwrap();
    let mut cache = HashMap::new();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for node in io::BufReader::new(file).lines().map(|l| l.unwrap()) {
        let (springs, damaged) = node.split_once(' ').unwrap();
        let damaged = damaged
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<usize>, _>>()
            .unwrap();
        sum1 += solve(springs.as_bytes(), &damaged, &mut cache);
        cache.clear();
        let expanded_springs = (0..5).map(|_| springs).collect::<Vec<_>>().join("?");
        sum2 += solve(expanded_springs.as_bytes(), &damaged.repeat(5), &mut cache);
        cache.clear();
    }
    println!("part 1: {}", sum1);
    println!("part 2: {}", sum2);
}
