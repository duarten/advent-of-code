use std::fs;

fn transform(mut n: usize, sub: usize) -> usize {
    n *= sub;
    n %= 20201227;
    n
}

fn find_loop_size(k: usize, sub: usize) -> usize {
    let mut n = 1;
    (1..)
        .find(|_| {
            n = transform(n, sub);
            n == k
        })
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("inputs/day25.input").unwrap();
    let mut codes = input.lines();
    let card_code: usize = codes.next().unwrap().parse().unwrap();
    let door_code: usize = codes.next().unwrap().parse().unwrap();
    let card_loop_size = find_loop_size(card_code, 7);
    let k = (0..card_loop_size).fold(1, |n, _| transform(n, door_code));
    println!("part 1: {}", k);
}
