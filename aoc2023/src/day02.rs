use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("aoc2023/inputs/day02.input").unwrap();
    let mut possible_sum = 0;
    let mut power_sum = 0;
    for line in io::BufReader::new(file).lines().map(Result::unwrap) {
        let mut possible = true;
        let mut min_colors = [usize::MIN; 3];
        let (game, sets) = line.split_once(": ").unwrap();
        for set in sets.split("; ") {
            for cubes in set.split(", ") {
                let (count, color) = cubes.split_once(' ').unwrap();
                let count = count.parse::<usize>().unwrap();
                for (idx, (expected, limit)) in [("red", 12), ("green", 13), ("blue", 14)]
                    .into_iter()
                    .enumerate()
                {
                    if color == expected {
                        if count > limit {
                            possible = false;
                        }
                        min_colors[idx] = min_colors[idx].max(count);
                        break;
                    }
                }
            }
        }
        if possible {
            let (_, id) = game.split_once(' ').unwrap();
            possible_sum += id.parse::<usize>().unwrap();
        }
        power_sum += min_colors.iter().product::<usize>();
    }
    println!("part 1: {:?}", possible_sum);
    println!("part 2: {:?}", power_sum);
}
