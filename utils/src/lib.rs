use std::ops;

pub mod bitvec;
pub mod list;

pub fn abs_diff<T: ops::Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

pub fn reverse(mut n: usize) -> usize {
    let mut reversed = 0;
    while n != 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }
    reversed
}
