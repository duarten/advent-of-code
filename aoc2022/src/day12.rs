use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{self, BufRead},
    ops::Index,
};

type Offset = (usize, usize);

struct Heights(Vec<Vec<usize>>, Offset);

impl Index<Offset> for Heights {
    type Output = usize;

    fn index(&self, index: Offset) -> &Self::Output {
        &self.0[index.1][index.0]
    }
}

impl Heights {
    fn at(&self, index: Offset) -> Option<usize> {
        (index.0 < self.0[0].len() && index.1 < self.0.len()).then(|| self[index])
    }

    fn target(&self) -> Offset {
        self.1
    }

    fn heuristic(&self, index: Offset) -> usize {
        self.target().0.abs_diff(index.0) + self.target().1.abs_diff(index.1)
    }
}

struct Path {
    offset: Offset,
    cost: usize,
    projected_cost: usize,
}

impl Path {
    fn new(offset: Offset, cost: usize, projected_cost: usize) -> Self {
        Self {
            offset,
            cost,
            projected_cost,
        }
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.projected_cost == other.projected_cost
    }
}

impl Eq for Path {}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.projected_cost.partial_cmp(&self.projected_cost)
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.projected_cost.cmp(&self.projected_cost)
    }
}

fn min_distance(heights: &Heights, starts: Vec<Offset>) -> usize {
    let mut to_visit = BinaryHeap::new();
    to_visit.extend(
        starts
            .into_iter()
            .map(|start| Path::new(start, 0, heights.heuristic(start))),
    );
    let mut final_cost = usize::MAX;
    let mut cache = HashMap::<Offset, usize>::new();
    while let Some(Path { offset, cost, .. }) = to_visit.pop() {
        if offset == heights.target() {
            final_cost = std::cmp::min(cost, final_cost);
            continue;
        }
        let current_height = heights.at(offset).unwrap();
        for new_offset in
            [(0, 1), (1, 0), (-1, 0), (0, -1)]
                .into_iter()
                .filter_map(|step: (i32, i32)| {
                    let new_offset = (
                        offset.0.wrapping_add(step.0 as usize),
                        offset.1.wrapping_add(step.1 as usize),
                    );
                    let new_height = heights.at(new_offset)?;
                    (new_height <= current_height + 1).then_some(new_offset)
                })
        {
            let new_cost = cost + 1;
            if new_cost < *cache.get(&new_offset).unwrap_or(&usize::MAX) {
                cache.insert(new_offset, new_cost);
                to_visit.push(Path::new(
                    new_offset,
                    new_cost,
                    new_cost + heights.heuristic(new_offset),
                ));
            }
        }
    }
    final_cost
}

fn main() {
    let file = File::open("aoc2022/inputs/day12.input").unwrap();
    let mut heights = Vec::new();
    let mut start = (0, 0);
    let mut other_starts = Vec::new();
    let mut end = (0, 0);
    for (y, l) in io::BufReader::new(file).lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in l.unwrap().bytes().enumerate() {
            if c == b'S' {
                start = (x, y);
                row.push(0usize);
            } else if c == b'E' {
                end = (x, y);
                row.push((b'z' - b'a') as usize);
            } else {
                row.push((c - b'a') as usize);
                if c == b'a' {
                    other_starts.push((x, y));
                }
            }
        }
        heights.push(row);
    }
    let heights = Heights(heights, end);
    println!("part 1: {}", min_distance(&heights, vec![start]));
    println!("part 2: {}", min_distance(&heights, other_starts));
}
