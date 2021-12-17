use std::{
    cmp,
    fs::{self},
    ops::RangeInclusive,
};

use regex::Regex;

struct Probe {
    xvelocity: i64,
    yvelocity: i64,
    xposition: i64,
    yposition: i64,
}

impl Probe {
    fn new(xvelocity: i64, yvelocity: i64) -> Self {
        Self {
            xvelocity,
            yvelocity,
            xposition: 0,
            yposition: 0,
        }
    }

    fn step(&mut self) {
        self.xposition += self.xvelocity;
        self.yposition += self.yvelocity;
        self.xvelocity = cmp::max(self.xvelocity - 1, 0);
        self.yvelocity -= 1;
    }
}

fn search(xtarget: RangeInclusive<i64>, ytarget: RangeInclusive<i64>) -> (i64, usize) {
    let mut max_y = 0;
    let mut cnt = 0;
    // `x` never goes below 0.
    for xvelocity in 1..=*xtarget.end() {
        // If `yvelocity.abs()` is larger than `ytarget.start().abs()` before hitting the target,
        // then we'll overshoot (`ytarget.start().abs() > ytarget.end().abs()`).
        // For a positive initial velocity, the probe reaches `y = 0` in exactly double the time
        // it takes to reach the highest point and with exactly the opposite value of the initial
        // velocity. So, the initial velocity can't be higher than `ytarget.start().abs()`.
        for yvelocity in *ytarget.start()..=ytarget.start().abs() {
            let mut local_max_y = 0;
            let mut probe = Probe::new(xvelocity, yvelocity);
            while probe.xposition < *xtarget.end() && probe.yposition > *ytarget.end() {
                probe.step();
                local_max_y = cmp::max(local_max_y, probe.yposition);
                if xtarget.contains(&probe.xposition) && ytarget.contains(&probe.yposition) {
                    max_y = cmp::max(local_max_y, max_y);
                    cnt += 1;
                    break;
                }
            }
        }
    }
    (max_y, cnt)
}

fn main() {
    let input = fs::read_to_string("aoc2021/inputs/day17.input").unwrap();
    let values = Regex::new(r"(-?\d+)")
        .unwrap()
        .captures_iter(&input)
        .map(|c| c[1].parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let target_x = values[0]..=values[1];
    let target_y = values[2]..=values[3];
    let (maxy, cnt) = search(target_x, target_y);
    println!("part 1: {}", maxy);
    println!("part 2: {}", cnt);
}
