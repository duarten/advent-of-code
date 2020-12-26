#![feature(bool_to_option, min_const_generics)]

use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::mem::{self, MaybeUninit};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Cube(i64, i64, i64, i64);

impl Cube {
    fn neighbors(&self) -> [Cube; 26] {
        unsafe { mem::transmute::<_, [Cube; 26]>(self.neighbords_sized::<26>(false)) }
    }

    fn neighbors_hyper(&self) -> [Cube; 80] {
        unsafe { mem::transmute::<_, [Cube; 80]>(self.neighbords_sized::<80>(true)) }
    }

    fn neighbords_sized<const N: usize>(&self, hyper: bool) -> [MaybeUninit<Cube>; N] {
        let mut neighbors: [MaybeUninit<Cube>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        let mut i = 0;
        let rs = [-1, 0, 1];
        for x in rs.iter() {
            for y in rs.iter() {
                for z in rs.iter() {
                    let non_hyper = [0];
                    for w in if hyper { rs.iter() } else { non_hyper.iter() } {
                        if *x == 0 && *y == 0 && *z == 0 && *w == 0 {
                            continue;
                        }
                        neighbors[i] = MaybeUninit::new(Self(
                            self.0 + *x,
                            self.1 + *y,
                            self.2 + *z,
                            self.3 + *w,
                        ));
                        i += 1;
                    }
                }
            }
        }
        neighbors
    }
}

fn cycle<F, const N: usize>(d: Vec<Cube>, neighbors: F) -> Vec<Cube>
where
    F: Fn(&Cube) -> [Cube; N],
{
    let mut cache = HashMap::<Cube, (bool, usize)>::new();
    for c in d.into_iter() {
        cache
            .entry(c)
            .and_modify(|(a, _)| *a = true)
            .or_insert((true, 0));
        for n in neighbors(&c).iter() {
            cache
                .entry(*n)
                .and_modify(|(_, nc)| *nc += 1)
                .or_insert((false, 1));
        }
    }
    cache
        .into_iter()
        .filter_map(|(c, (active, an))| (an == 3 || active && an == 2).then_some(c))
        .collect()
}

fn main() {
    let file = File::open("aoc2020/inputs/day17.input").unwrap();
    let d: Vec<_> = io::BufReader::new(file)
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.unwrap()
                .chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some(Cube(x as i64, y as i64, 0, 0))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let x = (0..6).fold(d.clone(), |acc, _| cycle(acc, Cube::neighbors));
    println!("part 1: {}", x.len());
    let x = (0..6).fold(d, |acc, _| cycle(acc, Cube::neighbors_hyper));
    println!("part 2: {}", x.len());
}
