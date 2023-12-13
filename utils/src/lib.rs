pub mod bitvec;
pub mod list;

pub fn reverse(mut n: usize) -> usize {
    let mut reversed = 0;
    while n != 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }
    reversed
}

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

pub fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

pub fn transpose<T: Clone>(src: &[Vec<T>]) -> Vec<Vec<T>> {
    (0..src[0].len())
        .map(|i| src.iter().map(|inner| inner[i].clone()).collect())
        .collect()
}
