use std::{
    fs::File,
    io::{self, BufRead},
};

// https://github.com/rust-lang/rust/issues/88581
pub const fn div_ceil(lhs: usize, rhs: usize) -> usize {
    let d = lhs / rhs;
    let r = lhs % rhs;
    if r > 0 && rhs > 0 {
        d + 1
    } else {
        d
    }
}

enum ExplodeResult {
    Replace(Number, (usize, usize)),
    AddLeft(usize),
    AddRight(usize),
    AlreadyExploded,
    NoExplode,
}

enum SplitResult {
    Split(Pair),
    AlreadySplit,
    NoSplit,
}

#[derive(Clone)]
enum Number {
    Pair(Box<Pair>),
    Literal(usize),
}

impl Number {
    fn magnitude(&self) -> usize {
        match self {
            Number::Pair(p) => p.magnitude(),
            Number::Literal(x) => *x,
        }
    }

    fn explode(&mut self, nested: usize) -> ExplodeResult {
        match self {
            Number::Pair(p) => p.explode(nested),
            Number::Literal(_) => ExplodeResult::NoExplode,
        }
    }

    fn split(&mut self) -> SplitResult {
        match self {
            Number::Pair(p) => p.split(),
            Number::Literal(x) if *x >= 10 => SplitResult::Split(Pair {
                left: Number::Literal(*x / 2),
                right: Number::Literal(div_ceil(*x, 2)),
            }),
            Number::Literal(_) => SplitResult::NoSplit,
        }
    }

    fn stash_left(&mut self, value: usize) {
        match self {
            Number::Pair(p) => p.stash_left(value),
            Number::Literal(x) => *x += value,
        }
    }

    fn stash_right(&mut self, value: usize) {
        match self {
            Number::Pair(p) => p.stash_right(value),
            Number::Literal(x) => *x += value,
        }
    }
}

#[derive(Clone)]
struct Pair {
    left: Number,
    right: Number,
}

impl Pair {
    fn reduce(mut self) -> Self {
        loop {
            if matches!(self.explode(0), ExplodeResult::NoExplode)
                && matches!(self.split(), SplitResult::NoSplit)
            {
                break;
            }
        }
        self
    }

    fn magnitude(&self) -> usize {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }

    fn explode(&mut self, nested: usize) -> ExplodeResult {
        if nested == 4 {
            if let (Number::Literal(left), Number::Literal(right)) = (&self.left, &self.right) {
                return ExplodeResult::Replace(Number::Literal(0), (*left, *right));
            }
            unreachable!();
        }
        match self.left.explode(nested + 1) {
            ExplodeResult::Replace(nleft, (left, right)) => {
                self.left = nleft;
                self.right.stash_left(right);
                ExplodeResult::AddLeft(left)
            }
            ExplodeResult::AddRight(right) => {
                self.right.stash_left(right);
                ExplodeResult::AlreadyExploded
            }
            ExplodeResult::NoExplode => match self.right.explode(nested + 1) {
                ExplodeResult::Replace(nright, (left, right)) => {
                    self.right = nright;
                    self.left.stash_right(left);
                    ExplodeResult::AddRight(right)
                }
                ExplodeResult::AddLeft(left) => {
                    self.left.stash_right(left);
                    ExplodeResult::AlreadyExploded
                }
                r => r,
            },
            r => r,
        }
    }

    fn stash_left(&mut self, value: usize) {
        self.left.stash_left(value);
    }

    fn stash_right(&mut self, value: usize) {
        self.right.stash_right(value);
    }

    fn split(&mut self) -> SplitResult {
        match self.left.split() {
            SplitResult::Split(left) => {
                self.left = Number::Pair(Box::new(left));
                SplitResult::AlreadySplit
            }
            SplitResult::AlreadySplit => SplitResult::AlreadySplit,
            SplitResult::NoSplit => match self.right.split() {
                SplitResult::Split(right) => {
                    self.right = Number::Pair(Box::new(right));
                    SplitResult::AlreadySplit
                }
                r => r,
            },
        }
    }
}

impl std::ops::Add for Pair {
    type Output = Pair;

    fn add(self, right: Self) -> Self::Output {
        Pair {
            left: Number::Pair(Box::new(self)),
            right: Number::Pair(Box::new(right)),
        }
        .reduce()
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Pair(p) => p.fmt(f),
            Number::Literal(x) => x.fmt(f),
        }
    }
}
impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        '['.fmt(f)?;
        self.left.fmt(f)?;
        ','.fmt(f)?;
        self.right.fmt(f)?;
        ']'.fmt(f)
    }
}

fn parse_pair(s: &str) -> (Pair, usize) {
    let (left, consumed_left) = match s.chars().nth(1).unwrap() {
        '[' => {
            let (p, consumed) = parse_pair(&s[1..]);
            (Number::Pair(Box::new(p)), consumed)
        }
        x => (Number::Literal(x.to_digit(10).unwrap() as usize), 1),
    };
    let (right, consumed_right) = match s.chars().nth(consumed_left + 2).unwrap() {
        '[' => {
            let (p, consumed) = parse_pair(&s[(consumed_left + 2)..]);
            (Number::Pair(Box::new(p)), consumed)
        }
        x => (Number::Literal(x.to_digit(10).unwrap() as usize), 1),
    };
    (Pair { left, right }, consumed_left + consumed_right + 3)
}

fn main() {
    let file = File::open("aoc2021/inputs/day18.input").unwrap();
    let pairs = io::BufReader::new(file)
        .lines()
        .map(|l| parse_pair(&l.unwrap()).0)
        .collect::<Vec<_>>();
    let sum = pairs.iter().cloned().reduce(std::ops::Add::add).unwrap();
    println!("part 1: {}", sum.magnitude());
    let mut largest_mag = 0;
    for (i, p1) in pairs.iter().enumerate() {
        for (j, p2) in pairs.iter().enumerate() {
            if i != j {
                largest_mag = std::cmp::max(largest_mag, (p1.clone() + p2.clone()).magnitude());
            }
        }
    }
    println!("part 2: {}", largest_mag);
}
