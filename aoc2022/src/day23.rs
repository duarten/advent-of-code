use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    io::{self, BufRead},
};

type Offset = (isize, isize);

fn adjacent((x, y): Offset) -> impl Iterator<Item = Offset> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .map(|(step_x, step_y)| (x + step_x, y + step_y))
    .into_iter()
}

fn simulate(mut elves: HashSet<Offset>, max_rounds: Option<usize>) -> (usize, usize) {
    let mut directions = [
        [(0, -1), (1, -1), (-1, -1)],
        [(0, 1), (1, 1), (-1, 1)],
        [(-1, 0), (-1, 1), (-1, -1)],
        [(1, 0), (1, 1), (1, -1)],
    ]
    .into_iter()
    .collect::<VecDeque<_>>();
    let mut rounds = 0;
    while max_rounds.is_none() || rounds < max_rounds.unwrap() {
        rounds += 1;
        let mut moves = HashMap::<Offset, Vec<Offset>>::new();
        for o in &elves {
            if adjacent(*o).all(|a| !elves.contains(&a)) {
                continue;
            }
            for ds in &directions {
                if ds.iter().all(|d| !elves.contains(&(o.0 + d.0, o.1 + d.1))) {
                    moves
                        .entry((o.0 + ds[0].0, o.1 + ds[0].1))
                        .or_default()
                        .push(*o);
                    break;
                }
            }
        }
        if moves.is_empty() {
            break;
        }
        for (move_to, current) in moves.into_iter().filter(|(_, v)| v.len() == 1) {
            elves.remove(&current[0]);
            elves.insert(move_to);
        }
        let dirs = directions.pop_front().unwrap();
        directions.push_back(dirs);
    }
    let max_x = elves.iter().map(|(x, _)| x).max().unwrap();
    let min_x = elves.iter().map(|(x, _)| x).min().unwrap();
    let max_y = elves.iter().map(|(_, y)| y).max().unwrap();
    let min_y = elves.iter().map(|(_, y)| y).min().unwrap();
    let bounding_box = ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize - elves.len();
    (bounding_box, rounds)
}

fn main() {
    let file = fs::File::open("aoc2022/inputs/day23.input").unwrap();
    let mut elves = HashSet::new();
    for (y, l) in io::BufReader::new(file).lines().enumerate() {
        for (x, c) in l.unwrap().chars().enumerate() {
            if c == '#' {
                elves.insert((x as isize, y as isize));
            }
        }
    }
    println!("part 1: {}", simulate(elves.clone(), Some(10)).0);
    println!("part 2: {}", simulate(elves.clone(), None).1);
}
