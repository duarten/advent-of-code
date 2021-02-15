use std::fs;

fn find_hash(k: &str, mask: u8) -> usize {
    (1..)
        .map(|i| (i, md5::compute(k.to_owned() + &i.to_string())))
        .find(|(_, r)| r[0] == 0 && r[1] == 0 && (r[2] & mask) == 0)
        .unwrap()
        .0
}

fn main() {
    let k = fs::read_to_string("aoc2015/inputs/day04.input").unwrap();
    println!("part 1: {:?}", find_hash(&k, 0xF0));
    println!("part 2: {:?}", find_hash(&k, 0xFF));
}
