use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn neighbors(
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let mut res = Vec::new();
    for x_delta in [-1, 0, 1] {
        for y_delta in [-1, 0, 1] {
            let new_x = x as isize + x_delta;
            let new_y = y as isize + y_delta;
            if x_delta == 0 && y_delta == 0
                || new_x as usize >= max_x
                || new_y as usize >= max_y
                || new_x < 0
                || new_y < 0
            {
                continue;
            }
            res.push((new_x as usize, new_y as usize));
        }
    }
    res.into_iter()
}

fn part_detected(
    number: String,
    mut potential_gears: Vec<(usize, usize)>,
    gears: &mut HashMap<(usize, usize), Vec<usize>>,
) -> usize {
    let Ok(parsed) = number.parse::<usize>() else {
        return 0;
    };
    potential_gears.dedup();
    for g in potential_gears {
        gears.entry(g).or_default().push(parsed);
    }
    parsed
}

fn main() {
    let file = File::open("aoc2023/inputs/day03.input").unwrap();
    let mut sum1 = 0;
    let board = io::BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut gears = HashMap::new();
    for (y, row) in board.iter().enumerate() {
        let mut number = String::new();
        let mut is_part = false;
        let mut potential_gears = Vec::new();
        for (x, col) in row.iter().enumerate() {
            if col.is_ascii_digit() {
                number.push(*col);
                for n in neighbors(x, y, row.len(), board.len()) {
                    let c = board[n.1][n.0];
                    if !c.is_ascii_digit() && c != '.' {
                        is_part = true;
                        if c == '*' {
                            potential_gears.push(n);
                        }
                    }
                }
            } else {
                let number = std::mem::take(&mut number);
                if std::mem::take(&mut is_part) {
                    sum1 += part_detected(number, std::mem::take(&mut potential_gears), &mut gears);
                }
            }
        }
        if is_part {
            sum1 += part_detected(number, potential_gears, &mut gears);
        }
    }
    let sum2 = gears
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().product::<usize>())
        .sum::<usize>();
    println!("part 1: {:?}", sum1);
    println!("part 2: {:?}", sum2);
}
