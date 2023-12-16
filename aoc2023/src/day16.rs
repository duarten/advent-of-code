use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("aoc2023/inputs/day16.input").unwrap();
    let mut layout = HashMap::new();
    let mut width = 0;
    let mut height = 0;
    for (y, l) in io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
    {
        for (x, c) in l.chars().enumerate() {
            layout.insert((x, y), c);
            width = x as i32;
        }
        height = y as i32;
    }
    println!("part 1: {}", energized(((0, 0), (1, 0)), &layout));
    let mut max_energize = 0;
    let edges = (0..width)
        .flat_map(|x| vec![((x, 0), (0, 1)), ((x, height), (0, -1))])
        .chain((0..height).flat_map(|y| vec![((0, y), (1, 0)), ((width, y), (-1, 0))]));
    for edge in edges {
        max_energize = max_energize.max(energized(edge, &layout));
    }
    println!("part 2: {}", max_energize);
}

fn energized(initial: ((i32, i32), (i32, i32)), layout: &HashMap<(usize, usize), char>) -> usize {
    let mut seen = HashSet::new();
    let mut to_visit = VecDeque::from(vec![initial]);
    while let Some(state @ ((x, y), (dx, dy))) = to_visit.pop_front() {
        let Some(tile) = layout.get(&(x as usize, y as usize)) else {
            continue;
        };
        if !seen.insert(state) {
            continue;
        }
        let new_dirs = match tile {
            '.' => vec![(dx, dy)],
            '/' => vec![(-dy, -dx)],
            '\\' => vec![(dy, dx)],
            '-' if dy == 0 => vec![(dx, dy)],
            '-' => vec![(1, 0), (-1, 0)],
            '|' if dx == 0 => vec![(dx, dy)],
            '|' => vec![(0, 1), (0, -1)],
            _ => continue,
        };
        to_visit.extend(
            new_dirs
                .into_iter()
                .map(|(dx, dy)| ((x + dx, y + dy), (dx, dy))),
        );
    }
    seen.into_iter()
        .map(|(pos, _)| pos)
        .collect::<HashSet<_>>()
        .len()
}
