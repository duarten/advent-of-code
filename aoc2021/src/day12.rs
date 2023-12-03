use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

struct Path<'a> {
    next: &'a str,
    visited: HashSet<&'a str>,
    visited_small_twice: bool,
}

impl<'a> Path<'a> {
    fn new(next: &'a str, visited: HashSet<&'a str>, visited_small_twice: bool) -> Self {
        Self {
            next,
            visited,
            visited_small_twice,
        }
    }
}

fn main() {
    let file = File::open("aoc2021/inputs/day12.input").unwrap();
    let mut connections = HashMap::<String, Vec<String>>::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let (from, to) = line.split_once('-').unwrap();
        connections
            .entry(from.to_owned())
            .or_default()
            .push(to.to_owned());
        connections
            .entry(to.to_owned())
            .or_default()
            .push(from.to_owned());
    }
    let mut to_visit = vec![Path::new("start", HashSet::new(), false)];
    let mut paths = 0;
    let mut paths_visited_twice = 0;
    while let Some(Path {
        next,
        mut visited,
        visited_small_twice,
    }) = to_visit.pop()
    {
        visited.insert(next);
        if next == "end" {
            paths += 1;
            paths_visited_twice += visited_small_twice as usize;
            continue;
        }
        for c in connections[next].iter() {
            if !visited.contains(c.as_str()) || c.chars().all(char::is_uppercase) {
                to_visit.push(Path::new(c, visited.clone(), visited_small_twice));
            } else if !visited_small_twice && c != "start" {
                to_visit.push(Path::new(c, visited.clone(), true));
            }
        }
    }
    println!("part 1: {}", paths - paths_visited_twice);
    println!("part 2: {}", paths);
}
