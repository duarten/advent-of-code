use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    iter::repeat,
    str,
};

use utils::abs_diff;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn diag(&self, other: &Point) -> bool {
        abs_diff(self.x, other.x) == abs_diff(self.y, other.y)
    }
}

impl str::FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").expect("point");
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Debug)]
struct Line(Point, Point);

fn range(a: usize, b: usize) -> Box<dyn Iterator<Item = usize>> {
    if a <= b {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    }
}

fn points(x: impl Iterator<Item = usize>, y: impl Iterator<Item = usize>) -> Vec<Point> {
    x.zip(y).map(|(x, y)| Point { x, y }).collect()
}

impl Line {
    fn points(&self) -> Vec<Point> {
        if self.0.x == self.1.x {
            points(repeat(self.0.x), range(self.0.y, self.1.y))
        } else if self.0.y == self.1.y {
            points(range(self.0.x, self.1.x), repeat(self.0.y))
        } else {
            vec![]
        }
    }

    fn points_diag(&self) -> Vec<Point> {
        if self.0.diag(&self.1) {
            points(range(self.0.x, self.1.x), range(self.0.y, self.1.y))
        } else {
            vec![]
        }
    }
}

impl str::FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(" -> ").expect("line");
        Ok(Self(a.parse()?, b.parse()?))
    }
}

fn main() {
    let file = File::open("aoc2021/inputs/day05.input").unwrap();
    let lines = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect::<Vec<Line>>();
    let mut map: HashMap<Point, usize> = HashMap::default();
    for p in lines.iter().flat_map(Line::points) {
        *map.entry(p).or_insert(0) += 1;
    }
    println!("part 1: {}", map.values().filter(|&&v| v > 1).count());
    for p in lines.iter().flat_map(Line::points_diag) {
        *map.entry(p).or_insert(0) += 1;
    }
    println!("part 2: {}", map.values().filter(|&&v| v > 1).count());
}
