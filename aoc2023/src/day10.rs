use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

fn neighbors(pos: (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .into_iter()
        .map(move |d| (pos.0 + d.0, pos.1 + d.1))
}

fn main() {
    let file = File::open("aoc2023/inputs/day10.input").unwrap();
    let mut pipes = HashMap::new();
    let mut starting_position = (0, 0);
    for (y, line) in io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
    {
        for (x, c) in line.chars().enumerate() {
            let pipe = match c {
                '|' => [(0, -1), (0, 1)],
                '-' => [(-1, 0), (1, 0)],
                'L' => [(0, -1), (1, 0)],
                'J' => [(0, -1), (-1, 0)],
                '7' => [(0, 1), (-1, 0)],
                'F' => [(0, 1), (1, 0)],
                'S' => {
                    starting_position = (x as i32, y as i32);
                    continue;
                }
                _ => continue,
            };
            pipes.insert((x as _, y as _), pipe);
        }
    }
    let starting_pipe = neighbors(starting_position)
        .filter_map(|pipe_position| {
            for d in pipes.get(&pipe_position)?.iter() {
                let connected_position = (pipe_position.0 + d.0, pipe_position.1 + d.1);
                if connected_position == starting_position {
                    return Some((-d.0, -d.1));
                }
            }
            None
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    let mut pipe_loop = HashSet::from([starting_position]);
    let mut current_position = starting_position;
    let mut start = (i32::MAX, i32::MAX);
    let mut end = (0, 0);
    'outer: loop {
        let next_pipe = pipes.get(&current_position).unwrap_or(&starting_pipe);
        for d in next_pipe.iter() {
            let next_pos = (current_position.0 + d.0, current_position.1 + d.1);
            if pipe_loop.insert(next_pos) {
                start = (start.0.min(next_pos.0), start.1.min(next_pos.1));
                end = (end.0.max(next_pos.0), end.1.max(next_pos.1));
                current_position = next_pos;
                continue 'outer;
            }
        }
        break;
    }
    println!("part 1: {}", pipe_loop.len() / 2);
    let mut inside_count = 0;
    for y in start.1..=end.1 {
        let mut outside = true;
        for x in start.0..=end.0 {
            if let Some(position) = pipe_loop.get(&(x, y)) {
                let directions = pipes.get(position).unwrap_or(&starting_pipe);
                if directions.contains(&(0, 1)) {
                    outside = !outside;
                }
            } else if !outside {
                inside_count += 1;
            }
        }
    }
    println!("part 2: {}", inside_count);
}
