use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            'U' => Direction::Up,
            'D' => Direction::Down,
            _ => unreachable!(),
        }
    }
}

fn decode(dirs: &[Vec<Direction>], keypad: HashMap<(i32, i32), char>) -> Vec<char> {
    let mut coord = (0, 0);
    dirs.iter()
        .map(|dirs| {
            for d in dirs {
                let mut new_coord = coord;
                match d {
                    Direction::Left => new_coord.0 -= 1,
                    Direction::Right => new_coord.0 += 1,
                    Direction::Up => new_coord.1 += 1,
                    Direction::Down => new_coord.1 -= 1,
                }
                if keypad.contains_key(&new_coord) {
                    coord = new_coord
                }
            }
            *keypad.get(&coord).unwrap()
        })
        .collect()
}

fn main() {
    let file = File::open("aoc2016/inputs/day02.input").unwrap();
    let input = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().chars().map(Direction::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let keypad1 = HashMap::from_iter(
        [
            ((-1, 1), '1'),
            ((0, 1), '2'),
            ((1, 1), '3'),
            ((-1, 0), '4'),
            ((0, 0), '5'),
            ((1, 0), '6'),
            ((-1, -1), '7'),
            ((0, -1), '8'),
            ((1, -1), '9'),
        ],
    );
    println!(
        "part 1: {}",
        decode(&input, keypad1).into_iter().collect::<String>()
    );
    let keypad2 = HashMap::from_iter(
        [
            ((2, 2), '1'),
            ((1, 1), '2'),
            ((2, 1), '3'),
            ((3, 1), '4'),
            ((0, 0), '5'),
            ((1, 0), '6'),
            ((2, 0), '7'),
            ((3, 0), '8'),
            ((4, 0), '9'),
            ((1, -1), 'A'),
            ((2, -1), 'B'),
            ((3, -1), 'C'),
            ((2, -2), 'D'),
        ],
    );
    println!(
        "part 2: {}",
        decode(&input, keypad2).into_iter().collect::<String>()
    );
}
