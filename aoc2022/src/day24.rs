use std::{
    collections::{HashSet, VecDeque},
    fs,
    io::{self, BufRead},
};

type Offset = (isize, isize);

fn adjacent((x, y): Offset) -> impl Iterator<Item = Offset> {
    [(1, 0), (0, 1), (-1, 0), (0, -1), (0, 0)]
        .map(|(step_x, step_y)| (x + step_x, y + step_y))
        .into_iter()
}

fn main() {
    let file = fs::File::open("aoc2022/inputs/day24.input").unwrap();
    let mut walls = HashSet::new();
    let mut blizzards = Vec::new();
    for (y, l) in io::BufReader::new(file).lines().enumerate() {
        for (x, c) in l.unwrap().chars().enumerate() {
            match c {
                '#' => {
                    walls.insert((x as isize - 1, y as isize - 1));
                }
                b => {
                    let delta = match b {
                        '>' => (1, 0),
                        '<' => (-1, 0),
                        'v' => (0, 1),
                        '^' => (0, -1),
                        _ => continue,
                    };
                    blizzards.push(((x as isize - 1, y as isize - 1), delta));
                }
            }
        }
    }
    let x_bounds = (-1, *walls.iter().map(|(x, _)| x).max().unwrap());
    let y_bounds = (-1, *walls.iter().map(|(_, y)| y).max().unwrap());
    let start = (x_bounds.0 + 1, y_bounds.0);
    let end = (x_bounds.1 - 1, y_bounds.1);
    walls.extend([(start.0, start.1 - 1), (end.0, end.1 + 1)]);

    let mut goals = [end, start, end].into_iter().collect::<VecDeque<_>>();
    let mut goal_steps = Vec::with_capacity(goals.len());
    let mut steps = 0;
    let mut to_visit = [start].into_iter().collect::<HashSet<_>>();
    while let Some(goal) = goals.front() {
        steps += 1;
        let new_blizzards = blizzards
            .iter()
            .map(|((x, y), (x_step, y_step))| {
                (
                    (x + x_step * steps).rem_euclid(x_bounds.1),
                    (y + y_step * steps).rem_euclid(y_bounds.1),
                )
            })
            .collect::<HashSet<_>>();
        to_visit = to_visit
            .into_iter()
            .flat_map(adjacent)
            .filter(|pos| !new_blizzards.contains(pos) && !walls.contains(pos))
            .collect();
        if to_visit.contains(goal) {
            goal_steps.push(steps);
            to_visit = goals.pop_front().into_iter().collect();
        }
    }
    println!("part 1: {}", goal_steps.first().unwrap());
    println!("part 2: {}", goal_steps.last().unwrap());
}
