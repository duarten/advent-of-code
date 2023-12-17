use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{self, BufRead},
};

use utils::manhattan_distance_i32;

fn main() {
    let file = File::open("aoc2023/inputs/day17.input").unwrap();
    let mut map = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, l) in io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
    {
        for (x, c) in l.chars().enumerate() {
            map.insert((x as i32, y as i32), c.to_digit(10).unwrap() as usize);
            width = x;
        }
        height = y;
    }
    println!(
        "part 1: {:?}",
        min_heat_loss(&map, (width as i32, height as i32), false),
    );
    println!(
        "part 2: {:?}",
        min_heat_loss(&map, (width as i32, height as i32), true),
    );
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct State {
    next: (i32, i32),
    dir: (i32, i32),
    straight: usize,
}

impl State {
    fn new(next: (i32, i32), dir: (i32, i32), straight: usize) -> Self {
        Self {
            next,
            dir,
            straight,
        }
    }
}

struct Path {
    state: State,
    heat_loss: usize,
    projected_cost: usize,
}

impl Path {
    fn new(state: State, heat_loss: usize, projected_cost: usize) -> Self {
        Self {
            state,
            heat_loss,
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
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.projected_cost.cmp(&self.projected_cost)
    }
}

fn min_heat_loss(map: &HashMap<(i32, i32), usize>, end: (i32, i32), is_ultra: bool) -> usize {
    let mut visited = HashMap::new();
    let mut to_visit = BinaryHeap::from([
        Path::new(State::new((0, 0), (1, 0), 1), 0, 0),
        Path::new(State::new((0, 0), (0, 1), 1), 0, 0),
    ]);
    let mut min_heat_loss = usize::MAX;
    while let Some(Path {
        state: State {
            next,
            dir,
            straight,
        },
        heat_loss,
        ..
    }) = to_visit.pop()
    {
        if next == end && (!is_ultra || straight >= 4) {
            min_heat_loss = min_heat_loss.min(heat_loss);
            continue;
        }

        for candidate in candidates(next, dir, straight, is_ultra) {
            if let Some(candidate_heat_loss) = map.get(&candidate.next) {
                let new_heat_loss = heat_loss + candidate_heat_loss;
                if new_heat_loss < *visited.get(&candidate).unwrap_or(&usize::MAX) {
                    visited.insert(candidate.clone(), new_heat_loss);
                    let projected_cost =
                        new_heat_loss + manhattan_distance_i32(candidate.next, end);
                    to_visit.push(Path::new(candidate, new_heat_loss, projected_cost));
                }
            }
        }
    }
    min_heat_loss
}

fn candidates(pos: (i32, i32), dir: (i32, i32), straight: usize, is_ultra: bool) -> Vec<State> {
    let mut candidates = Vec::new();
    if straight < if is_ultra { 10 } else { 3 } {
        candidates.push(State::new(
            (pos.0 + dir.0, pos.1 + dir.1),
            dir,
            straight + 1,
        ));
    }
    if !is_ultra || straight >= 4 {
        if dir.0 != 0 {
            candidates.push(State::new((pos.0, pos.1 + 1), (0, 1), 1));
            candidates.push(State::new((pos.0, pos.1 - 1), (0, -1), 1));
        } else {
            candidates.push(State::new((pos.0 + 1, pos.1), (1, 0), 1));
            candidates.push(State::new((pos.0 - 1, pos.1), (-1, 0), 1));
        }
    }
    candidates
}
