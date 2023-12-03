use std::fs;

fn sine(angle: usize) -> f64 {
    match angle {
        0 => 0.0,
        90 => 1.0,
        180 => 0.0,
        270 => -1.0,
        _ => panic!(),
    }
}

fn cosine(angle: usize) -> f64 {
    match angle {
        0 => 1.0,
        90 => 0.0,
        180 => -1.0,
        270 => 0.0,
        _ => panic!(),
    }
}

type TileId = usize;

#[derive(Debug, Default)]
struct Tile {
    id: TileId,
    pattern: Vec<char>,
    sqr: usize,
    flipped_horizontally: bool,
    rotate_angle: usize, // 0, 90, 180, 270
    top: Option<TileId>,
    bottom: Option<TileId>,
    left: Option<TileId>,
    right: Option<TileId>,
    stiched: bool,
}

impl Tile {
    fn matches<'a>(
        &self,
        others: &'a mut [Tile],
        matches: fn(&Tile, &Tile) -> bool,
    ) -> Option<&'a mut Tile> {
        'outer: for other in others.iter_mut() {
            for _ in 0..2 {
                for _ in 0..4 {
                    if matches(self, other) {
                        return Some(other);
                    } else if other.connections() > 0 {
                        continue 'outer;
                    }
                    other.rotate_left();
                }
                other.flip_horizontally();
            }
        }
        None
    }

    fn try_stich(&mut self, others: &mut [Tile]) {
        if self.top.is_none() {
            if let Some(t) = self.matches(others, Tile::match_top_bottom) {
                self.top = Some(t.id);
                t.bottom = Some(self.id);
            }
        }
        if self.bottom.is_none() {
            if let Some(t) = self.matches(others, Tile::match_bottom_top) {
                self.bottom = Some(t.id);
                t.top = Some(self.id);
            }
        }
        if self.left.is_none() {
            if let Some(t) = self.matches(others, Tile::match_left_right) {
                self.left = Some(t.id);
                t.right = Some(self.id);
            }
        }
        if self.right.is_none() {
            if let Some(t) = self.matches(others, Tile::match_right_left) {
                self.right = Some(t.id);
                t.left = Some(self.id);
            }
        }
    }

    fn connections(&self) -> usize {
        self.top.is_some() as usize
            + self.bottom.is_some() as usize
            + self.left.is_some() as usize
            + self.right.is_some() as usize
    }

    fn flip_horizontally(&mut self) {
        self.flipped_horizontally = !self.flipped_horizontally;
        std::mem::swap(&mut self.left, &mut self.right);
    }

    fn rotate_left(&mut self) {
        self.rotate_angle = (self.rotate_angle + 90) % 360;
        std::mem::swap(&mut self.top, &mut self.left);
        std::mem::swap(&mut self.top, &mut self.bottom);
        std::mem::swap(&mut self.top, &mut self.right);
    }

    fn match_top_bottom(&self, t: &Tile) -> bool {
        for i in 0..self.sqr {
            if self.pixel_at(i, 0) != t.pixel_at(i, self.sqr - 1) {
                return false;
            }
        }
        true
    }

    fn match_bottom_top(&self, t: &Tile) -> bool {
        for i in 0..self.sqr {
            if self.pixel_at(i, self.sqr - 1) != t.pixel_at(i, 0) {
                return false;
            }
        }
        true
    }

    fn match_left_right(&self, t: &Tile) -> bool {
        for i in 0..self.sqr {
            if self.pixel_at(0, i) != t.pixel_at(self.sqr - 1, i) {
                return false;
            }
        }
        true
    }

    fn match_right_left(&self, t: &Tile) -> bool {
        for i in 0..self.sqr {
            if self.pixel_at(self.sqr - 1, i) != t.pixel_at(0, i) {
                return false;
            }
        }
        true
    }

    fn pixel_at(&self, mut x: usize, mut y: usize) -> char {
        let m = (self.sqr - 1) as f64 / 2.0;
        if self.rotate_angle > 0 {
            let new_x = cosine(self.rotate_angle) * (x as f64 - m)
                - sine(self.rotate_angle) * (y as f64 - m)
                + m;
            let new_y = sine(self.rotate_angle) * (x as f64 - m)
                + cosine(self.rotate_angle) * (y as f64 - m)
                + m;
            x = new_x as usize;
            y = new_y as usize;
        }
        if self.flipped_horizontally {
            x = self.sqr - x - 1;
        }
        self.pattern[y * self.sqr + x]
    }

    fn remove_borders(self) -> Self {
        let mut pattern = Vec::<char>::new();
        for y in 1..(self.sqr - 1) {
            for x in 1..(self.sqr - 1) {
                pattern.push(self.pixel_at(x, y));
            }
        }
        Tile {
            pattern,
            id: self.id,
            sqr: self.sqr - 2,
            flipped_horizontally: false,
            rotate_angle: 0,
            ..self
        }
    }
}

