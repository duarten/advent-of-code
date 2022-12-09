use std::{
    fs::File,
    io::{self, BufRead},
    iter::repeat,
};

struct Trees(Vec<Vec<usize>>);

type Offset = (usize, usize);

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn step(&self) -> (isize, isize) {
        match self {
            Direction::Down => (1, 0),
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
        }
    }

    fn adjust(&self, offset: Offset) -> Offset {
        let step = self.step();
        (
            offset.0.wrapping_add(step.0 as usize),
            offset.1.wrapping_add(step.1 as usize),
        )
    }
}

impl Trees {
    fn offsets(&self) -> Vec<Offset> {
        (0..self.0.len())
            .flat_map(|row| repeat(row).zip(0..self.0[0].len()))
            .collect()
    }

    fn viewing_distance(
        &self,
        target_height: usize,
        o: Offset,
        dir: Direction,
        dist: usize,
    ) -> (usize, bool) {
        if let Some(offset_height) = self.at(o) {
            if offset_height >= target_height {
                return (dist + 1, false);
            }
            let neighbor = dir.adjust(o);
            self.viewing_distance(target_height, neighbor, dir, dist + 1)
        } else {
            (dist, true)
        }
    }

    fn eval(&self, o: Offset) -> (usize, bool) {
        let height = self.at(o).unwrap();
        let mut visible = false;
        let mut scenic_score = 1;
        use Direction::*;
        for dir in [Up, Down, Right, Left] {
            let dist = self.viewing_distance(height, dir.adjust(o), dir, 0);
            scenic_score *= dist.0;
            visible = visible || dist.1;
        }
        (scenic_score, visible)
    }

    fn at(&self, index: Offset) -> Option<usize> {
        (index.0 < self.0.len() && index.1 < self.0[0].len()).then(|| self.0[index.0][index.1])
    }
}

fn main() {
    let file = File::open("aoc2022/inputs/day08.input").unwrap();
    let input = io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let trees = Trees(input);
    let mut total_visible = 0;
    let mut best_score = 0;
    for o in trees.offsets() {
        let (scenic_score, visible) = trees.eval(o);
        total_visible += visible as usize;
        best_score = std::cmp::max(best_score, scenic_score);
    }
    println!("part 1: {}", total_visible);
    println!("part 2: {}", best_score);
}
