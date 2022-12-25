use std::{
    collections::VecDeque,
    fs,
    io::{self, BufRead},
};

fn main() {
    let file = fs::File::open("aoc2022/inputs/day25.input").unwrap();
    let mut decimal = io::BufReader::new(file)
        .lines()
        .map(|s| {
            s.unwrap()
                .chars()
                .rev()
                .enumerate()
                .fold(0, |acc, (position, chr)| {
                    let digit = match chr {
                        '=' => -2,
                        '-' => -1,
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        _ => unreachable!(),
                    };

                    acc + 5i64.pow(position as _) * digit
                })
        })
        .sum::<i64>();

    let mut snafu = VecDeque::new();
    while decimal > 0 {
        let (digit, chr) = match decimal % 5 {
            0 => (0, '0'),
            1 => (1, '1'),
            2 => (2, '2'),
            3 => (-2, '='),
            4 => (-1, '-'),
            _ => unreachable!(),
        };
        snafu.push_front(chr);
        decimal -= digit;
        decimal /= 5;
    }

    println!("part 1: {:?}", snafu.into_iter().collect::<String>());
}
