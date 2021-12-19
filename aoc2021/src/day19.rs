use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Copy)]
enum Pos {
    X,
    Y,
    Z,
}

impl Coord {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn apply(&self, neg: &Coord, remap: &(Pos, Pos, Pos)) -> Coord {
        Self::new(
            self.at(remap.0) * neg.x,
            self.at(remap.1) * neg.y,
            self.at(remap.2) * neg.z,
        )
    }

    fn at(&self, pos: Pos) -> i32 {
        match pos {
            Pos::X => self.x,
            Pos::Y => self.y,
            Pos::Z => self.z,
        }
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    beacons: HashSet<Coord>,
}

impl Scanner {
    fn new(beacons: HashSet<Coord>) -> Self {
        Self { beacons }
    }

    fn orientations(&self) -> ScannerRotations {
        ScannerRotations::new(self)
    }
}

struct ScannerRotations<'a> {
    scanner: &'a Scanner,
    negations: Vec<Coord>,
    negations_index: usize,
    remaps: Vec<(Pos, Pos, Pos)>,
    remap_index: usize,
}

impl<'a> ScannerRotations<'a> {
    fn new(scanner: &'a Scanner) -> Self {
        Self {
            scanner,
            negations: vec![
                Coord::new(1, 1, 1),
                Coord::new(1, 1, -1),
                Coord::new(1, -1, 1),
                Coord::new(1, -1, -1),
                Coord::new(-1, 1, 1),
                Coord::new(-1, 1, -1),
                Coord::new(-1, -1, 1),
                Coord::new(-1, -1, -1),
            ],
            negations_index: 0,
            remaps: vec![
                (Pos::X, Pos::Y, Pos::Z),
                (Pos::X, Pos::Z, Pos::Y),
                (Pos::Y, Pos::X, Pos::Z),
                (Pos::Y, Pos::Z, Pos::X),
                (Pos::Z, Pos::X, Pos::Y),
                (Pos::Z, Pos::Y, Pos::X),
            ],
            remap_index: 0,
        }
    }
}

impl<'a> Iterator for ScannerRotations<'a> {
    type Item = Scanner;

    fn next(&mut self) -> Option<Self::Item> {
        if self.negations_index >= self.negations.len() {
            return None;
        }
        let scanner = Scanner::new(
            self.scanner
                .beacons
                .iter()
                .map(|x| {
                    x.apply(
                        &self.negations[self.negations_index],
                        &self.remaps[self.remap_index],
                    )
                })
                .collect(),
        );
        self.remap_index += 1;
        if self.remap_index >= self.remaps.len() {
            self.remap_index = 0;
            self.negations_index += 1;
        }
        Some(scanner)
    }
}

fn overlaps(s1: &Scanner, s2: &Scanner, dists: &mut Vec<Coord>) -> Option<Scanner> {
    for s2 in s2.orientations() {
        for a in &s1.beacons {
            for b in &s2.beacons {
                let delta = Coord::new(b.x - a.x, b.y - a.y, b.z - a.z);
                let mut overlaps = 0;
                let mut mappings = HashSet::new();
                for bb in &s2.beacons {
                    let relative = Coord::new(bb.x - delta.x, bb.y - delta.y, bb.z - delta.z);
                    overlaps += s1.beacons.contains(&relative) as usize;
                    mappings.insert(relative);
                }
                if overlaps >= 12 {
                    dists.push(delta);
                    return Some(Scanner::new(mappings));
                }
            }
        }
    }
    None
}

fn main() {
    let file = File::open("aoc2021/inputs/day19.input").unwrap();
    let input = io::BufReader::new(file)
        .lines()
        .flatten()
        .collect::<Vec<_>>();
    let mut scanners = Vec::new();
    let mut beacons = HashSet::new();
    for l in input {
        if l.is_empty() {
            scanners.push(Scanner {
                beacons: std::mem::take(&mut beacons),
            });
        } else if l.starts_with("---") {
            continue;
        } else {
            let coords = l.split(',').map(|s| s.parse().unwrap()).collect::<Vec<_>>();
            beacons.insert(Coord {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            });
        }
    }
    scanners.push(Scanner { beacons });
    let mut map = HashSet::new();
    map.extend(scanners[0].beacons.iter().cloned());
    let mut dists = vec![Coord::new(0, 0, 0)];
    let mut aligned_indices = HashSet::new();
    aligned_indices.insert(0);
    let mut aligned = HashMap::<usize, Scanner>::new();
    aligned.insert(0, scanners[0].clone());
    let mut unaligned = HashSet::new();
    while aligned_indices.len() < scanners.len() {
        for (i, scanner) in scanners.iter().enumerate() {
            if aligned_indices.contains(&i) {
                continue;
            }
            for &j in &aligned_indices {
                if unaligned.contains(&(i, j)) {
                    continue;
                }
                if let Some(scanner) = overlaps(aligned.get(&j).unwrap(), scanner, &mut dists) {
                    aligned_indices.insert(i);
                    map.extend(scanner.beacons.iter().cloned());
                    aligned.insert(i, scanner);
                    break;
                }
                unaligned.insert((i, j));
            }
        }
    }
    let mut mdists = Vec::new();
    for a in dists.iter() {
        for b in dists.iter() {
            mdists.push((a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs());
        }
    }
    println!("part 1: {}", map.len());
    println!("part 2: {}", mdists.into_iter().max().unwrap());
}
