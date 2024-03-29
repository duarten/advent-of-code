use std::{
    collections,
    fs,
    io::{self, BufRead},
};

fn is_valid(nums: &collections::HashSet<u64>, x: u64) -> Option<(u64, u64)> {
    nums.iter()
        .find_map(|y| nums.get(&x.abs_diff(*y)).map(|z| (*y, *z)))
}

fn find_weakness(nums: Vec<u64>, target: u64) -> Option<u64> {
    let mut memo = collections::HashMap::<u64, usize>::new();
    let mut sum = nums.iter().sum();
    nums.iter().enumerate().find_map(|(i, n)| {
        memo.insert(sum, i);
        sum -= n;
        if let Some(j) = memo.get(&(sum + target)) {
            let (min, max) = nums[*j..(i + 1)]
                .iter()
                .fold((u64::MAX, 0), |(min, max), x| {
                    (u64::min(min, *x), u64::max(max, *x))
                });
            Some(min + max)
        } else {
            None
        }
    })
}

fn main() {
    let file = fs::File::open("aoc2020/inputs/day09.input").unwrap();
    let nums: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    let mut iter = nums.iter().copied();
    let mut preamble: collections::VecDeque<_> = iter.by_ref().take(25).collect();
    let mut map: collections::HashSet<_> = preamble.iter().copied().collect();
    let invalid = iter
        .skip_while(|n| {
            is_valid(&map, *n)
                .map(|_| {
                    map.remove(&preamble.pop_front().unwrap());
                    preamble.push_back(*n);
                    map.insert(*n);
                })
                .is_some()
        })
        .take(1)
        .sum();
    println!("invalid: {:?}", invalid);
    println!("weakness: {:?}", find_weakness(nums, invalid).unwrap());
}
