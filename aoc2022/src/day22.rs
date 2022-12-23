use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

use nom::{
    branch::alt,
    character::complete::{char, u64},
    combinator::map,
    multi::many1,
    IResult,
};

type Offset = (isize, isize);

#[derive(Clone, Copy)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn turn_clockwise(self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn turn_counterclockwise(self) -> Direction {
        match self {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }

    fn reverse(self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    fn step(self, (x, y): Offset) -> Offset {
        match self {
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    TurnClockwise,
    TurnCounterclockwise,
    Forward(u64),
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    many1(alt((
        map(char('R'), |_| Instruction::TurnClockwise),
        map(char('L'), |_| Instruction::TurnCounterclockwise),
        map(u64, Instruction::Forward),
    )))(input)
}

fn wrap1(map: &HashMap<Offset, char>, mut pos: Offset, dir: Direction) -> (Offset, Direction) {
    let rev = dir.reverse();
    while map.contains_key(&pos) {
        pos = rev.step(pos);
    }
    (dir.step(pos), dir)
}

fn wrap2(_: &HashMap<Offset, char>, pos: Offset, dir: Direction) -> (Offset, Direction) {
    let (cube_x, cube_y, new_dir) = match (pos.0 / 50, pos.1 / 50, dir) {
        (0, 2, Direction::Up) => (1, 1, Direction::Right),
        (0, 2, Direction::Left) => (1, 0, Direction::Right),
        (0, 3, Direction::Right) => (1, 2, Direction::Up),
        (0, 3, Direction::Down) => (2, 0, Direction::Down),
        (0, 3, Direction::Left) => (1, 0, Direction::Down),
        (1, 0, Direction::Up) => (0, 3, Direction::Right),
        (1, 0, Direction::Left) => (0, 2, Direction::Right),
        (1, 1, Direction::Right) => (2, 0, Direction::Up),
        (1, 1, Direction::Left) => (0, 2, Direction::Down),
        (1, 2, Direction::Right) => (2, 0, Direction::Left),
        (1, 2, Direction::Down) => (0, 3, Direction::Left),
        (2, 0, Direction::Up) => (0, 3, Direction::Up),
        (2, 0, Direction::Right) => (1, 2, Direction::Left),
        (2, 0, Direction::Down) => (1, 1, Direction::Left),
        _ => unreachable!(),
    };

    let (x_idx, y_idx) = (pos.0 % 50, pos.1 % 50);
    let i = match dir {
        Direction::Left => 49 - y_idx,
        Direction::Right => y_idx,
        Direction::Up => x_idx,
        Direction::Down => 49 - x_idx,
    };
    let new_x = match new_dir {
        Direction::Left => 49,
        Direction::Right => 0,
        Direction::Up => i,
        Direction::Down => 49 - i,
    };
    let new_y = match new_dir {
        Direction::Left => 49 - i,
        Direction::Right => i,
        Direction::Up => 49,
        Direction::Down => 0,
    };

    let new_pos = (cube_x * 50 + new_x, cube_y * 50 + new_y);
    (new_pos, new_dir)
}

fn solve(
    map: &HashMap<Offset, char>,
    insts: &[Instruction],
    wrap: impl Fn(&HashMap<Offset, char>, Offset, Direction) -> (Offset, Direction),
) -> usize {
    let x = (0..).take_while(|x| !map.contains_key(&(*x, 0))).count();
    let mut pos = (x as isize, 0);
    let mut dir = Direction::Right;
    for inst in insts {
        match inst {
            Instruction::TurnClockwise => {
                dir = dir.turn_clockwise();
            }
            Instruction::TurnCounterclockwise => {
                dir = dir.turn_counterclockwise();
            }
            Instruction::Forward(n) => {
                for _ in 0..*n {
                    let npos = dir.step(pos);
                    match map.get(&npos) {
                        Some('#') => break,
                        None => {
                            let (npos, newdir) = wrap(map, pos, dir);
                            if matches!(map.get(&npos), Some('#')) {
                                break;
                            }
                            pos = npos;
                            dir = newdir;
                        }
                        _ => {
                            pos = npos;
                        }
                    }
                }
            }
        }
    }
    ((pos.1 + 1) * 1000 + (pos.0 + 1) * 4) as usize + dir as usize
}

fn main() {
    let file = File::open("aoc2022/inputs/day22.input").unwrap();
    let mut lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
    let mut map = HashMap::new();
    for (y, l) in lines.by_ref().take_while(|l| !l.is_empty()).enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c != ' ' {
                map.insert((x as isize, y as isize), c);
            }
        }
    }
    let insts = instructions(&lines.next().unwrap()).unwrap().1;
    println!("part 1: {}", solve(&map, &insts, wrap1));
    println!("part 2: {}", solve(&map, &insts, wrap2));
}
