use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("aoc2023/inputs/day15.input").unwrap();
    let sum = input.split(',').map(hash).sum::<usize>();
    println!("part 1: {sum}");
    let mut boxes = HashMap::<_, Vec<(&str, usize)>>::new();
    for step in input.split(',') {
        let (label, focal_length) = if step.chars().nth(step.len() - 1) == Some('-') {
            (&step[0..step.len() - 1], None)
        } else {
            (
                &step[0..step.len() - 2],
                Some(step[step.len() - 1..].parse::<usize>().unwrap()),
            )
        };
        let lenses = boxes.entry(hash(label)).or_default();
        let idx = lenses.iter().position(|(l, _)| *l == label);
        if let Some(focal_length) = focal_length {
            if let Some(idx) = idx {
                lenses[idx].1 = focal_length;
            } else {
                lenses.push((label, focal_length));
            }
        } else if let Some(idx) = idx {
            lenses.remove(idx);
        }
    }
    let mut total = 0;
    for (id, lenses) in boxes {
        for (idx, (_, focal_length)) in lenses.iter().enumerate() {
            let focus = (1 + id) * (1 + idx) * focal_length;
            total += focus
        }
    }
    println!("part 2: {total}");
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}
