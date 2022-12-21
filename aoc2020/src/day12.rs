use std::{
    fs,
    io::{self, BufRead},
    str::FromStr,
};

use anyhow::{anyhow, Result};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cardinal {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Cardinal {
    fn inverse(&self) -> Self {
        match self {
            Cardinal::East => Cardinal::West,
            Cardinal::West => Cardinal::East,
            Cardinal::North => Cardinal::South,
            Cardinal::South => Cardinal::North,
        }
    }

    fn from(v: usize) -> Self {
        match v {
            0 => Cardinal::North,
            1 => Cardinal::East,
            2 => Cardinal::South,
            3 => Cardinal::West,
            _ => panic!("oh no"),
        }
    }

    fn rotate_left(self, degrees: usize) -> Self {
        self.rotate_right(360 - degrees)
    }

    fn rotate_right(self, degrees: usize) -> Self {
        Self::from((self as usize + (degrees / 90)) % 4)
    }
}

#[derive(Debug)]
struct Position {
    cardinal: Cardinal,
    value: usize,
}

impl Position {
    fn new(cardinal: Cardinal, value: usize) -> Self {
        Self { cardinal, value }
    }

    fn rotate_left(&mut self, v: usize) {
        self.cardinal = self.cardinal.rotate_left(v);
    }

    fn rotate_right(&mut self, v: usize) {
        self.cardinal = self.cardinal.rotate_right(v);
    }

    fn move_toward(&mut self, c: Cardinal, v: usize) {
        if self.cardinal == c {
            self.value += v;
        } else if c == self.cardinal.inverse() {
            let diff = usize::min(v, self.value);
            self.value -= diff;
            let rem = v - diff;
            if rem > 0 {
                self.value += rem;
                self.cardinal = c;
            }
        }
    }
}

#[derive(Debug)]
struct Waypoint {
    p1: Position,
    p2: Position,
}

impl Waypoint {
    fn new() -> Self {
        Self {
            p1: Position::new(Cardinal::East, 10),
            p2: Position::new(Cardinal::North, 1),
        }
    }

    fn rotate_left(&mut self, v: usize) {
        self.p1.rotate_left(v);
        self.p2.rotate_left(v);
    }

    fn rotate_right(&mut self, v: usize) {
        self.p1.rotate_right(v);
        self.p2.rotate_right(v);
    }

    fn move_toward(&mut self, c: Cardinal, v: usize) {
        self.p1.move_toward(c, v);
        self.p2.move_toward(c, v);
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Cardinal(Cardinal, usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = usize::from_str(&s[1..])?;
        match &s[0..1] {
            "N" => Ok(Instruction::Cardinal(Cardinal::North, v)),
            "S" => Ok(Instruction::Cardinal(Cardinal::South, v)),
            "E" => Ok(Instruction::Cardinal(Cardinal::East, v)),
            "W" => Ok(Instruction::Cardinal(Cardinal::West, v)),
            "L" => Ok(Instruction::Left(v)),
            "R" => Ok(Instruction::Right(v)),
            "F" => Ok(Instruction::Forward(v)),
            _ => Err(anyhow!("unable to parse {}", s)),
        }
    }
}

#[derive(Debug)]
struct Ship {
    p1: Position,
    p2: Position,
    facing: Cardinal,
}

impl Ship {
    fn new() -> Self {
        Self {
            p1: Position::new(Cardinal::East, 0),
            p2: Position::new(Cardinal::North, 0),
            facing: Cardinal::East,
        }
    }

    fn move_toward(&mut self, c: Cardinal, v: usize) {
        self.p1.move_toward(c, v);
        self.p2.move_toward(c, v);
    }

    fn rotate_left(&mut self, v: usize) {
        self.facing = self.facing.rotate_left(v);
    }

    fn rotate_right(&mut self, v: usize) {
        self.facing = self.facing.rotate_right(v);
    }

    fn move_forward(&mut self, v: usize) {
        self.move_toward(self.facing, v);
    }

    fn move_to(&mut self, w: &Waypoint, v: usize) {
        self.move_toward(w.p1.cardinal, w.p1.value * v);
        self.move_toward(w.p2.cardinal, w.p2.value * v);
    }
}

fn apply_pt1(s: &mut Ship, inst: Instruction) {
    match inst {
        Instruction::Cardinal(c, v) => s.move_toward(c, v),
        Instruction::Left(v) => s.rotate_left(v),
        Instruction::Right(v) => s.rotate_right(v),
        Instruction::Forward(v) => s.move_forward(v),
    }
}

fn apply_pt2(s: &mut Ship, w: &mut Waypoint, inst: Instruction) {
    match inst {
        Instruction::Cardinal(c, v) => w.move_toward(c, v),
        Instruction::Left(v) => w.rotate_left(v),
        Instruction::Right(v) => w.rotate_right(v),
        Instruction::Forward(v) => s.move_to(w, v),
    }
}

fn main() {
    let file = fs::File::open("aoc2020/inputs/day12.input").unwrap();
    let instructions: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    let mut s = Ship::new();
    instructions
        .iter()
        .copied()
        .for_each(|i| apply_pt1(&mut s, i));
    println!("manhattan distance pt 1: {}", s.p1.value + s.p2.value);
    let mut s = Ship::new();
    let mut w = Waypoint::new();
    instructions
        .into_iter()
        .for_each(|i| apply_pt2(&mut s, &mut w, i));
    println!("manhattan distance pt 2: {}", s.p1.value + s.p2.value);
}
