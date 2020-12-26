use anyhow::{anyhow, Result};
use std::fs;
use std::io::{self, BufRead};

const ROW_SIZE: usize = 98;

#[derive(Clone, Copy, PartialEq)]
enum Position {
    Floor,
    Free,
    Seated,
}

fn within_bounds(wr: &[Position], pos: usize, step: i64, dir: i64) -> bool {
    let new_pos = pos.wrapping_add(step as usize);
    new_pos < wr.len()
        && (dir == 0
            || (dir > 0 && (pos % ROW_SIZE < new_pos % ROW_SIZE))
            || (dir < 0 && (pos % ROW_SIZE > new_pos % ROW_SIZE)))
}

fn find_seated(wr: &[Position], pos: usize, step: i64, limit: usize, dir: i64) -> bool {
    (1..)
        .take(limit)
        .map(|i| i * step)
        .take_while(|s| within_bounds(wr, pos, *s, dir))
        .map(|s| wr[pos.wrapping_add(s as usize)])
        .find(|p| *p != Position::Floor)
        == Some(Position::Seated)
}

fn adjacent_occupied(wr: &[Position], pos: usize, limit: usize) -> usize {
    let rs = ROW_SIZE as i64;
    [
        (1, 1),
        (-1, -1),
        (rs, 0),
        (rs + 1, 1),
        (rs - 1, -1),
        (-rs, 0),
        (-(rs + 1), -1),
        (-(rs - 1), 1),
    ]
    .iter()
    .filter(|(s, d)| find_seated(wr, pos, *s, limit, *d))
    .count()
}

fn simulate(wr: &[Position], threshold: usize, limit: usize) -> Vec<(usize, Position)> {
    wr.iter()
        .enumerate()
        .filter_map(|(i, p)| {
            let adj = adjacent_occupied(wr, i, limit);
            match p {
                Position::Free if adj == 0 => Some((i, Position::Seated)),
                Position::Seated if adj >= threshold => Some((i, Position::Free)),
                _ => None,
            }
        })
        .collect()
}

fn apply_changes(wr: &mut [Position], changes: Vec<(usize, Position)>) {
    changes.into_iter().for_each(|(i, p)| wr[i] = p)
}

fn run_simulation(mut wr: Vec<Position>, threshold: usize, limit: usize) -> usize {
    loop {
        let changes = simulate(&wr, threshold, limit);
        if changes.is_empty() {
            break;
        }
        apply_changes(&mut wr, changes);
    }
    wr.into_iter().filter(|p| *p == Position::Seated).count()
}

fn main() -> Result<()> {
    let file = fs::File::open("aoc2020/inputs/day11.input").unwrap();
    let wr = io::BufReader::new(file)
        .lines()
        .flat_map(|l| {
            l.unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Ok(Position::Floor),
                    'L' => Ok(Position::Free),
                    '#' => Ok(Position::Seated),
                    _ => Err(anyhow!("unable to parse {}", c)),
                })
                .collect::<Vec<Result<Position>>>()
        })
        .collect::<Result<Vec<_>>>()?;
    println!("part 1: {}", run_simulation(wr.clone(), 4, 1));
    println!("part 2: {}", run_simulation(wr, 5, usize::MAX));
    Ok(())
}
