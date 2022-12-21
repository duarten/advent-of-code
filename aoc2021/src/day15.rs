use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{self, BufRead},
    ops::Index,
};

type Offset = (usize, usize);

struct Risks(Vec<Vec<usize>>, Offset);

impl Index<Offset> for Risks {
    type Output = usize;

    fn index(&self, index: Offset) -> &Self::Output {
        &self.0[index.1][index.0]
    }
}

impl Risks {
    fn at(&self, index: Offset) -> Option<usize> {
        if index.0 > self.end().0 || index.1 > self.end().1 {
            return None;
        }
        let len = self.0.len();
        let adjusted = (index.0 % len, index.1 % len);
        let increase = index.0 / len + index.1 / len;
        Some((self[adjusted] + increase - 1) % 9 + 1)
    }

    fn end(&self) -> Offset {
        self.1
    }

    fn heuristic(&self, index: Offset) -> usize {
        (self.end().0 - index.0) + (self.end().1 - index.1)
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

impl Eq for Path {
    fn assert_receiver_is_total_eq(&self) {}
}

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

fn min_distance(risks: &Risks) -> usize {
    let mut to_visit = BinaryHeap::new();
    to_visit.push(Path::new((0, 0), 0, 0));
    let mut final_cost = usize::MAX;
    let mut cache = HashMap::<Offset, usize>::new();
    while let Some(Path { offset, cost, .. }) = to_visit.pop() {
        if offset == risks.end() {
            final_cost = std::cmp::min(cost, final_cost);
            continue;
        }
        for (c, o) in
            [(0, 1), (1, 0), (-1, 0), (0, -1)]
                .into_iter()
                .filter_map(|step: (i32, i32)| {
                    let new_offset = (
                        offset.0.wrapping_add(step.0 as usize),
                        offset.1.wrapping_add(step.1 as usize),
                    );
                    risks.at(new_offset).map(|c| (c, new_offset))
                })
        {
            let ncost = cost + c;
            if ncost < *cache.get(&o).unwrap_or(&usize::MAX) {
                cache.insert(o, ncost);
                to_visit.push(Path::new(o, ncost, ncost + risks.heuristic(o)));
            }
        }
    }
    final_cost
}

fn main() {
    let file = File::open("aoc2021/inputs/day15.input").unwrap();
    let input = io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let len = input.len();
    let risks = Risks(input, (len - 1, len - 1));
    println!("part 1: {}", min_distance(&risks));
    let risks = Risks(risks.0, (len * 5 - 1, len * 5 - 1));
    println!("part 2: {}", min_distance(&risks));
}
