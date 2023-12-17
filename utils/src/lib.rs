use std::mem::MaybeUninit;

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

pub fn manhattan_distance_i32(p1: (i32, i32), p2: (i32, i32)) -> usize {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as usize
}

pub fn transpose<T: Clone>(src: &[Vec<T>]) -> Vec<Vec<T>> {
    (0..src[0].len())
        .map(|i| src.iter().map(|inner| inner[i].clone()).collect())
        .collect()
}

pub fn rotate<T: Copy>(src: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut rotated = vec![vec![MaybeUninit::uninit(); src.len()]; src[0].len()];
    for r in 0..src.len() {
        for (c, item) in rotated.iter_mut().enumerate().take(src[0].len()) {
            item[src.len() - 1 - r].write(src[r][c]);
        }
    }
    // SAFETY: `MaybeUnit<T>` is `#[repr(transparent)]` over `T``.
    unsafe { std::mem::transmute(rotated) }
}
