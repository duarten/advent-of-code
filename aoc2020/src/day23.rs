use std::{fs, num::NonZeroUsize};

use utils::list::{Arena, CircularNodeIterator, NodeId};

struct Cups {
    arena: Arena<usize>,
    current: NodeId,
}

impl Cups {
    fn new(labels: Vec<usize>) -> Self {
        let mut arena = Arena::new();
        (1..=labels.len()).for_each(|x| {
            arena.new_node(x);
        });
        let current = NodeId(NonZeroUsize::new(labels[0]).unwrap());
        labels
            .into_iter()
            .map(|l| NodeId(NonZeroUsize::new(l).unwrap()))
            .reduce(|a, b| a.append_after(b, &mut arena));
        Self { arena, current }
    }

    fn do_move(&mut self) {
        let excluded = self.current.pop_after(3, &mut self.arena).unwrap();
        let target = (1..)
            .map(|t| ((self.current.0.get() - 1) + self.arena.len() - t) % self.arena.len() + 1)
            .skip_while(|t| excluded.contains(t, &self.arena))
            .take(1)
            .last()
            .unwrap();
        NodeId(NonZeroUsize::new(target).unwrap()).append_after(excluded, &mut self.arena);
        self.current = self.arena[self.current].next
    }

    fn result(&self, limit: usize) -> Vec<usize> {
        CircularNodeIterator::new(&self.arena, NodeId(NonZeroUsize::new(1).unwrap()))
            .skip(1)
            .take(limit)
            .map(|n| n.data)
            .collect()
    }
}

fn main() {
    let mut labels: Vec<_> = fs::read_to_string("aoc2020/inputs/day23.input")
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut c = Cups::new(labels.clone());
    (0..100).for_each(|_| c.do_move());
    let r = c
        .result(8)
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("");
    println!("part 1: {}", r);
    labels.extend(10..=1000000);
    let mut c = Cups::new(labels);
    (0..10000000).for_each(|_| c.do_move());
    let r = c.result(2).iter().product::<usize>();
    println!("part 2: {}", r);
}
