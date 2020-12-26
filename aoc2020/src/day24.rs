use std::collections::HashMap;
use std::fs;

#[derive(Default, Eq, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn neighbors(&self) -> Vec<Coord> {
        let mut r = vec![
            Coord {
                x: self.x - 2,
                y: self.y,
            },
            Coord {
                x: self.x + 2,
                y: self.y,
            },
        ];
        for x in [-1, 1].iter() {
            for y in [-1, 1].iter() {
                r.push(Coord {
                    x: self.x + x,
                    y: self.y + y,
                });
            }
        }
        r
    }
}

struct Tile {
    black: bool,
}

impl Tile {
    fn black() -> Self {
        Self { black: true }
    }

    fn white() -> Self {
        Self { black: false }
    }

    fn flip(&mut self) {
        self.black = !self.black;
    }
}

type Layout = HashMap<Coord, Tile>;

fn black_tiles(l: &Layout) -> usize {
    l.values().filter(|t| t.black).count()
}

fn next_day(l: Layout) -> Layout {
    let mut ln = HashMap::<Coord, (Tile, usize)>::new();
    for (c, t) in l.into_iter() {
        for n in c.neighbors() {
            ln.entry(n)
                .and_modify(|nt| nt.1 += t.black as usize)
                .or_insert((Tile::white(), t.black as usize));
        }
        ln.entry(c)
            .and_modify(|nt| nt.0.black = t.black)
            .or_insert((t, 0));
    }
    ln.into_iter()
        .map(|(c, (mut t, bn))| {
            match (bn, &t) {
                (x, Tile { black: true }) if x == 0 || x > 2 => t.flip(),
                (2, Tile { black: false }) => t.flip(),
                _ => {}
            };
            (c, t)
        })
        .filter(|(_, t)| t.black)
        .collect()
}

fn main() {
    let mut layout = Layout::new();
    for l in fs::read_to_string("aoc2020/inputs/day24.input")
        .unwrap()
        .lines()
    {
        let mut coord = Coord::default();
        let mut i = 0;
        while i < l.len() {
            let x_plane = |i: usize, cnt: i32, coord: &mut Coord| {
                match l.chars().nth(i) {
                    Some('e') => coord.x += cnt,
                    Some('w') => coord.x -= cnt,
                    _ => panic!(),
                };
            };
            match l.chars().nth(i) {
                Some('n') => {
                    coord.y += 1;
                    x_plane(i + 1, 1, &mut coord);
                    i += 1;
                }
                Some('s') => {
                    coord.y -= 1;
                    x_plane(i + 1, 1, &mut coord);
                    i += 1;
                }
                _ => x_plane(i, 2, &mut coord),
            };
            i += 1;
        }
        layout
            .entry(coord)
            .and_modify(Tile::flip)
            .or_insert_with(Tile::black);
    }
    println!("part 1: {}", black_tiles(&layout));
    println!(
        "part 2: {}",
        black_tiles(&(0..100).fold(layout, |l, _| next_day(l)))
    );
}
