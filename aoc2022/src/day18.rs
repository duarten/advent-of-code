use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

type Cube = (isize, isize, isize);

fn neighbors(&(x, y, z): &Cube) -> impl Iterator<Item = Cube> {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
    .into_iter()
}

fn bounding_box(cubes: &HashSet<Cube>) -> (Cube, Cube) {
    let mut min = (isize::MAX, isize::MAX, isize::MAX);
    let mut max = (isize::MIN, isize::MIN, isize::MIN);
    for &(x, y, z) in cubes {
        min.0 = min.0.min(x - 1);
        min.1 = min.1.min(y - 1);
        min.2 = min.2.min(z - 1);
        max.0 = max.0.max(x + 1);
        max.1 = max.1.max(y + 1);
        max.2 = max.2.max(z + 1);
    }
    (min, max)
}

fn in_bounds((min, max): &(Cube, Cube), c: &Cube) -> bool {
    c.0 >= min.0 && c.0 <= max.0 && c.1 >= min.1 && c.1 <= max.1 && c.2 >= min.2 && c.2 <= max.2
}

fn flood_fill(cubes: &HashSet<Cube>) -> HashSet<Cube> {
    let bounds = bounding_box(cubes);
    let mut exposed = HashSet::new();
    let mut visited = HashSet::new();
    let mut to_visit = vec![bounds.0];
    while let Some(cube) = to_visit.pop() {
        for c in neighbors(&cube)
            .filter(|n| !cubes.contains(n) && in_bounds(&bounds, n) && visited.insert(*n))
        {
            to_visit.push(c);
            exposed.insert(c);
        }
    }
    exposed
}

fn main() {
    let file = File::open("aoc2022/inputs/day18.input").unwrap();
    let cubes = io::BufReader::new(file)
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let mut split = l.split(',').map(|s| s.parse().unwrap());
            (
                split.next().unwrap(),
                split.next().unwrap(),
                split.next().unwrap(),
            )
        })
        .collect::<HashSet<_>>();
    let neighbors = cubes.iter().flat_map(neighbors).collect::<Vec<_>>();
    let exposed = flood_fill(&cubes);
    println!(
        "part 1: {}",
        neighbors.iter().filter(|c| !cubes.contains(c)).count()
    );
    println!(
        "part 2: {}",
        neighbors.iter().filter(|c| exposed.contains(c)).count()
    );
}
