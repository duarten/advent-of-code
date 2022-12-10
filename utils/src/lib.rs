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
