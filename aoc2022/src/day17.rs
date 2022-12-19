use std::{collections::HashMap, fs};

struct Rock {
    rows: Vec<Vec<usize>>,
    base_y: usize,
}

fn generate_rock(kind: usize) -> Rock {
    const X: usize = 2;
    let base_y = 3;
    let rows = match kind % 5 {
        0 => vec![vec![X, X + 1, X + 2, X + 3]],
        1 => vec![vec![X + 1], vec![X, X + 1, X + 2], vec![X + 1]],
        2 => vec![vec![X, X + 1, X + 2], vec![X + 2], vec![X + 2]],
        3 => vec![vec![X], vec![X], vec![X], vec![X]],
        4 => vec![vec![X, X + 1], vec![X, X + 1]],
        _ => unreachable!(),
    };
    Rock { rows, base_y }
}

fn history_idx(history: &[Vec<usize>], y_base: usize, y_offset: usize) -> Option<usize> {
    let y = y_base + y_offset;
    (y < history.len()).then_some(history.len() - 1 - (history.len() - y - 1))
}

fn adjust(rock: &mut Rock, history_x: &[Vec<usize>], step: (i32, i32)) -> bool {
    // Check y-axis bounds.
    let new_base_y = if step.1 < 0 {
        if rock.base_y == 0 {
            return false;
        }
        rock.base_y - step.1.unsigned_abs() as usize
    } else {
        rock.base_y + step.1 as usize
    };
    for (y_offset, row) in rock.rows.iter().enumerate() {
        for &x in row {
            // Check x-axis bounds.
            let x = match step.0 {
                step if step > 0 => {
                    if x == 6 {
                        return false;
                    }
                    x + step as usize
                }
                step if step < 0 => {
                    if x == 0 {
                        return false;
                    }
                    x - step.unsigned_abs() as usize
                }
                _ => x,
            };
            // Given the new position, check for collisions.
            if let Some(hist_idx) = history_idx(history_x, new_base_y, y_offset) {
                if history_x[hist_idx].contains(&x) {
                    return false;
                }
            }
        }
    }
    for row in rock.rows.iter_mut() {
        for x in row.iter_mut() {
            *x = (*x as isize + step.0 as isize) as usize;
        }
    }
    rock.base_y = new_base_y;
    true
}

fn ceiling(history_x: &[Vec<usize>]) -> Vec<usize> {
    (0..=6)
        .map(|x_index| {
            history_x
                .iter()
                .rev()
                .take_while(|x| !x.contains(&x_index))
                .count()
        })
        .collect()
}

fn settle_rock<'a>(
    rocks: &mut impl Iterator<Item = (usize, Rock)>,
    jet: &mut impl Iterator<Item = (usize, &'a u8)>,
    history_x: &mut Vec<Vec<usize>>,
) -> (Vec<usize>, usize, usize) {
    let (rock_idx, mut rock) = rocks.next().unwrap();
    rock.base_y += history_x.len();
    let jet_idx = loop {
        let (jet_idx, x_step) = jet.next().unwrap();
        let step = (if *x_step == b'>' { 1 } else { -1 }, 0);
        adjust(&mut rock, history_x, step);
        if !adjust(&mut rock, history_x, (0, -1)) {
            break jet_idx;
        }
    };
    // Merge the rock rows into the history.
    for (offset, r) in rock.rows.into_iter().enumerate() {
        if let Some(idx) = history_idx(history_x, rock.base_y, offset) {
            history_x[idx].extend(r);
        } else {
            history_x.push(r);
        }
    }
    (ceiling(history_x), rock_idx, jet_idx)
}

fn solve(num_rocks: usize, jet_pattern: &[u8]) -> usize {
    let mut rocks_fallen = 0;
    let mut history_x = Vec::with_capacity(100);
    let mut cache = HashMap::new();
    let mut jet = jet_pattern.iter().enumerate().cycle();
    let mut rocks = (0..5).map(generate_rock).enumerate().cycle();
    while rocks_fallen < num_rocks {
        let state = settle_rock(&mut rocks, &mut jet, &mut history_x);
        rocks_fallen += 1;
        if let Some((rocks_fallen_before, height_before)) = cache.get(&state) {
            let blocks_in_cycle = rocks_fallen - rocks_fallen_before;
            let remaining = num_rocks - rocks_fallen;
            let repeats = remaining / blocks_in_cycle;
            let reminder = remaining - (repeats * blocks_in_cycle);
            let mut total_height = history_x.len() + (history_x.len() - height_before) * repeats;
            for _ in 0..reminder {
                let height_now = history_x.len();
                settle_rock(&mut rocks, &mut jet, &mut history_x);
                total_height += history_x.len() - height_now;
            }
            return total_height;
        }
        cache.insert(state, (rocks_fallen, history_x.len()));
    }
    history_x.len()
}

fn main() {
    let file = fs::read_to_string("aoc2022/inputs/day17.input").unwrap();
    println!("part 1: {}", solve(2022, file.as_bytes()));
    println!("part 2: {}", solve(1000000000000, file.as_bytes()));
}
