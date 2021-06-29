fn part1(n: usize) -> usize {
    (1..=((n as f64).sqrt() + 1.0) as usize)
        .filter(|i| n % i == 0)
        .map(|i| i + (n / i))
        .sum::<usize>()
        * 10
}

fn part2(n: usize) -> usize {
    (1..=((n as f64).sqrt() + 1.0) as usize)
        .filter(|i| n % i == 0)
        .map(|i| if i <= 50 { n / i } else { 0 } + if n / i <= 50 { i } else { 0 })
        .sum::<usize>()
        * 11
}

fn main() {
    let input = 34000000;
    println!(
        "part 1: {}",
        (1..).map(part1).take_while(|&n| n < input).count() + 1
    );
    println!(
        "part 2: {}",
        (1..).map(part2).take_while(|&n| n < input).count() + 1
    );
}
