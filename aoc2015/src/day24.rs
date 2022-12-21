use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// What's the smallest set of values that add up to capacity?
// Exhaustive solution to the multiple subset sum problem.
fn pack(
    values: &[u128],
    idx: usize,
    sum: u128,
    product: u128,
    capacity: u128,
    min_product: &mut u128,
) {
    if sum == capacity {
        *min_product = std::cmp::min(product, *min_product);
    } else if sum < capacity && product < *min_product && idx < values.len() {
        pack(values, idx + 1, sum, product, capacity, min_product);
        let v = values[idx];
        pack(values, idx + 1, sum + v, product * v, capacity, min_product);
    }
}

fn qe(values: &[u128], target: u128) -> u128 {
    let mut min_product = u128::MAX;
    pack(values, 0, 0, 1, target, &mut min_product);
    min_product
}

fn main() {
    let file = File::open("aoc2015/inputs/day24.input").unwrap();
    let values = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .filter_map(|a| a.parse::<u128>().ok())
        .collect::<Vec<_>>();
    println!("part 1: {}", qe(&values, values.iter().sum::<u128>() / 3));
    println!("part 2: {}", qe(&values, values.iter().sum::<u128>() / 4));
}