fn find_tile_position(id: TileId, tiles: &[Tile]) -> Option<usize> {
    tiles.iter().position(|t| t.id == id)
}

fn stich_node(id: TileId, tiles: &mut [Tile]) {
    if let Some(idx) = find_tile_position(id, tiles) {
        tiles.swap(0, idx);
        if let Some((n, others)) = tiles.split_first_mut() {
            if n.stiched {
                return;
            }
            n.try_stich(others);
            n.stiched = true;
            for edge in [n.top, n.bottom, n.left, n.right].iter().flatten() {
                stich_node(*edge, others);
            }
        }
    }
}

struct Picture {
    tiles: Vec<Tile>,
    sqr: usize,
}

impl Picture {
    fn new(mut tiles: Vec<Tile>) -> Self {
        stich_node(tiles[0].id, &mut tiles);
        let tiles = tiles
            .into_iter()
            .map(|t| t.remove_borders())
            .collect::<Vec<_>>();
        let sqr = (tiles.len() as f64).sqrt() as usize * 8;
        Self { tiles, sqr }
    }

    fn count_hashes(&self) -> usize {
        self.tiles
            .iter()
            .map(|t| t.pattern.iter().filter(|c| **c == '#').count())
            .sum()
    }

    fn rotate_left(&mut self) {
        for t in self.tiles.iter_mut() {
            t.rotate_left();
        }
    }

    fn flip_horizontally(&mut self) {
        for t in self.tiles.iter_mut() {
            t.flip_horizontally();
        }
    }

    fn pixel_at(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.sqr || y >= self.sqr {
            None
        } else {
            let t = self.find_tile(x / 8, y / 8);
            Some(t.pixel_at(x % 8, y % 8))
        }
    }

    fn find_tile(&self, x: usize, y: usize) -> &Tile {
        let mut t = self
            .tiles
            .iter()
            .find(|t| t.top.is_none() && t.left.is_none())
            .unwrap();
        for _ in 0..x {
            let pos = find_tile_position(t.right.unwrap(), &self.tiles).unwrap();
            t = &self.tiles[pos];
        }
        for _ in 0..y {
            let pos = find_tile_position(t.bottom.unwrap(), &self.tiles).unwrap();
            t = &self.tiles[pos];
        }
        t
    }

    fn count_sea_monsters(&self) -> usize {
        let mut cnt = 0;
        for y in 1..(self.sqr - 1) {
            for x in 0..(self.sqr - 19) {
                cnt += [
                    (x, y),
                    (x + 1, y + 1),
                    (x + 4, y + 1),
                    (x + 5, y),
                    (x + 6, y),
                    (x + 7, y + 1),
                    (x + 10, y + 1),
                    (x + 11, y),
                    (x + 12, y),
                    (x + 13, y + 1),
                    (x + 16, y + 1),
                    (x + 17, y),
                    (x + 18, y),
                    (x + 18, y - 1),
                    (x + 19, y),
                ]
                .iter()
                .all(|(x, y)| self.pixel_at(*x, *y) == Some('#')) as usize
            }
        }
        cnt
    }
}

fn main() {
    let input = fs::read_to_string("aoc2020/inputs/day20.input").unwrap();
    let tiles: Vec<_> = input
        .split("\n\n")
        .collect::<Vec<_>>()
        .into_iter()
        .map(|t| {
            let parts = t.split(':').collect::<Vec<_>>();
            Tile {
                id: parts[0][5..].parse().unwrap(),
                pattern: parts[1].chars().filter(|c| !c.is_whitespace()).collect(),
                sqr: 10,
                ..Default::default()
            }
        })
        .collect();
    let mut p = Picture::new(tiles);
    let corners: usize = p
        .tiles
        .iter()
        .filter(|t| t.connections() == 2)
        .map(|t| t.id)
        .product();
    println!("corners: {:?}", corners);
    for _ in 0..2 {
        for _ in 0..4 {
            let x = p.count_sea_monsters();
            if x > 0 {
                println!("sea monsters: {}", x);
                println!("roughness: {}", p.count_hashes() - 15 * x);
                return;
            }
            p.rotate_left();
        }
        p.flip_horizontally();
    }
}
