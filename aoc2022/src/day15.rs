use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

use regex::Regex;

type Offset = (isize, isize);

struct Sensor {
    pos: Offset,
    beacon: Offset,
}

impl Sensor {
    fn dist(&self) -> isize {
        (self.pos.0.abs_diff(self.beacon.0) + self.pos.1.abs_diff(self.beacon.1)) as isize
    }
}

fn part1(sensors: &[Sensor]) -> usize {
    let y = 2000000;
    let mut ranges = sensors
        .iter()
        .flat_map(|s| {
            let y_dist = s.dist() - s.pos.1.abs_diff(y) as isize;
            (y_dist >= 0).then_some((s.pos.0 - y_dist)..=(s.pos.0 + y_dist))
        })
        .collect::<Vec<_>>();
    ranges.sort_unstable_by_key(|r| *r.start());
    let r = ranges
        .into_iter()
        .reduce(|a, b| std::cmp::min(*a.start(), *b.start())..=std::cmp::max(*a.end(), *b.end()))
        .unwrap();
    let beacons = sensors
        .iter()
        .flat_map(|s| (s.beacon.1 == y).then_some(s.beacon.0))
        .collect::<HashSet<_>>();
    (r.end() - r.start() + 1) as usize - beacons.len()
}

fn part2(sensors: &[Sensor]) -> usize {
    let min = 0;
    let max = 4000000;
    // Rotate coordinate system by 45° and initially consider a rectangle bounding the search area.
    let mut search_space = vec![(-max..=min, max..=(2 * max))];
    let mut new_search_space = Vec::new();
    for sensor in sensors {
        let center = (sensor.pos.0 - sensor.pos.1)..=(sensor.pos.0 + sensor.pos.1);
        let start = (center.start() - sensor.dist())..=(center.end() - sensor.dist());
        let end = (center.start() + sensor.dist())..=(center.end() + sensor.dist());

        for (p_start, p_end) in search_space {
            if !(start.start() <= p_end.start()
                && p_start.start() <= end.start()
                && start.end() <= p_end.end()
                && p_start.end() <= end.end())
            {
                new_search_space.push((p_start, p_end));
            } else {
                if start.start() > p_start.start() {
                    new_search_space.push((p_start.clone(), start.start() - 1..=*p_end.end()));
                }
                if p_end.start() > end.start() {
                    new_search_space.push((((end.start() + 1)..=*p_start.end()), p_end.clone()));
                }
                if start.end() > p_start.end() {
                    new_search_space.push((
                        std::cmp::max(*start.start(), *p_start.start())..=*p_start.end(),
                        std::cmp::min(*end.start(), *p_end.start())..=(start.end() - 1),
                    ));
                }
                if p_end.end() > end.end() {
                    new_search_space.push((
                        std::cmp::max(*start.start(), *p_start.start())..=(end.end() + 1),
                        std::cmp::min(*end.start(), *p_end.start())..=*p_end.end(),
                    ));
                }
            }
        }
        search_space = std::mem::take(&mut new_search_space);
    }

    // Look for an 1x1 rectangle.
    search_space
        .into_iter()
        .filter_map(|(start, end)| {
            if start == end && (start.start() + start.end()) % 2 == 0 {
                // Transform back into original coordinate system.
                let pos = ((start.end() + start.start()) / 2)..=((start.end() - start.start()) / 2);
                if *pos.start() >= 0 && *pos.end() <= max {
                    return Some(pos.start() * max + pos.end());
                }
            }
            None
        })
        .last()
        .unwrap() as usize
}

fn main() {
    let file = File::open("aoc2022/inputs/day15.input").unwrap();
    let re = Regex::new(r#"x=(-?\d+), y=(-?\d+)"#).unwrap();
    let mut sensors = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let mut capts = re.captures_iter(&line).map(|c| {
            (
                c.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                c.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            )
        });
        let pos = capts.next().unwrap();
        let beacon = capts.next().unwrap();
        sensors.push(Sensor { pos, beacon });
    }
    println!("part 1: {}", part1(&sensors));
    println!("part 2: {}", part2(&sensors));
}
