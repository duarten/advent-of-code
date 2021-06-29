fn nth(mut row: usize, mut col: usize) -> usize {
    let mut result = 0;
    while row > 1 || col > 1 {
        result += 1;
        if col > 1 {
            col -= 1;
            row += 1;
        } else {
            col = row - 1;
            row = 1;
        }
    }
    result
}

fn main() {
    let mut v: usize = 20151125;
    for _ in 0..nth(3010, 3019) {
        v *= 252533;
        v %= 33554393;
    }
    println!("part 1: {}", v);
}
