use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn fits(
    containers: &[usize],
    idx: usize,
    litres: usize,
    path: Vec<usize>,
    out: &mut Vec<Vec<usize>>,
) {
    if litres == 150 {
        out.push(path);
    } else if litres < 150 && idx < containers.len() {
        let mut npath = path.clone();
        npath.push(containers[idx]);
        fits(containers, idx + 1, litres, path, out);
        fits(containers, idx + 1, litres + containers[idx], npath, out);
    }
}

fn main() {
    let file = File::open("aoc2015/inputs/day17.input").unwrap();
    let containers = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .filter_map(|a| a.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let mut results = Vec::new();
    fits(&containers, 0, 0, vec![], &mut results);
    println!("part 1: {}", results.len());
    results.sort_unstable_by_key(|v| v.len());
    let min = results[0].len();
    println!(
        "part 2: {}",
        results.iter().take_while(|v| v.len() == min).count()
    );
}
