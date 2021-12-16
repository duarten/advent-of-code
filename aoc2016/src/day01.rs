use std::{collections::HashSet, fs};

#[derive(Clone, Copy)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::North
    }
}

impl From<usize> for Direction {
    fn from(value: usize) -> Self {
        match value {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        ((*self as usize + 1) % 4).into()
    }

    fn rotate_left(&self) -> Direction {
        ((*self as usize).wrapping_sub(1) % 4).into()
    }
}

#[derive(Default)]
struct Waypoint {
    visited: HashSet<(i32, i32)>,
    facing: Direction,
    x: i32,
    y: i32,
    visited_twice: Option<i32>,
}

impl Waypoint {
    fn value(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn forward(&mut self, blocks: i32) {
        for _ in 0..blocks {
            match self.facing {
                Direction::North => self.y += 1,
                Direction::South => self.y -= 1,
                Direction::East => self.x += 1,
                Direction::West => self.x -= 1,
            }
            if self.visited_twice.is_none() && !self.visited.insert((self.x, self.y)) {
                self.visited_twice = Some(self.value())
            }
        }
    }

    fn rotate_left_and_move(&mut self, blocks: i32) {
        self.facing = self.facing.rotate_left();
        self.forward(blocks);
    }

    fn rotate_right_and_move(&mut self, blocks: i32) {
        self.facing = self.facing.rotate_right();
        self.forward(blocks);
    }
}

fn main() {
    let mut waypoint = Waypoint::default();
    fs::read_to_string("aoc2016/inputs/day01.input")
        .unwrap()
        .split(", ")
        .for_each(|dir| {
            let blocks = dir[1..].parse().unwrap();
            match dir.chars().next().unwrap() {
                'L' => waypoint.rotate_left_and_move(blocks),
                'R' => waypoint.rotate_right_and_move(blocks),
                _ => unreachable!(),
            };
        });
    println!("part 1: {}", waypoint.value());
    println!("part 2: {}", waypoint.visited_twice.unwrap());
}
