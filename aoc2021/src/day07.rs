fn main() {
    let file = std::fs::read_to_string("aoc2021/inputs/day07.input").unwrap();
    let mut fuel = file
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();
    fuel.sort_unstable();
    let cheapest: usize = (fuel[0]..fuel[fuel.len() - 1])
        .map(|target| fuel.iter().map(|f| f.abs_diff(target)).sum())
        .min()
        .unwrap();
    println!("part 1: {}", cheapest);
    let cheapest: usize = (fuel[0]..fuel[fuel.len() - 1])
        .map(|target| {
            fuel.iter()
                .map(|f| {
                    let diff = f.abs_diff(target);
                    diff * (1 + diff) / 2
                })
                .sum()
        })
        .min()
        .unwrap();
    println!("part 2: {}", cheapest);
}
