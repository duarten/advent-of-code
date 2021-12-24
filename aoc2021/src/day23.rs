use std::{
    cmp,
    collections::{BinaryHeap, HashSet},
    fs::File,
    io::{self, BufRead},
};

use utils::abs_diff;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Room {
    amphipods: Vec<Amphipod>,
    capacity: usize,
    pos: usize,
}

impl Room {
    fn free_pos(&self) -> usize {
        self.capacity - self.amphipods.len()
    }

    fn hallway_pos(&self) -> usize {
        (self.pos + 1) * 2
    }

    fn amphipod_to_move(&self) -> Option<Amphipod> {
        if self.amphipods.iter().all(|&a| a.dest_pos() == self.pos) {
            None
        } else {
            self.amphipods.last().copied()
        }
    }

    fn is_complete(&self) -> bool {
        self.amphipods
            .iter()
            .filter(|a| a.dest_pos() == self.pos)
            .count()
            == self.capacity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Amphipod(u8);

impl Amphipod {
    fn cost(&self) -> usize {
        [1, 10, 100, 1000][(self.0 - b'A') as usize]
    }

    fn dest_pos(&self) -> usize {
        (self.0 - b'A') as usize
    }
}

fn move_cost(a: Amphipod, hallway_pos: usize, room: &Room, to_hallway: bool) -> usize {
    let mut cost = abs_diff(hallway_pos, room.hallway_pos());
    cost += room.free_pos() + (to_hallway as usize);
    cost * a.cost()
}

#[derive(Clone)]
struct State {
    rooms: Vec<Room>,
    hallway: Vec<Option<Amphipod>>,
    energy: usize,
}

impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rooms.hash(state);
        self.hallway.hash(state);
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.rooms == other.rooms && self.hallway == other.hallway
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.energy.partial_cmp(&self.energy)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.energy.cmp(&self.energy)
    }
}

impl State {
    fn is_complete(&self) -> bool {
        self.rooms.iter().all(Room::is_complete)
    }

    fn hallway_positions(&self) -> Vec<usize> {
        (0..self.hallway.len())
            .filter(|p| self.rooms.iter().all(|r| r.hallway_pos() != *p))
            .collect()
    }

    fn move_to_room(&self, amphipod: Amphipod, hallway_pos: usize, room_pos: usize) -> State {
        let mut ns = self.clone();
        ns.hallway[hallway_pos].take();
        ns.rooms[room_pos].amphipods.push(amphipod);
        ns.energy += move_cost(amphipod, hallway_pos, &self.rooms[room_pos], false);
        ns
    }

    fn move_to_hallway(&self, amphipod: Amphipod, hallway_pos: usize, room_pos: usize) -> State {
        let mut ns = self.clone();
        ns.hallway[hallway_pos].replace(amphipod);
        ns.rooms[room_pos].amphipods.pop();
        ns.energy += move_cost(amphipod, hallway_pos, &self.rooms[room_pos], true);
        ns
    }

    fn free_path(&self, hallway_pos: usize, room: &Room) -> bool {
        let rh = room.hallway_pos();
        let slice = &self.hallway[cmp::min(hallway_pos + 1, rh)..cmp::max(hallway_pos, rh)];
        slice.iter().flatten().count() == 0
    }
}

fn advance(s: State, hallway_positions: &[usize]) -> Vec<State> {
    let mut candidates = Vec::new();
    for room in s.rooms.iter() {
        if let Some(amphipod) = room.amphipod_to_move() {
            for &hallway_pos in hallway_positions.iter() {
                if s.hallway[hallway_pos].is_none() && s.free_path(hallway_pos, room) {
                    candidates.push(s.move_to_hallway(amphipod, hallway_pos, room.pos));
                }
            }
        }
    }
    for &hallway_pos in hallway_positions.iter() {
        if let Some(amphipod) = s.hallway[hallway_pos] {
            let room = &s.rooms[amphipod.dest_pos()];
            if room.amphipods.iter().all(|&a| a == amphipod) && s.free_path(hallway_pos, room) {
                candidates.push(s.move_to_room(amphipod, hallway_pos, room.pos));
            }
        }
    }
    candidates
}

fn solve(initial: State) -> usize {
    let hallway_positions = initial.hallway_positions();
    let mut min_cost = usize::MAX;
    let mut cache = HashSet::<State>::new();
    let mut to_visit = BinaryHeap::new();
    to_visit.push(initial);
    while let Some(s) = to_visit.pop() {
        if s.is_complete() {
            min_cost = cmp::min(s.energy, min_cost);
        } else {
            for c in advance(s, &hallway_positions) {
                if c.energy < cache.get(&c).map(|c| c.energy).unwrap_or(usize::MAX)
                    && c.energy < min_cost
                {
                    cache.insert(c.clone());
                    to_visit.push(c);
                }
            }
        }
    }
    min_cost
}

fn main() {
    let file = File::open("aoc2021/inputs/day23.input").unwrap();
    let mut hallway = Vec::new();
    let mut rooms = Vec::new();
    for l in io::BufReader::new(file).lines().flatten() {
        let mut room_id = 0;
        for c in l.bytes() {
            match c {
                b'.' => hallway.push(None),
                a @ (b'A' | b'B' | b'C' | b'D') => {
                    if room_id >= rooms.len() {
                        rooms.push(Room {
                            amphipods: Vec::new(),
                            capacity: 2,
                            pos: room_id,
                        });
                    }
                    rooms[room_id].amphipods.insert(0, Amphipod(a));
                    room_id += 1;
                }
                _ => {}
            }
        }
    }
    let mut initial = State {
        rooms,
        hallway,
        energy: 0,
    };
    println!("part 1: {}", solve(initial.clone()));
    let to_add = vec![(b'D', b'D'), (b'C', b'B'), (b'B', b'A'), (b'A', b'C')];
    for (idx, (a, b)) in to_add.into_iter().enumerate() {
        initial.rooms[idx].capacity = 4;
        initial.rooms[idx].amphipods.insert(1, Amphipod(a));
        initial.rooms[idx].amphipods.insert(1, Amphipod(b));
    }
    println!("part 2: {}", solve(initial));
}
