use std::fs;

use utils::transpose;

fn main() {
    let input = fs::read_to_string("aoc2023/inputs/day13.input").unwrap();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for pattern in input.split("\n\n") {
        let p = pattern
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        sum1 += 100 * find_mirror(&p, false) + find_mirror(&transpose(&p), false);
        sum2 += 100 * find_mirror(&p, true) + find_mirror(&transpose(&p), true);
    }
    println!("part 1: {sum1}");
    println!("part 2: {sum2}");
}

fn find_mirror(pattern: &[Vec<char>], maybe_smudged: bool) -> usize {
    (1..pattern.len())
        .find(|line| is_mirrored(pattern, *line, maybe_smudged))
        .unwrap_or_default()
}

fn is_mirrored(pattern: &[Vec<char>], line: usize, maybe_smudged: bool) -> bool {
    let upper = pattern.iter().take(line).rev();
    let lower = pattern.iter().skip(line);
    let mut smudges_fixed = 0;
    lower.zip(upper).all(|(l, u)| {
        let count = l.iter().zip(u.iter()).filter(|(l, u)| l != u).count();
        smudges_fixed += (count == 1) as usize;
        count <= (maybe_smudged as usize)
    }) && smudges_fixed == (maybe_smudged as usize)
}
