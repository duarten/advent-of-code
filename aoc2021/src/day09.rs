use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    iter::repeat,
    ops::{Index, Mul},
};

struct Heights(Vec<Vec<usize>>);

type Offset = (usize, usize);

impl Index<Offset> for Heights {
    type Output = usize;

    fn index(&self, index: Offset) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl Heights {
    fn offsets(&self) -> Vec<Offset> {
        (0..self.0.len())
            .flat_map(|row| repeat(row).zip(0..self.0[0].len()))
            .collect()
    }

    fn at(&self, index: Offset) -> Option<usize> {
        (index.0 < self.0.len() && index.1 < self.0[0].len()).then(|| self[index])
    }
}

fn neighbors(heights: &Heights, offset: Offset) -> Vec<Offset> {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .filter_map(|step: (i32, i32)| {
            let new_offset = (
                offset.0.wrapping_add(step.0 as usize),
                offset.1.wrapping_add(step.1 as usize),
            );
            match heights.at(new_offset) {
                Some(h) if h < 9 => Some(new_offset),
                _ => None,
            }
        })
        .collect()
}

fn low_offsets(heights: &Heights) -> Vec<Offset> {
    let mut os = heights.offsets();
    os.retain(|&offset| {
        neighbors(heights, offset)
            .into_iter()
            .map(|no| heights[no])
            .min()
            .unwrap_or(0)
            > heights[offset]
    });
    os
}

fn count_basin_locations(heights: &Heights, pos: Offset, visited: &mut HashSet<Offset>) -> usize {
    visited.insert(pos);
    1 + neighbors(heights, pos)
        .into_iter()
        .filter_map(|no| {
            (!visited.contains(&no)).then(|| count_basin_locations(heights, no, visited))
        })
        .sum::<usize>()
}

fn basins(heights: &Heights, low_offsets: Vec<Offset>) -> usize {
    let mut bs = low_offsets
        .into_iter()
        .map(|o| count_basin_locations(heights, o, &mut HashSet::default()))
        .collect::<Vec<_>>();
    bs.sort_unstable();
    bs[bs.len() - 3..].iter().cloned().reduce(Mul::mul).unwrap()
}

fn main() {
    let file = File::open("aoc2021/inputs/day09.input").unwrap();
    let input = io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let heights = Heights(input);
    let low_offsets = low_offsets(&heights);
    let low_points: usize = low_offsets.iter().map(|&pos| heights[pos] + 1).sum();
    println!("part 1: {}", low_points);
    println!("part 2: {}", basins(&heights, low_offsets));
}
