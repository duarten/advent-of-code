use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Clone, Debug)]
struct Node {
    value: isize,
    next: usize,
    prev: usize,
}

fn mix(numbers: &mut [Node]) {
    for idx in 0..numbers.len() {
        let value = numbers[idx].value;
        if value == 0 {
            continue;
        }
        let steps = if value < 0 { value - 1 } else { value } % (numbers.len() - 1) as isize;
        let target_prev = (0..steps.abs()).fold(idx, |current, _| {
            if steps > 0 {
                numbers[current].next
            } else {
                numbers[current].prev
            }
        });
        numbers[numbers[idx].next].prev = numbers[idx].prev;
        numbers[numbers[idx].prev].next = numbers[idx].next;
        numbers[numbers[target_prev].next].prev = idx;
        numbers[idx].next = numbers[target_prev].next;
        numbers[target_prev].next = idx;
        numbers[idx].prev = target_prev;
    }
}

fn grove_coordinates(mut numbers: Vec<Node>, cycles: usize) -> isize {
    (0..cycles).for_each(|_| mix(&mut numbers));
    let mut cur = numbers.iter().position(|n| n.value == 0).unwrap();
    let mut sum = 0;
    for idx in 0..=3000 {
        if idx % 1000 == 0 {
            sum += numbers[cur].value;
        }
        cur = numbers[cur].next;
    }
    sum
}

fn main() {
    let file = File::open("aoc2022/inputs/day20.input").unwrap();
    let mut numbers = io::BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(idx, l)| Node {
            value: l.unwrap().parse().unwrap(),
            next: idx + 1,
            prev: idx.saturating_sub(1),
        })
        .collect::<Vec<_>>();
    numbers.last_mut().unwrap().next = 0;
    numbers.first_mut().unwrap().prev = numbers.len() - 1;
    println!("part 1: {}", grove_coordinates(numbers.clone(), 1));
    numbers.iter_mut().for_each(|n| n.value *= 811589153);
    println!("part 2: {}", grove_coordinates(numbers, 10));
}
