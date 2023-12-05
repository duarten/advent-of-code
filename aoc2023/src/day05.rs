use std::{
    fs::{self},
    ops::Range,
};

fn solve(mut values: Vec<Range<usize>>, maps: &[Vec<(Range<usize>, Range<usize>)>]) -> usize {
    for map in maps {
        for value in std::mem::take(&mut values) {
            let start_len = values.len();
            for (src, dst) in map.iter().take_while(|(src, _)| src.start < value.end) {
                if value.start < src.end && src.start < value.end {
                    let overlapping_range = value.start.max(src.start)..value.end.min(src.end);
                    let mapped_range_start = dst.start + (overlapping_range.start - src.start);
                    let mapped_range_end =
                        mapped_range_start + (overlapping_range.end - overlapping_range.start);
                    values.push(mapped_range_start..mapped_range_end);
                }
            }
            if start_len == values.len() {
                values.push(value);
            }
        }
    }
    values.into_iter().min_by_key(|x| x.start).unwrap().start
}

fn main() {
    let input = fs::read_to_string("aoc2023/inputs/day05.input").unwrap();
    let mut sections = input.split("\n\n");
    let (_, seeds) = sections.next().unwrap().split_once(": ").unwrap();
    let seeds = seeds
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let all_seeds = seeds
        .chunks(2)
        .map(|x| (x[0]..(x[0] + x[1])))
        .collect::<Vec<_>>();
    let maps = sections
        .map(|m| {
            let mut interval_map = m
                .lines()
                .skip(1)
                .map(|ranges| {
                    let mut range_info = ranges.split_ascii_whitespace();
                    let dst_start = range_info.next().unwrap().parse::<usize>().unwrap();
                    let src_start = range_info.next().unwrap().parse::<usize>().unwrap();
                    let length = range_info.next().unwrap().parse::<usize>().unwrap();
                    (src_start..src_start + length, dst_start..dst_start + length)
                })
                .collect::<Vec<_>>();
            interval_map.sort_unstable_by_key(|x| x.0.start);
            interval_map
        })
        .collect::<Vec<_>>();
    println!(
        "part 1: {}",
        solve(seeds.into_iter().map(|x| x..x + 1).collect(), &maps)
    );
    println!("part 2: {}", solve(all_seeds, &maps));
}
