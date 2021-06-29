use anyhow::Result;
use std::fs;
use std::io::{self, BufRead};

const LINE_SIZE: i64 = 100;

#[derive(Clone, Copy, PartialEq)]
enum State {
    Off,
    On,
}

fn is_corner(row: i64, col: i64) -> bool {
    (col == 0 || col == (LINE_SIZE - 1)) && (row == 0 || row == (LINE_SIZE - 1))
}

fn is_on(wr: &[Vec<State>], row: i64, col: i64) -> bool {
    (0..LINE_SIZE).contains(&col)
        && (0..LINE_SIZE).contains(&row)
        && wr[row as usize][col as usize] == State::On
}

fn adjacent_on(wr: &[Vec<State>], row: i64, col: i64) -> usize {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .filter(|(r, c)| is_on(wr, row + r, col + c))
    .count()
}

fn simulate(wr: &[Vec<State>], corners_on: bool) -> Vec<Vec<State>> {
    (0..LINE_SIZE)
        .map(|row| {
            (0..LINE_SIZE)
                .map(|col| {
                    let adj = adjacent_on(wr, row, col);
                    match wr[row as usize][col as usize] {
                        _ if corners_on && is_corner(row, col) => State::On,
                        State::Off if adj == 3 => State::On,
                        State::On if adj != 2 && adj != 3 => State::Off,
                        s => s,
                    }
                })
                .collect()
        })
        .collect()
}

fn run_simulation(wr: Vec<Vec<State>>, corners_on: bool) -> usize {
    itertools::iterate(wr, |wr| simulate(wr, corners_on))
        .nth(100)
        .unwrap()
        .into_iter()
        .flatten()
        .filter(|x| *x == State::On)
        .count()
}

fn main() -> Result<()> {
    let file = fs::File::open("aoc2015/inputs/day18.input").unwrap();
    let wr = io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| match c {
                    '.' => State::Off,
                    '#' => State::On,
                    _ => panic!("unable to parse {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    println!("part 1: {}", run_simulation(wr.clone(), false));
    println!("part 2: {}", run_simulation(wr, true));
    Ok(())
}
