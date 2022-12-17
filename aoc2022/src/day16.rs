use itertools::{Either, Itertools};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, character::complete::u32,
    combinator::map, multi::separated_list0, sequence::tuple, IResult,
};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    iter::once,
};

fn valve(input: &str) -> IResult<&str, (String, (usize, Vec<String>))> {
    let t = tuple((
        tag("Valve "),
        alpha1,
        tag(" has flow rate="),
        u32,
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list0(tag(", "), map(alpha1, str::to_owned)),
    ));
    map(t, |(_, valve, _, rate, _, tunnels)| {
        (valve.to_owned(), (rate as usize, tunnels))
    })(input)
}

struct Path<'a> {
    next: Vec<&'a str>,
    opened: HashSet<&'a str>,
    minutes: usize,
    pressure: usize,
}

impl<'a> Path<'a> {
    fn new(next: Vec<&'a str>, opened: HashSet<&'a str>, minutes: usize, pressure: usize) -> Self {
        Self {
            next,
            opened,
            minutes,
            pressure,
        }
    }
}

fn calc_pressure(opened: &HashSet<&str>, valves: &HashMap<String, (usize, Vec<String>)>) -> usize {
    opened.iter().map(|v| valves[*v].0).sum()
}

fn max_pressure(
    valves: &HashMap<String, (usize, Vec<String>)>,
    start: Vec<&str>,
    total_time: usize,
) -> usize {
    let mut visited = HashMap::new();
    let openable = valves.values().filter(|(rate, _)| *rate > 0).count();
    let mut to_visit: Vec<_> = vec![Path::new(start, HashSet::<&str>::new(), 0, 0)];
    let mut max = 0;
    while let Some(Path {
        next,
        minutes,
        mut pressure,
        opened,
    }) = to_visit.pop()
    {
        if let Some(&max_pressure) = visited.get(&(next.clone(), minutes)) {
            if max_pressure >= pressure {
                continue;
            }
        }
        visited.insert((next.clone(), minutes), pressure);
        if minutes == total_time {
            if pressure > max {
                max = pressure;
            }
            continue;
        }
        pressure += calc_pressure(&opened, valves);
        if opened.len() >= openable {
            to_visit.push(Path::new(next, opened, minutes + 1, pressure));
            continue;
        }
        let candidates = next
            .into_iter()
            .map(|next| {
                let (new_pressure, tunnels) = &valves[next];
                let mut opened = opened.clone();
                if *new_pressure > 0 && opened.insert(next) {
                    Either::Left(once((next, opened)))
                } else {
                    Either::Right(tunnels.iter().map(move |c| (c.as_str(), opened.clone())))
                }
            })
            .multi_cartesian_product()
            .map(|paths| {
                let next = paths.iter().map(|(n, _)| *n).collect::<Vec<_>>();
                let opened = paths.into_iter().fold(HashSet::new(), |mut acc, p| {
                    acc.extend(&p.1);
                    acc
                });
                Path::new(next, opened, minutes + 1, pressure)
            });
        to_visit.extend(candidates);
    }
    max
}

fn main() {
    let file = File::open("aoc2022/inputs/day16.input").unwrap();
    let valves = io::BufReader::new(file)
        .lines()
        .map(|l| valve(&l.unwrap()).unwrap().1)
        .collect::<HashMap<_, _>>();
    println!("part 1: {:?}", max_pressure(&valves, vec!["AA"], 30));
    println!("part 2: {:?}", max_pressure(&valves, vec!["AA", "AA"], 26));
}
