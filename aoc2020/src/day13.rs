struct Bus {
    pos: usize,
    id: usize,
}

fn main() {
    let file = std::fs::read_to_string("aoc2020/inputs/day13.input").unwrap();
    let raw: Vec<_> = file.split('\n').collect();
    let ts: usize = raw[0].parse().unwrap();
    let buses: Vec<_> = raw[1]
        .split(',')
        .enumerate()
        .filter(|(_, c)| *c != "x")
        .map(|(i, c)| Bus {
            pos: i,
            id: c.parse::<usize>().unwrap(),
        })
        .collect();
    let mut buses_by_wait: Vec<_> = buses
        .iter()
        .map(|b| (ts - (ts % b.id) + b.id, b.id))
        .collect();
    buses_by_wait.sort_unstable();
    println!("part 1: {}", (buses_by_wait[0].0 - ts) * buses_by_wait[0].1);

    let mut ts = 0;
    let mut step = 1;
    for b in buses {
        loop {
            if (ts + b.pos) % b.id == 0 {
                break;
            }
            ts += step
        }
        step *= b.id;
    }
    println!("part 2: {}", ts);
}
