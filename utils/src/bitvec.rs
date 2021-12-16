use std::borrow::Borrow;

pub trait BitVec {
    fn to_number(self) -> usize;
}

impl<V: Borrow<u8>, I: Iterator<Item = V>> BitVec for I {
    fn to_number(self) -> usize {
        self.fold(0_usize, |acc, b| (acc << 1) | (*b.borrow() as usize))
    }
}

pub fn char_to_bits(value: char) -> [u8; 4] {
    let d = value.to_digit(16).unwrap() as u8;
    let mut res = [0; 4];
    for i in 0..4 {
        res[4 - 1 - i] = (d >> i) & 1;
    }
    res
}
