use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

struct Wrapper {
    length: usize,
    width: usize,
    height: usize,
}

impl Wrapper {
    fn sides(&self) -> [usize; 3] {
        [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ]
    }

    fn perimeters(&self) -> [usize; 3] {
        [
            2 * self.length + 2 * self.width,
            2 * self.width + 2 * self.height,
            2 * self.height + 2 * self.length,
        ]
    }

    fn slack(&self) -> usize {
        self.sides().iter().copied().min().unwrap()
    }

    fn area(&self) -> usize {
        self.sides().iter().copied().map(|s| s * 2).sum::<usize>()
    }

    fn volume(&self) -> usize {
        self.length * self.height * self.width
    }

    fn paper(&self) -> usize {
        self.slack() + self.area()
    }

    fn ribbon(&self) -> usize {
        self.perimeters().iter().copied().min().unwrap() + self.volume()
    }
}

impl FromStr for Wrapper {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('x');
        Ok(Wrapper {
            length: split.next().unwrap().parse()?,
            width: split.next().unwrap().parse()?,
            height: split.next().unwrap().parse()?,
        })
    }
}

fn main() {
    let file = File::open("aoc2015/inputs/day02.input").unwrap();
    let dims: Vec<_> = BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();
    println!("part 1: {}", dims.iter().map(Wrapper::paper).sum::<usize>());
    println!(
        "part 2: {}",
        dims.iter().map(Wrapper::ribbon).sum::<usize>()
    );
}
