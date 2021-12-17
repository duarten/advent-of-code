use std::{
    fs::File,
    io::{self, BufRead},
};

fn iterate(v: &[char], mut f: impl FnMut(&[char], bool)) {
    let mut lower_bound = 0;
    let delimiters = ['[', ']'];
    let mut looking_for = 0;
    while lower_bound < v.len() {
        let upper_bound = lower_bound
            + v[lower_bound..]
                .iter()
                .position(|&c| c == delimiters[looking_for])
                .unwrap_or(v.len() - lower_bound);
        f(&v[lower_bound..upper_bound], delimiters[looking_for] == ']');
        looking_for = (looking_for + 1) % delimiters.len();
        lower_bound = upper_bound + 1;
    }
}

fn abba(v: &[char]) -> bool {
    if v.len() < 4 {
        return false;
    }
    (0..(v.len() - 3))
        .any(|idx| v[idx] != v[idx + 1] && v[idx] == v[idx + 3] && v[idx + 1] == v[idx + 2])
}

fn tls(v: &[char]) -> bool {
    let mut has_abba = false;
    let mut has_abba_hypernet = false;
    iterate(v, |v, hypernet| {
        if abba(v) {
            has_abba = true;
            if hypernet {
                has_abba_hypernet = true;
            }
        }
    });
    has_abba && !has_abba_hypernet
}

fn collect_abas(v: &[char], collector: &mut Vec<(char, char)>) {
    if v.len() < 3 {
        return;
    }
    collector.extend(
        (0..(v.len() - 2))
            .filter(|&idx| v[idx] != v[idx + 1] && v[idx] == v[idx + 2])
            .map(|idx| (v[idx], v[idx + 1])),
    )
}

fn ssl(v: &[char]) -> bool {
    let mut abas = Vec::new();
    let mut babs = Vec::new();
    iterate(v, |v, hypernet| {
        if hypernet {
            collect_abas(v, &mut babs);
        } else {
            collect_abas(v, &mut abas);
        }
    });
    abas.into_iter().any(|(a, b)| babs.contains(&(b, a)))
}

fn main() {
    let file = File::open("aoc2016/inputs/day07.input").unwrap();
    let mut tls_cnt = 0;
    let mut ssl_cnt = 0;
    for l in io::BufReader::new(file).lines().map(Result::unwrap) {
        let chars = l.chars().collect::<Vec<_>>();
        tls_cnt += tls(&chars) as usize;
        ssl_cnt += ssl(&chars) as usize;
    }
    println!("part 1: {}", tls_cnt);
    println!("part 2: {}", ssl_cnt);
}
