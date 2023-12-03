use std::{
    fs::File,
    io::{self, BufRead},
};

fn first_digit(
    line: &str,
    substrings: impl Iterator<Item = usize>,
    use_spelled: bool,
) -> Option<char> {
    const SPELLED_DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for idx in substrings {
        for (spelled, digit) in SPELLED_DIGITS.iter().zip(1..=9) {
            if (use_spelled && line[idx..].starts_with(spelled))
                || line[idx..].starts_with(digit.to_string().as_str())
            {
                return char::from_digit(digit, 10);
            }
        }
    }
    None
}

fn sum_spelled_digits(line: &str, use_spelled: bool) -> usize {
    let first = first_digit(line, 0..line.len(), use_spelled).unwrap();
    let last = first_digit(line, (0..line.len()).rev(), use_spelled).unwrap();
    String::from_iter([first, last]).parse::<usize>().unwrap()
}

fn main() {
    let file = File::open("aoc2023/inputs/day01.input").unwrap();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        sum1 += sum_spelled_digits(&line, false);
        sum2 += sum_spelled_digits(&line, true);
    }
    println!("part 1: {:?}", sum1);
    println!("part 2: {:?}", sum2);
}
