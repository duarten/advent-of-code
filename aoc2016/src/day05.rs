use std::fs;

fn main() {
    let id = fs::read_to_string("aoc2016/inputs/day05.input").unwrap();
    let mut p1 = Vec::with_capacity(8);
    let mut p2 = Vec::with_capacity(8);
    p2.resize(8, String::new());
    for h in (0..)
        .map(|i| md5::compute(id.to_owned() + &i.to_string()))
        .filter(|h| h[0] == 0 && h[1] == 0 && (h[2] & 0xF0) == 0)
    {
        let f = h[2] & 0x0F;
        if p1.len() < 8 {
            p1.push(format!("{:x?}", f));
        }
        if f < 8 && p2[f as usize].is_empty() {
            p2[f as usize] = format!("{:x?}", h[3] >> 4);
            if p2.iter().all(|d| !d.is_empty()) {
                break;
            }
        }
    }
    println!("part 1: {}", p1.into_iter().collect::<String>());
    println!("part 2: {}", p2.into_iter().collect::<String>());
}
