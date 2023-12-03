use std::{
    collections::{HashMap, VecDeque},
    convert::Infallible,
    fs::File,
    hash::Hash,
    io::{self, BufRead},
    str::FromStr,
};

use regex::Regex;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Material {
    fn all() -> impl Iterator<Item = Self> {
        [Self::Ore, Self::Clay, Self::Obsidian, Self::Geode].into_iter()
    }
}

#[derive(Debug)]
struct Cost(HashMap<Material, usize>);

#[derive(Debug)]
struct Blueprint(HashMap<Material, Cost>);

impl FromStr for Blueprint {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r#"Blueprint \d+: Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian."#).unwrap();
        Ok(Blueprint(
            re.captures_iter(s)
                .flat_map(|c| {
                    use Material::*;
                    Material::all().zip(
                        [
                            vec![(Ore, c.get(1).unwrap().as_str().parse().unwrap())],
                            vec![(Ore, c.get(2).unwrap().as_str().parse().unwrap())],
                            vec![
                                (Ore, c.get(3).unwrap().as_str().parse().unwrap()),
                                (Clay, c.get(4).unwrap().as_str().parse().unwrap()),
                            ],
                            vec![
                                (Ore, c.get(5).unwrap().as_str().parse().unwrap()),
                                (Obsidian, c.get(6).unwrap().as_str().parse().unwrap()),
                            ],
                        ]
                        .map(|ms| Cost(ms.into_iter().collect())),
                    )
                })
                .collect(),
        ))
    }
}

impl Blueprint {
    fn can_build(&self, pack: &Pack, robot_type: Material) -> bool {
        self.0[&robot_type]
            .0
            .iter()
            .all(|(m, c)| pack.materials[m] >= *c)
    }

    fn should_build(&self, pack: &Pack, robot_type: Material, built: bool) -> bool {
        let max_cost = self
            .0.values().map(|c| *c.0.get(&robot_type).unwrap_or(&0))
            .max()
            .unwrap_or(0);
        // Are we producing enough of the material to build more robots?
        let still_needed = pack.robots[&robot_type] < max_cost;
        // If this is the branch where we don't build, skip if there's
        // a sibling branch where we did build the robot.
        still_needed && (built || !self.can_build(&pack.clone().unmine(), robot_type))
    }
}

#[derive(Clone, Debug)]
struct Pack {
    materials: HashMap<Material, usize>,
    robots: HashMap<Material, usize>,
}

impl Default for Pack {
    fn default() -> Self {
        let mut robots = Material::all().map(|m| (m, 0)).collect::<HashMap<_, _>>();
        robots.insert(Material::Ore, 1);
        Self {
            materials: Material::all().map(|m| (m, 0)).collect(),
            robots,
        }
    }
}

impl Pack {
    fn mine(mut self) -> Self {
        for m in Material::all() {
            self.materials
                .entry(m)
                .and_modify(|v| *v += self.robots[&m]);
        }
        self
    }

    fn unmine(mut self) -> Self {
        for m in Material::all() {
            self.materials
                .entry(m)
                .and_modify(|v| *v -= self.robots[&m]);
        }
        self
    }

    fn build(mut self, blueprint: &Blueprint, robot_type: Material) -> Pack {
        for (m, c) in &blueprint.0[&robot_type].0 {
            self.materials.entry(*m).and_modify(|v| *v -= *c);
        }
        let mut pack = self.mine();
        pack.robots.entry(robot_type).and_modify(|v| *v += 1);
        pack
    }
}

fn solve(blueprint: &Blueprint, total_time: usize) -> usize {
    let mut to_visit = VecDeque::new();
    to_visit.push_back((Pack::default(), 0, false));
    let mut cache = (0..=total_time).map(|i| (i, 0)).collect::<HashMap<_, _>>();
    while let Some((pack, minutes, built)) = to_visit.pop_front() {
        let &prior_best = cache.get(&minutes).unwrap();
        let geodes = pack.materials[&Material::Geode];
        if geodes < prior_best {
            continue;
        }
        cache.insert(minutes, prior_best.max(geodes));
        if minutes == total_time {
            continue;
        }
        if blueprint.can_build(&pack, Material::Geode) {
            to_visit.push_back((pack.build(blueprint, Material::Geode), minutes + 1, true));
            continue;
        }
        for robot in [Material::Obsidian, Material::Clay, Material::Ore] {
            if blueprint.can_build(&pack, robot) && blueprint.should_build(&pack, robot, built) {
                to_visit.push_back((pack.clone().build(blueprint, robot), minutes + 1, true));
            }
        }
        to_visit.push_back((pack.mine(), minutes + 1, false));
    }
    *cache.get(&total_time).unwrap()
}

fn main() {
    let file = File::open("aoc2022/inputs/day19.input").unwrap();
    let blueprints = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().parse::<Blueprint>().unwrap())
        .collect::<Vec<_>>();
    println!(
        "part 1: {}",
        blueprints
            .iter()
            .enumerate()
            .map(|(i, blueprint)| solve(blueprint, 24) * (i + 1))
            .sum::<usize>()
    );
    println!(
        "part 2: {}",
        blueprints
            .iter()
            .take(3)
            .map(|blueprint| solve(blueprint, 32))
            .product::<usize>()
    );
}
